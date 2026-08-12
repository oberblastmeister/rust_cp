use crate::io::{Reader, Writer};
use std::any::type_name;
use std::fmt::Display;
use std::io::{Read, Write};
use std::str::FromStr;

/// Typed input convenience wrapper around [`Reader`].
pub struct Cin {
    reader: Reader,
}

impl Cin {
    pub fn new() -> Self {
        Self::with_reader(Reader::from_read(std::io::stdin()))
    }

    pub fn from_reader(reader: impl Read + 'static) -> Self {
        Self::with_reader(Reader::from_read(reader))
    }

    pub fn with_reader(reader: Reader) -> Self {
        Self { reader }
    }

    pub fn reader(&mut self) -> &mut Reader {
        &mut self.reader
    }

    pub fn into_reader(self) -> Reader {
        self.reader
    }

    /// Reads and parses the next whitespace-delimited value.
    pub fn read<T: FromStr>(&mut self) -> T {
        self.read_opt()
            .unwrap_or_else(|| panic!("expected another {} in input", type_name::<T>()))
    }

    /// Reads the next value, or returns `None` at the end of input.
    pub fn read_opt<T: FromStr>(&mut self) -> Option<T> {
        self.reader
            .read_token()
            .expect("failed to read input")
            .map(|token| {
                let token = str::from_utf8(token).unwrap();
                token
                    .parse()
                    .unwrap_or_else(|_| panic!("failed to parse `{token}` as {}", type_name::<T>()))
            })
    }

    pub fn read_vec<T: FromStr>(&mut self, len: usize) -> Vec<T> {
        (0..len).map(|_| self.read()).collect()
    }

    pub fn read_chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}

/// Formatted output convenience wrapper around [`Writer`].
pub struct Cout {
    writer: Writer,
}

impl Cout {
    pub fn new() -> Self {
        Self::from_writer(Writer::from_write(std::io::stdout()))
    }

    pub fn from_write(writer: impl Write + 'static) -> Self {
        Self::from_writer(Writer::from_write(writer))
    }

    pub fn from_writer(writer: Writer) -> Self {
        Self { writer }
    }

    pub fn writer(&mut self) -> &mut Writer {
        &mut self.writer
    }

    pub fn into_writer(self) -> Writer {
        self.writer
    }

    pub fn flush(&mut self) {
        self.writer.flush().expect("Failed to flush the buffer");
    }

    pub fn print(&mut self, value: impl Display) -> &mut Self {
        write!(self.writer, "{value}").expect("failed to write output");
        self
    }

    pub fn println(&mut self, value: impl Display) -> &mut Self {
        writeln!(self.writer, "{value}").expect("failed to write output");
        self
    }

    pub fn print_iter<I>(&mut self, values: I, separator: &str) -> &mut Self
    where
        I: IntoIterator,
        I::Item: Display,
    {
        for (index, value) in values.into_iter().enumerate() {
            if index > 0 {
                self.print(separator);
            }
            self.print(value);
        }
        self
    }
}
