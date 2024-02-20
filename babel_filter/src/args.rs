use camino::Utf8PathBuf;

pub struct Args {
  pub babel_directory: Utf8PathBuf,
  pub filter_file: Utf8PathBuf,
  pub output_directory: Utf8PathBuf,
  pub exclude_category: Option<Vec<String>>,
  pub filter_file_category_key: String,
  pub filter_file_identifier: String,
  pub babel_identifier: String,
  pub output_format: Option<OutputFormat>,
  pub read_buf_capacity: usize,
  pub write_buf_capacity: usize,
}

pub enum OutputFormat {
  Gzipped,
  Plaintext,
}
