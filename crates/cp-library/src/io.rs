use std::io::{self, IoSlice, Read, Write};
use std::str::FromStr;

const BUFFER_SIZE: usize = 8 * 1024;
const MIN_FREE_SPACE: usize = 4 * 1024;

/// Buffered, whitespace-delimited byte input.
///
/// A token is a non-empty byte slice delimited by ASCII whitespace.
pub struct Reader {
    reader: Box<dyn Read>,
    buffer: Vec<u8>,
    // start..end is the range containing bytes read from the input.
    start: usize,
    end: usize,
    exhausted: bool,
}

impl Reader {
    pub fn from_read(reader: impl Read + 'static) -> Self {
        Self {
            reader: Box::new(reader),
            buffer: vec![0; BUFFER_SIZE],
            start: 0,
            end: 0,
            exhausted: false,
        }
    }

    fn free_space(&self) -> usize {
        self.buffer.len() - self.end
    }

    fn rebase(&mut self) {
        self.buffer.copy_within(self.start..self.end, 0);
        self.end -= self.start;
        self.start = 0;
    }

    fn fill(&mut self) -> io::Result<()> {
        loop {
            match self.reader.read(&mut self.buffer[self.end..]) {
                Ok(0) => {
                    self.exhausted = true;
                    return Ok(());
                }
                Ok(bytes_read) => {
                    self.end += bytes_read;
                    return Ok(());
                }
                Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                Err(error) => return Err(error),
            }
        }
    }

    // Invariant: this may only increase the size of the valid range.
    fn read_more(&mut self) -> io::Result<()> {
        if self.free_space() < MIN_FREE_SPACE {
            self.rebase();

            if self.free_space() < MIN_FREE_SPACE {
                self.buffer.resize(self.buffer.len() * 2, 0);
            }
        }

        self.fill()?;
        Ok(())
    }

    pub fn skip_while<F: FnMut(u8) -> bool>(&mut self, mut f: F) -> io::Result<()> {
        loop {
            while self.start < self.end && f(self.buffer[self.start]) {
                self.start += 1;
            }
            if self.start < self.end {
                return Ok(());
            }
            if self.exhausted {
                return Ok(());
            }
            self.rebase();
            self.fill()?;
        }
    }

    pub fn read_token(&mut self) -> io::Result<Option<&[u8]>> {
        loop {
            while self.start < self.end && self.buffer[self.start].is_ascii_whitespace() {
                self.start += 1;
            }
            if self.start < self.end {
                break;
            }
            if self.exhausted {
                assert_eq!(self.start, self.end);
                return Ok(None);
            }
            self.read_more()?;
        }

        // Everything from self.start..(self.start + off) is not whitespace.
        let mut off = 1;
        loop {
            assert!(self.start < self.end);
            if let Some(end_off) = self.buffer[(self.start + off)..self.end]
                .iter()
                .position(|byte| byte.is_ascii_whitespace())
            {
                off += end_off;
                let token_start = self.start;
                let token_end = self.start + off;
                self.start = token_end;
                return Ok(Some(&self.buffer[token_start..token_end]));
            }

            // The entire valid range is not whitespace.
            off = self.end - self.start;
            if self.exhausted {
                let token_start = self.start;
                let token_end = self.end;
                self.start = token_end;
                return Ok(Some(&self.buffer[token_start..token_end]));
            }
            self.read_more()?;
        }
    }
}

/// Buffered byte output.
pub struct Writer {
    writer: Box<dyn Write>,
    buffer: Box<[u8]>,
    position: usize,
}

fn write_all_vectored(writer: &mut dyn Write, mut slices: &mut [IoSlice<'_>]) -> io::Result<()> {
    while !slices.is_empty() {
        match writer.write_vectored(slices) {
            Ok(0) => {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "failed to write buffered output",
                ));
            }
            Ok(bytes_written) => IoSlice::advance_slices(&mut slices, bytes_written),
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

impl Writer {
    pub fn from_write(writer: impl Write + 'static) -> Self {
        Self {
            writer: Box::new(writer),
            buffer: vec![0; BUFFER_SIZE].into_boxed_slice(),
            position: 0,
        }
    }

    fn write_buffered(&mut self, input: &[u8]) -> io::Result<()> {
        let free_space = self.buffer.len() - self.position;
        if input.len() <= free_space {
            let end = self.position + input.len();
            self.buffer[self.position..end].copy_from_slice(input);
            self.position = end;
            return Ok(());
        }

        let mut slices = [
            IoSlice::new(&self.buffer[..self.position]),
            IoSlice::new(input),
        ];
        write_all_vectored(&mut *self.writer, &mut slices)?;
        self.position = 0;
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.write_all(&self.buffer[..self.position])?;
        self.position = 0;
        self.writer.flush()
    }

    pub fn write(&mut self, input: &[u8]) -> &mut Self {
        self.write_buffered(input)
            .expect("failed to write buffered output");
        self
    }
}

impl Write for Writer {
    fn write(&mut self, input: &[u8]) -> io::Result<usize> {
        self.write_buffered(input)?;
        Ok(input.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flush()
    }
}

impl Drop for Writer {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}
