use camino::Utf8PathBuf;

pub struct Config {
  pub babel_directory: Utf8PathBuf,
  pub filter_file: Utf8PathBuf,
  pub output_directory: Utf8PathBuf,
  pub exclude_category: Option<Vec<String>>,
  pub output_format: Option<OutputFormat>,
}

pub enum OutputFormat {
  Gzipped,
  Plaintext,
}
