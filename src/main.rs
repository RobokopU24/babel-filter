mod args;
mod file;

use std::{ffi::OsStr, fs, path::Path, process::ExitCode, time::Instant};

use ahash::AHashMap;
use clap::Parser;
use serde_json::{self, Value};

use file::{reader::Reader, writer::Writer};

fn main() -> ExitCode {
    let start = Instant::now();

    let args = args::Cli::parse();

    let babel_directory = args.babel_directory;
    let filter_file = args.filter_file;
    let output_directory = args.output_directory;

    if !babel_directory.is_dir() {
        eprintln!("The path provided to the Babel directory isn't a directory or doesn't exist");
        return ExitCode::FAILURE;
    }
    if !filter_file.is_file() {
        eprintln!("The path provided to the filter file isn't a file or doesn't exist");
        return ExitCode::FAILURE;
    }
    if !output_directory.is_dir() {
        eprintln!("The path provided to the output directory isn't a directory or doesn't exist");
        return ExitCode::FAILURE;
    }

    let mut filter_set: AHashMap<String, Value> = AHashMap::new();
    {
        let mut num_removed: usize = 0;
        let t0 = Instant::now();
        let lines = Reader::new(filter_file, args.read_buf_capacity)
            .expect("Error opening filter file")
            .lines();
        for line in lines {
            if let Ok(node_json) = line {
                let node: Result<Value, serde_json::Error> = serde_json::from_str(&node_json);
                if let Ok(node) = node {
                    if let Some(filter_file_identifier) = node
                        .get(&args.filter_file_identifier)
                        .and_then(|v| v.as_str())
                    {
                        if let Some(ref exclude_cats) = args.exclude_category {
                            let categories = node
                                .get(&args.filter_file_category_key)
                                .and_then(|v| v.as_array())
                                .map(|v| v.iter().filter_map(|i| i.as_str()));

                            if let Some(categories) = categories {
                                if !has_excluded_category(categories, &exclude_cats) {
                                    filter_set.insert(String::from(filter_file_identifier), node);
                                } else {
                                    num_removed += 1;
                                }
                            }
                        } else {
                            filter_set.insert(String::from(filter_file_identifier), node);
                        }
                    }
                }
            }
        }
        println!("Creating filter set took {:.2?}", t0.elapsed());
        println!("{} nodes excluded", num_removed);
    }

    for babel_file in fs::read_dir(babel_directory).unwrap() {
        if let Ok(f) = babel_file {
            if f.path().is_file() {
                let t0 = Instant::now();
                let mut num_nodes: usize = 0;
                let mut num_kept: usize = 0;

                let mut output_file_path = Path::join(
                    output_directory.as_std_path(),
                    f.path().file_name().unwrap(), // should be safe to unwrap as we're checking is_file() above
                );

                // force compressed/not compressed output if output_format arg is set
                match args.output_format {
                    Some(args::OutputFormat::Plaintext) => {
                        if output_file_path.extension() == Some(OsStr::new("gz")) {
                            output_file_path = output_file_path.with_extension("")
                        }
                    }
                    Some(args::OutputFormat::Gzipped) => {
                        if output_file_path.extension() != Some(OsStr::new("gz")) {
                            output_file_path = output_file_path.with_extension("gz")
                        }
                    }
                    None => (),
                }

                let reader: Reader = Reader::new(f.path(), args.read_buf_capacity)
                    .expect("Error opening file for reading");
                let mut writer: Writer =
                    Writer::new(output_file_path.clone(), args.write_buf_capacity)
                        .expect("Error creating file");

                for line in reader.lines() {
                    num_nodes += 1;
                    if let Ok(node_json) = line {
                        let node: Result<Value, serde_json::Error> =
                            serde_json::from_str(&node_json);
                        if let Ok(node) = node {
                            if let Some(babel_file_id) =
                                node.get(&args.babel_identifier).and_then(|v| v.as_str())
                            {
                                if filter_set.remove(babel_file_id).is_some() {
                                    num_kept += 1;
                                    writer.write_line(&node_json).expect("Error writing line");
                                }
                            }
                        }
                    }
                }

                println!(
                    "Writing {:?} took {:.2?}, kept {}/{} nodes ({:.2}%)",
                    output_file_path.file_name().unwrap_or_default(),
                    t0.elapsed(),
                    num_kept,
                    num_nodes,
                    num_kept as f64 / num_nodes as f64
                );
            }
        }
    }

    // create a new file (NonBabelNodes.txt.gz) for all the extra nodes in the filter_set
    let non_babel_nodes_path = Path::join(output_directory.as_std_path(), "./NonBabelNodes.txt.gz");
    let mut nbn_writer = Writer::new(non_babel_nodes_path, args.write_buf_capacity).expect("Error creating NonBabelNodes file");
    let filter_set_size = filter_set.len();
    for (curie, node_json) in filter_set {
        let name = node_json.get("name").and_then(|v| v.as_str());
        let categories = node_json
            .get("category")
            .and_then(|v| v.as_array())
            .map(|v| v.iter().filter_map(|i| i.as_str()));
        
        if let (Some(name), Some(categories)) = (name, categories) {
            let babel_identifier_key = &args.babel_identifier;
            let name_length = name.len();
            let types = categories
                .map(|s| s.replace("biolink:", ""))
                .map(|mut s| { s.insert(0, '"'); s.push('"'); s })
                .collect::<Vec<String>>()
                .join(",");

            let json = format!(r#"{{"{babel_identifier_key}":"{curie}","names":["{name}"],"types":[{types}],"preferred_name":["{name}"],"shortest_name_length":{name_length}}}"#);

            nbn_writer.write_line(&json).expect("Error writing line");
        }
    }
    println!("Wrote an extra {filter_set_size} nodes to NonBabelNodes.txt.gz");

    let duration = start.elapsed();
    println!("Program took {:.2?}", duration);

    ExitCode::SUCCESS
}

fn has_excluded_category<'a, I>(set: I, exclude_set: &Vec<String>) -> bool
where
    I: IntoIterator<Item = &'a str>,
{
    if exclude_set.is_empty() {
        return false;
    }
    for cat in set {
        for ex_cat in exclude_set.iter() {
            if cat == ex_cat {
                return true;
            }
        }
    }
    false
}
