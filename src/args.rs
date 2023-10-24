use camino::Utf8PathBuf;
use clap::Parser;

/// This script takes a directory of Babel files (JSONL) and creates filtered versions
/// in a new directory containing only the lines where the the json key (default `curie`)
/// value in the Babel file is present in a json key (default `id`) value in the filter file. 
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  /// The directory containing Babel JSONL files
  pub babel_directory: Utf8PathBuf,
  
  /// The path to the filter JSONL file to be used
  pub filter_file: Utf8PathBuf,

  /// The directory to put the filtered JSONL output files
  pub output_directory: Utf8PathBuf,

  /// Exclude nodes with these biolink categories from the output. Multiple categories 
  /// can be specified by using the flag again
  #[arg(short, long, value_name="CATEGORY")]
  pub exclude_category: Option<Vec<String>>,

  /// The identifier key in each line of the Babel JSONL
  #[arg(long, default_value_t = String::from("curie"), value_name="KEY")]
  pub babel_identifier: String,

  /// The identifier key in each line of the filter file JSONL
  #[arg(long, default_value_t = String::from("id"), value_name="KEY")]
  pub filter_file_identifier: String,
}
