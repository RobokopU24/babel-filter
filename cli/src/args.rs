use camino::Utf8PathBuf;
use clap::Parser;

/// This script takes a directory of Babel files (JSONL) and creates filtered versions
/// in a new directory containing only the lines where the `curie` value is present in
/// the filter file (based on `id` key in filter file lines). 
/// 
/// Gzipped files will be automatically detected
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
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

  /// Force format of all output files. If not set, output files will match their input files.
  #[clap(short='c', long, value_enum)]
  pub output_format: Option<OutputFormat>,

  /// read buffer capacity, in bytes
  #[arg(long, default_value_t = 32_000, value_name="BYTES")]
  pub read_buf_capacity: usize,

  /// write buffer capacity, in bytes
  #[arg(long, default_value_t = 32_000, value_name="BYTES")]
  pub write_buf_capacity: usize,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
  Gzipped,
  Plaintext,
}
