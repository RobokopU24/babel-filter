mod args;
mod file;
mod types;

use std::{ffi::OsStr, fs, path::Path, process::ExitCode, time::Instant};

use ahash::AHashSet;
use clap::Parser;
use serde_json;

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

    let mut filter_set: AHashSet<String> = AHashSet::new();
    {
        let mut num_removed: usize = 0;
        let t0 = Instant::now();
        let lines = Reader::new(filter_file, args.read_buf_capacity)
            .expect("Error opening filter file")
            .lines();
        for line in lines {
            if let Ok(node_json) = line {
                let node: Result<types::FilterFormat, serde_json::Error> =
                    serde_json::from_str(&node_json);
                if let Ok(node) = node {
                    if let Some(ref exclude_cats) = args.exclude_category {
                        if !has_excluded_category(&node.category, &exclude_cats) {
                            filter_set.insert(node.id);
                        } else {
                            num_removed += 1;
                        }
                    } else {
                        filter_set.insert(node.id);
                    }
                }
            }
        }
        println!("Creating filter set took {:?}", t0.elapsed());
        println!("{} nodes excluded", num_removed);
    }

    for babel_file in fs::read_dir(babel_directory).unwrap() {
        if let Ok(f) = babel_file {
            if f.path().is_file() {
                let t0 = Instant::now();

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
                    if let Ok(node_json) = line {
                        let node: Result<types::BabelFormat, serde_json::Error> =
                            serde_json::from_str(&node_json);
                        if let Ok(node) = node {
                            if filter_set.contains(&node.curie) {
                                writer.write_line(&node_json).expect("Error writing line")
                            }
                        }
                    }
                }

                println!(
                    "Writing {:?} took {:?}",
                    output_file_path.file_name().unwrap_or_default(),
                    t0.elapsed()
                );
            }
        }
    }

    let duration = start.elapsed();
    println!("Program took {:?}", duration);

    ExitCode::SUCCESS
}

fn has_excluded_category(set: &Vec<String>, exclude_set: &Vec<String>) -> bool {
    if exclude_set.is_empty() {
        return false;
    }
    for cat in set.iter() {
        for ex_cat in exclude_set.iter() {
            if cat == ex_cat {
                return true;
            }
        }
    }
    false
}
