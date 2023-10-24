mod args;
mod types;
use std::{
    fs::{self, File},
    io::{self, BufRead, BufWriter, Write},
    path::Path,
    process::ExitCode,
    time::Instant,
};

use ahash::AHashSet;
use clap::Parser;
use serde_json;

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
    let mut num_removed: usize = 0;

    {
        let t0 = Instant::now();
        if let Ok(lines) = read_lines(filter_file) {
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
        }
        println!("Creating filter set took {:?}", t0.elapsed());
        println!("{} nodes excluded", num_removed);
    }

    for babel_file in fs::read_dir(babel_directory).unwrap() {
        if let Ok(f) = babel_file {
            if f.path().is_file() {
                let t0 = Instant::now();

                let output_file = File::create(Path::join(
                    output_directory.as_std_path(),
                    f.path().file_name().unwrap(),
                ))
                .unwrap();
                let mut buf_writer = BufWriter::with_capacity(256_000, output_file);

                if let Ok(f) = read_lines(f.path()) {
                    for line in f {
                        if let Ok(mut node_json) = line {
                            let node: Result<types::BabelFormat, serde_json::Error> =
                                serde_json::from_str(&node_json);
                            if let Ok(node) = node {
                                if filter_set.contains(&node.curie) {
                                    node_json.push('\n');
                                    buf_writer
                                        .write_all(node_json.as_bytes())
                                        .expect("Error writing");
                                }
                            }
                        }
                    }
                }

                println!("Writing {:?} took {:?}", f.file_name(), t0.elapsed());
            }
        }
    }

    let duration = start.elapsed();
    println!("Program took {:?}", duration);

    ExitCode::SUCCESS
}

fn has_excluded_category(set: &Vec<String>, exclude_set: &Vec<String>) -> bool {
    for cat in set.iter() {
        for ex_cat in exclude_set.iter() {
            if cat == ex_cat {
                return true;
            }
        }
    }
    false
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::with_capacity(256_000, file).lines())
}
