mod args;

use babel_filter::{self, args::Args};
use std::process::ExitCode;
use clap::Parser;

fn main() -> ExitCode {
    let cli_args = args::CliArgs::parse();

    let filter_args = Args {
        babel_directory: cli_args.babel_directory,
        babel_identifier: cli_args.babel_identifier,
        exclude_category: cli_args.exclude_category,
        filter_file: cli_args.filter_file,
        filter_file_category_key: cli_args.filter_file_category_key,
        filter_file_identifier: cli_args.filter_file_identifier,
        output_directory: cli_args.output_directory,
        output_format: match cli_args.output_format {
            Some(args::OutputFormat::Gzipped) => Some(babel_filter::args::OutputFormat::Gzipped),
            Some(args::OutputFormat::Plaintext) => Some(babel_filter::args::OutputFormat::Plaintext),
            None => None,
        },
        read_buf_capacity: cli_args.read_buf_capacity,
        write_buf_capacity: cli_args.write_buf_capacity,
    };

    babel_filter::run(filter_args)
}
