mod args;

use babel_filter;
use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = args::CliArgs::parse();

    let filter_args = babel_filter::Config {
        babel_directory: args.babel_directory,
        exclude_category: args.exclude_category,
        filter_file: args.filter_file,
        output_directory: args.output_directory,
        output_format: match args.output_format {
            Some(args::OutputFormat::Gzipped) => Some(babel_filter::OutputFormat::Gzipped),
            Some(args::OutputFormat::Plaintext) => Some(babel_filter::OutputFormat::Plaintext),
            None => None,
        },
    };

    babel_filter::run(filter_args)
}
