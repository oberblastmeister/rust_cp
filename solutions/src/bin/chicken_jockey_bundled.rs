#[allow(warnings)]
mod cp_library {
    pub mod io {
        use std::any::type_name;
        use std::fmt::{self, Display};
        use std::io::{self, BufWriter, Read, Stdout, Write};
        use std::str::FromStr;
        /// Buffered, whitespace-delimited input similar to C++'s `cin`.
        pub struct Cin {
            input: Vec<u8>,
            position: usize,
        }
        impl Cin {
            /// Reads all of standard input into memory.
            pub fn new() -> Self {
                Self::from_reader(io::stdin())
            }
            pub fn from_reader(mut reader: impl Read) -> Self {
                let mut input = Vec::new();
                reader.read_to_end(&mut input).expect("failed to read input");
                Self { input, position: 0 }
            }
            /// Reads and parses the next whitespace-delimited value.
            pub fn read<T: FromStr>(&mut self) -> T {
                self.read_opt()
                    .unwrap_or_else(|| {
                        panic!("expected another {} in input", type_name::< T > ())
                    })
            }
            /// Reads the next value, or returns `None` at the end of input.
            pub fn read_opt<T: FromStr>(&mut self) -> Option<T> {
                while self
                    .input
                    .get(self.position)
                    .is_some_and(|byte| byte.is_ascii_whitespace())
                {
                    self.position += 1;
                }
                if self.position == self.input.len() {
                    return None;
                }
                let start = self.position;
                while self
                    .input
                    .get(self.position)
                    .is_some_and(|byte| !byte.is_ascii_whitespace())
                {
                    self.position += 1;
                }
                let token = std::str::from_utf8(&self.input[start..self.position])
                    .expect("input was not valid UTF-8");
                Some(
                    token
                        .parse()
                        .unwrap_or_else(|_| {
                            panic!(
                                "failed to parse `{token}` as {}", type_name::< T > ()
                            )
                        }),
                )
            }
            pub fn read_vec<T: FromStr>(&mut self, len: usize) -> Vec<T> {
                (0..len).map(|_| self.read()).collect()
            }
            pub fn read_chars(&mut self) -> Vec<char> {
                self.read::<String>().chars().collect()
            }
        }
        impl Default for Cin {
            fn default() -> Self {
                Self::new()
            }
        }
        /// Buffered output similar to C++'s `cout`.
        pub struct Cout<W: Write = BufWriter<Stdout>> {
            writer: W,
        }
        impl Cout<BufWriter<Stdout>> {
            pub fn new() -> Self {
                Self::from_writer(BufWriter::new(io::stdout()))
            }
        }
        impl Default for Cout<BufWriter<Stdout>> {
            fn default() -> Self {
                Self::new()
            }
        }
        impl<W: Write> Cout<W> {
            pub fn from_writer(writer: W) -> Self {
                Self { writer }
            }
            pub fn print(&mut self, value: impl Display) -> &mut Self {
                write!(self.writer, "{value}").expect("failed to write output");
                self
            }
            pub fn println(&mut self, value: impl Display) -> &mut Self {
                writeln!(self.writer, "{value}").expect("failed to write output");
                self
            }
            pub fn space(&mut self) -> &mut Self {
                self.print(' ')
            }
            pub fn newline(&mut self) -> &mut Self {
                self.print('\n')
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
            pub fn flush(&mut self) {
                self.writer.flush().expect("failed to flush output");
            }
            pub fn into_inner(mut self) -> io::Result<W> {
                self.writer.flush()?;
                Ok(self.writer)
            }
        }
        impl<W: Write> Write for Cout<W> {
            fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
                self.writer.write(buffer)
            }
            fn flush(&mut self) -> io::Result<()> {
                self.writer.flush()
            }
            fn write_fmt(&mut self, arguments: fmt::Arguments<'_>) -> io::Result<()> {
                self.writer.write_fmt(arguments)
            }
        }
        #[cfg(test)]
        mod tests {
            use super::{Cin, Cout};
            #[test]
            fn reads_typed_tokens_vectors_and_characters() {
                let mut cin = Cin::from_reader("3 10 -2 7 hello".as_bytes());
                let len: usize = cin.read();
                assert_eq!(cin.read_vec::< i32 > (len), [10, - 2, 7]);
                assert_eq!(cin.read_chars(), ['h', 'e', 'l', 'l', 'o']);
                assert_eq!(cin.read_opt::< i32 > (), None);
            }
            #[test]
            fn writes_chainable_buffered_output() {
                let mut cout = Cout::from_writer(Vec::new());
                cout.print("answer:").space().println(42);
                cout.print_iter([1, 2, 3], " ").newline();
                assert_eq!(cout.into_inner().unwrap(), b"answer: 42\n1 2 3\n");
            }
        }
    }
    pub use io::{Cin, Cout};
    pub struct End;
    impl<T> std::ops::Index<End> for Vec<T> {
        type Output = T;
        fn index(&self, _: End) -> &Self::Output {
            self.last().expect("cannot index the end of an empty vector")
        }
    }
    impl<T> std::ops::IndexMut<End> for Vec<T> {
        fn index_mut(&mut self, _: End) -> &mut Self::Output {
            self.last_mut().expect("cannot index the end of an empty vector")
        }
    }
    impl<T> std::ops::Index<End> for [T] {
        type Output = T;
        fn index(&self, _: End) -> &Self::Output {
            self.last().expect("cannot index the end of an empty slice")
        }
    }
    impl<T> std::ops::IndexMut<End> for [T] {
        fn index_mut(&mut self, _: End) -> &mut Self::Output {
            self.last_mut().expect("cannot index the end of an empty slice")
        }
    }
    impl std::ops::Index<End> for str {
        type Output = str;
        fn index(&self, _: End) -> &Self::Output {
            let start = self
                .char_indices()
                .next_back()
                .map(|(index, _)| index)
                .expect("cannot index the end of an empty string slice");
            &self[start..]
        }
    }
    impl std::ops::IndexMut<End> for str {
        fn index_mut(&mut self, _: End) -> &mut Self::Output {
            let start = self
                .char_indices()
                .next_back()
                .map(|(index, _)| index)
                .expect("cannot index the end of an empty string slice");
            &mut self[start..]
        }
    }
    impl std::ops::Index<End> for String {
        type Output = str;
        fn index(&self, index: End) -> &Self::Output {
            <str as std::ops::Index<End>>::index(self.as_str(), index)
        }
    }
    impl std::ops::IndexMut<End> for String {
        fn index_mut(&mut self, index: End) -> &mut Self::Output {
            <str as std::ops::IndexMut<End>>::index_mut(self.as_mut_str(), index)
        }
    }
}
use cp_library::{Cin, Cout, End};
fn solve(h: Vec<usize>) -> usize {
    let n = h.len();
    let mut dp = vec![usize::MAX; n];
    dp[0] = h[0];
    for i in 1..dp.len() {
        dp[i] = dp[i]
            .min(
                h[i].saturating_sub(i) + h[i - 1] + (if i > 1 { dp[i - 2] } else { 0 }),
            );
        dp[i] = dp[i].min(h[i].saturating_sub(1) + dp[i - 1]);
    }
    dp[End]
}
fn main() {
    let mut cin = Cin::new();
    let mut cout = Cout::new();
    let t = cin.read();
    for _ in 0..t {
        let n: usize = cin.read();
        let h: Vec<usize> = cin.read_vec(n);
        let res = solve(h);
        cout.println(res);
    }
}
