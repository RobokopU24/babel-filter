use flate2::read::GzDecoder;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Buffered file reader that supports gzipped files
pub struct Reader {
  reader: Box<dyn BufRead>,
}
 
impl Reader {
  /// Creates a buffered file reader given a `Path`. It checks the file extension for `.gz`
  /// to determine whether to decompress with `flate2` as it reads. Note that this only checks
  /// the file extension so it's up to the user to ensure files have the proper extensions.
  /// 
  /// Returns `Err` if there is an issue opening the file.
  pub fn new<P>(path: P) -> io::Result<Reader>
  where
      P: AsRef<Path>,
  {
      let file = File::open(&path)?;

      let reader: Box<dyn BufRead> = if path.as_ref().extension() == Some(&OsStr::new("gz")) {
          Box::new(BufReader::new(GzDecoder::new(file)))
      } else {
          Box::new(BufReader::new(file))
      };

      Ok(Reader { reader })
  }

  /// Returns an iterator of lines on this reader
  pub fn lines(self) -> std::io::Lines<Box<dyn BufRead>> {
      self.reader.lines()
  }
}