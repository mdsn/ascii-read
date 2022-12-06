use ascii::{AsciiChar, AsciiStr, AsciiString};
use std::io;
use std::io::{BufRead, BufReader, Read, StdinLock};
use thiserror::Error;

/// Error returned when reading from a type that implements [`AsciiBufRead`]. This may be
/// an I/O error or an error when the sequence of `u8` are not all ASCII.
#[derive(Error, Debug)]
pub enum AsciiReadError {
    #[error("io error")]
    Io(#[from] io::Error),
    #[error("ascii decoding error")]
    Ascii(#[from] ascii::AsAsciiStrError),
}

pub trait AsciiBufRead: BufRead {
    /// Read all bytes until a newline is reached, and append them to the provided buffer.
    ///
    /// This function will read bytes from the underlying stream until the newline delimiter
    /// is found. Once found, all bytes, including the delimiter, will be appended to `buf`.
    ///
    /// If successful, this function returns the total number of bytes read.
    ///
    /// If this function returns [`Ok(0)`], the stream has reached `EOF`.
    ///
    /// [`Ok(0)`]: Ok
    fn read_ascii_line(&mut self, buf: &mut AsciiString) -> Result<usize, AsciiReadError> {
        let mut s = String::new();
        let n = self.read_line(&mut s)?;
        buf.push_str(AsciiStr::from_ascii(&s)?);
        Ok(n)
    }

    /// Returns an iterator over the lines of this reader.
    ///
    /// The iterator yields instances of <code>[Result]<[AsciiString], [AsciiReadError]></code>.
    /// Each string will *not* have a newline byte or `CRLF` at the end.
    fn ascii_lines(self) -> AsciiLines<Self>
    where
        Self: Sized,
    {
        AsciiLines { buf: self }
    }
}

impl<R: Read> AsciiBufRead for BufReader<R> {}
impl AsciiBufRead for StdinLock<'_> {}

/// An iterator over the lines of an instance of [`AsciiBufRead`].
///
/// This struct is created by calling [`ascii_lines`] on an [`AsciiBufRead`].
///
/// [`AsciiBufRead`]: AsciiBufRead
/// [`ascii_lines`]: AsciiBufRead::ascii_lines
#[derive(Debug)]
pub struct AsciiLines<B> {
    buf: B,
}

impl<B: AsciiBufRead> Iterator for AsciiLines<B> {
    type Item = Result<AsciiString, AsciiReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = AsciiString::new();
        match self.buf.read_ascii_line(&mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                if buf[buf.len() - 1] == AsciiChar::LineFeed {
                    let _ = buf.pop();
                    if !buf.is_empty() && buf[buf.len() - 1] == AsciiChar::CarriageReturn {
                        let _ = buf.pop();
                    }
                }
                Some(Ok(buf))
            }
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::path::Path;

    // TODO test StdinLock impl
    // TODO test io and ascii errors
    // TODO test Iterator impl

    #[test]
    fn test_read_ascii_line_bufreader() {
        let manifest_path = Path::new(
            &env::var("CARGO_MANIFEST_DIR")
                .expect("Environment variable CARGO_MANIFEST_DIR not set."),
        )
        .join("resources/test/file.txt");
        let file = File::open(&manifest_path).expect("Failed to open {manifest_path:?}");
        let mut reader = BufReader::new(file);
        let mut asciistring = AsciiString::new();
        loop {
            let n = reader
                .read_ascii_line(&mut asciistring)
                .expect("Panic at read_ascii_line()");
            if n == 0 {
                break;
            }
        }
        assert_eq!(asciistring, "this is a test file\nsecond line\nhola\n");
    }
}
