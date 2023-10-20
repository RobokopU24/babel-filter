mod args;
mod types;
use std::{fs::File, time::Instant, process::ExitCode, io::{self, BufRead}, path::Path};

use ahash::AHashSet;
use clap::Parser;
use serde_json;

fn main() -> ExitCode {
    let start = Instant::now();

    let args = args::Cli::parse();

    let babel_directory = args.babel_directory;
    let output_directory = args.output_directory;
    let filter_file = args.filter_file;

    if !babel_directory.is_dir() {
        eprintln!("The path provided to the Babel directory isn't a directory or doesn't exist");
        return ExitCode::FAILURE;
    }
    if !output_directory.is_dir() {
        eprintln!("The path provided to the output directory isn't a directory or doesn't exist");
        return ExitCode::FAILURE;
    }
    if !filter_file.is_file() {
        eprintln!("The path provided to the filter file isn't a file or doesn't exist");
        return ExitCode::FAILURE;
    }

    let mut filter_set: AHashSet<String> = AHashSet::new();

    if let Ok(lines) = read_lines(filter_file) {
        for line in lines {
            if let Ok(node_json) = line {
                let node: Result<types::FilterFormat, serde_json::Error> = serde_json::from_str(&node_json);
                if let Ok(node) = node {
                    filter_set.insert(node.id);
                }
            }
        }
    }

    let duration = start.elapsed();
    println!("Program took {:?}", duration);

    ExitCode::SUCCESS
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}