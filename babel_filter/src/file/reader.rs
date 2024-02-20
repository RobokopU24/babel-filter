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
  pub fn new<P>(path: P, buffer_capacity: usize) -> io::Result<Reader>
  where
      P: AsRef<Path>,
  {
      let file = File::open(&path)?;

      let reader: Box<dyn BufRead> = if path.as_ref().extension() == Some(&OsStr::new("gz")) {
          Box::new(BufReader::with_capacity(buffer_capacity, GzDecoder::new(file)))
      } else {
          Box::new(BufReader::with_capacity(buffer_capacity, file))
      };

      Ok(Reader { reader })
  }

  /// Returns an iterator of lines on this reader
  pub fn lines(self) -> std::io::Lines<Box<dyn BufRead>> {
      self.reader.lines()
  }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::{self, Write}};

    use tempfile::tempdir;

    use super::Reader;

    #[test]
    fn read_lines_from_plaintext() -> io::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("test.txt");
        let mut file: File = File::create(&path)?;
        file.write_all("read\nmy\nlines\n".as_bytes())?;

        let lines: Vec<String> = Reader::new(&path, 32_000)?
            .lines()
            .map(|f: Result<String, io::Error>| f.unwrap())
            .collect();

        let expect: Vec<String> = vec![
            String::from("read"),
            String::from("my"),
            String::from("lines"),
        ];

        assert_eq!(lines, expect);

        dir.close()
    }

    #[test]
    fn read_lines_from_gzip() -> io::Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("test.txt.gz");
        let mut file: File = File::create(&path)?;

        // "gzipped\nlines\n"
        let bytes: Vec<u8> = vec![
            0x1F, 0x8B, 0x08, 0x08, 0x6C, 0x3C, 0x41, 0x65, 0x00, 0x03, 0x74,
            0x65, 0x73, 0x74, 0x2E, 0x74, 0x78, 0x74, 0x00, 0x4B, 0xAF, 0xCA,
            0x2C, 0x28, 0x48, 0x4D, 0xE1, 0xCA, 0xC9, 0xCC, 0x4B, 0x2D, 0xE6,
            0x02, 0x00, 0x57, 0x62, 0x83, 0x79, 0x0E, 0x00, 0x00, 0x00
        ];
        file.write_all(&bytes)?;

        let lines: Vec<String> = Reader::new(&path, 32_000)?
            .lines()
            .map(|f: Result<String, io::Error>| f.unwrap())
            .collect();

        let expect: Vec<String> = vec![
            String::from("gzipped"),
            String::from("lines")
        ];

        assert_eq!(lines, expect);

        dir.close()
    }
}