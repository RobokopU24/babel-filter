use flate2::write::GzEncoder;
use flate2::Compression;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

/// Buffered file writer supporting optional gzip compression
pub struct Writer {
    writer: Box<dyn Write>,
}

impl Writer {
    /// Creates a new file writer given a `Path`. If the path ends in `.gz`, it will encode as
    /// a gzipped file using `flate2`.
    ///
    /// Returns `Err` if there is a problem creating the file
    pub fn new<P>(path: P, buffer_capacity: usize) -> io::Result<Writer>
    where
        P: AsRef<Path>,
    {
        let file = File::create(&path)?;

        let writer: Box<dyn Write> = if path.as_ref().extension() == Some(&OsStr::new("gz")) {
            Box::new(BufWriter::with_capacity(
                buffer_capacity,
                GzEncoder::new(file, Compression::default()),
            ))
        } else {
            Box::new(BufWriter::with_capacity(buffer_capacity, file))
        };

        Ok(Writer { writer })
    }

    /// Appends a string `line` with a newline character (`\n`) at the end
    ///
    /// Returns `Err` if there is a problem writing to the file
    pub fn write_line(&mut self, line: &str) -> io::Result<&mut Writer> {
        self.writer.write_all(line.as_bytes())?;
        self.writer.write_all(b"\n")?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use std::{io, fs, path::PathBuf};
    use tempfile::tempdir;

    use super::Writer;

    #[test]
    fn writes_line_to_plaintext() -> io::Result<()> {
        let dir = tempdir()?;
        let path: PathBuf = dir.path().join("test.txt");

        Writer::new(&path, 32_000)?
            .write_line("hello")?
            .write_line("world")?;


        let contents = fs::read_to_string(&path)?;

        assert_eq!(&contents, "hello\nworld\n");
        
        dir.close()
    }

    #[test]
    fn writes_line_to_gzip() -> io::Result<()> {
        let dir = tempdir()?;
        let path: PathBuf = dir.path().join("test.txt.gz");

        Writer::new(&path, 32_000)?
            .write_line("gzipped")?;


        let contents: Vec<u8> = fs::read(&path)?;
        let expected: Vec<u8> = vec![31, 139, 8, 0, 0, 0, 0, 0, 0, 255, 75, 175, 202, 44, 40, 72, 77, 225, 2, 0, 25, 169, 90, 229, 8, 0, 0, 0]; 

        assert_eq!(contents, expected);
        
        dir.close()
    }
}