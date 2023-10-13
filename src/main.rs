mod args;
use std::{fs, time::Instant};

use clap::Parser;

fn main() {
    let start = Instant::now();

    let args = args::Cli::parse();

    println!("{:?}", fs::canonicalize(args.babel_directory));
    println!("{:?}", fs::canonicalize(args.output_directory));
    println!("{:?}", fs::canonicalize(args.filter_file));

    let duration = start.elapsed();
    println!("Program took {:?}", duration)
}
