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
    pub mod itertools {
        //! Contest-focused iterator extensions based on `rust-itertools` 0.15.0.
        //!
        //! The Cartesian product and uniqueness adapters are adapted from
        //! <https://github.com/rust-itertools/itertools>. They are
        //! modified to form a small, dependency-free subset suitable for bundling.
        //! See `third-party/itertools-LICENSE-MIT` for the upstream license.
        use std::cmp::Ordering;
        use std::collections::HashMap;
        use std::collections::hash_map::Entry;
        use std::fmt::{Display, Write as _};
        use std::hash::Hash;
        use std::iter::FusedIterator;
        /// Dependency-free iterator helpers for common competitive-programming tasks.
        pub trait Itertools: Iterator + Sized {
            fn collect_vec(self) -> Vec<Self::Item> {
                self.collect()
            }
            fn sorted(self) -> std::vec::IntoIter<Self::Item>
            where
                Self::Item: Ord,
            {
                let mut values = self.collect_vec();
                values.sort();
                values.into_iter()
            }
            fn sorted_by<F>(self, compare: F) -> std::vec::IntoIter<Self::Item>
            where
                F: FnMut(&Self::Item, &Self::Item) -> Ordering,
            {
                let mut values = self.collect_vec();
                values.sort_by(compare);
                values.into_iter()
            }
            fn sorted_by_key<K, F>(self, key: F) -> std::vec::IntoIter<Self::Item>
            where
                K: Ord,
                F: FnMut(&Self::Item) -> K,
            {
                let mut values = self.collect_vec();
                values.sort_by_key(key);
                values.into_iter()
            }
            fn join(self, separator: &str) -> String
            where
                Self::Item: Display,
            {
                let mut output = String::new();
                for (index, value) in self.enumerate() {
                    if index > 0 {
                        output.push_str(separator);
                    }
                    write!(& mut output, "{value}")
                        .expect("writing to a String cannot fail");
                }
                output
            }
            fn unique(self) -> Unique<Self>
            where
                Self::Item: Eq + Hash + Clone,
            {
                Unique {
                    iterator: self,
                    used: HashMap::new(),
                }
            }
            fn unique_by<V, F>(self, key: F) -> UniqueBy<Self, V, F>
            where
                V: Eq + Hash,
                F: FnMut(&Self::Item) -> V,
            {
                UniqueBy {
                    iterator: self,
                    used: HashMap::new(),
                    key,
                }
            }
            fn counts(self) -> HashMap<Self::Item, usize>
            where
                Self::Item: Eq + Hash,
            {
                self.counts_by(|value| value)
            }
            fn counts_by<K, F>(self, mut key: F) -> HashMap<K, usize>
            where
                K: Eq + Hash,
                F: FnMut(Self::Item) -> K,
            {
                let mut counts = HashMap::new();
                for value in self {
                    *counts.entry(key(value)).or_insert(0) += 1;
                }
                counts
            }
            fn cartesian_product<J>(self, other: J) -> Product<Self, J::IntoIter>
            where
                Self::Item: Clone,
                J: IntoIterator,
                J::IntoIter: Clone,
            {
                Product::new(self, other.into_iter())
            }
            fn all_equal(mut self) -> bool
            where
                Self::Item: PartialEq,
            {
                let Some(first) = self.next() else {
                    return true;
                };
                self.all(|value| value == first)
            }
        }
        impl<I: Iterator> Itertools for I {}
        /// A lazy adapter that retains the first item for each distinct value.
        #[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
        pub struct Unique<I: Iterator>
        where
            I::Item: Eq + Hash + Clone,
        {
            iterator: I,
            used: HashMap<I::Item, ()>,
        }
        impl<I> Iterator for Unique<I>
        where
            I: Iterator,
            I::Item: Eq + Hash + Clone,
        {
            type Item = I::Item;
            fn next(&mut self) -> Option<Self::Item> {
                self.iterator
                    .by_ref()
                    .find_map(|value| {
                        if let Entry::Vacant(entry) = self.used.entry(value) {
                            let result = entry.key().clone();
                            entry.insert(());
                            Some(result)
                        } else {
                            None
                        }
                    })
            }
            fn size_hint(&self) -> (usize, Option<usize>) {
                let (lower, upper) = self.iterator.size_hint();
                (usize::from(lower > 0 && self.used.is_empty()), upper)
            }
        }
        impl<I> FusedIterator for Unique<I>
        where
            I: FusedIterator,
            I::Item: Eq + Hash + Clone,
        {}
        /// A lazy adapter that retains the first item for each distinct key.
        #[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
        pub struct UniqueBy<I: Iterator, V, F> {
            iterator: I,
            used: HashMap<V, ()>,
            key: F,
        }
        impl<I, V, F> Iterator for UniqueBy<I, V, F>
        where
            I: Iterator,
            V: Eq + Hash,
            F: FnMut(&I::Item) -> V,
        {
            type Item = I::Item;
            fn next(&mut self) -> Option<Self::Item> {
                let Self { iterator, used, key } = self;
                iterator.find(|value| used.insert(key(value), ()).is_none())
            }
            fn size_hint(&self) -> (usize, Option<usize>) {
                let (lower, upper) = self.iterator.size_hint();
                (usize::from(lower > 0 && self.used.is_empty()), upper)
            }
        }
        impl<I, V, F> FusedIterator for UniqueBy<I, V, F>
        where
            I: FusedIterator,
            V: Eq + Hash,
            F: FnMut(&I::Item) -> V,
        {}
        /// A lazy iterator over the Cartesian product of two iterators.
        ///
        /// This follows upstream `itertools` ordering: the right iterator advances
        /// fastest, producing `(a0, b0), (a0, b1), (a1, b0), ...`.
        #[derive(Clone, Debug)]
        #[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
        pub struct Product<I: Iterator, J: Iterator> {
            left: I,
            current_left: Option<Option<I::Item>>,
            right: J,
            original_right: J,
        }
        impl<I, J> Product<I, J>
        where
            I: Iterator,
            J: Iterator + Clone,
            I::Item: Clone,
        {
            fn new(left: I, right: J) -> Self {
                Self {
                    left,
                    current_left: None,
                    right: right.clone(),
                    original_right: right,
                }
            }
        }
        impl<I, J> Iterator for Product<I, J>
        where
            I: Iterator,
            J: Iterator + Clone,
            I::Item: Clone,
        {
            type Item = (I::Item, J::Item);
            fn next(&mut self) -> Option<Self::Item> {
                let right = match self.right.next() {
                    Some(value) => value,
                    None => {
                        self.right = self.original_right.clone();
                        let value = self.right.next()?;
                        self.current_left = Some(self.left.next());
                        value
                    }
                };
                self.current_left
                    .get_or_insert_with(|| self.left.next())
                    .as_ref()
                    .map(|left| (left.clone(), right))
            }
            fn size_hint(&self) -> (usize, Option<usize>) {
                let (left_lower, left_upper) = self.left.size_hint();
                let (right_lower, right_upper) = self.original_right.size_hint();
                let mut lower = left_lower.saturating_mul(right_lower);
                let mut upper = left_upper
                    .zip(right_upper)
                    .and_then(|(left, right)| left.checked_mul(right));
                if matches!(self.current_left, Some(Some(_))) {
                    let (remaining_lower, remaining_upper) = self.right.size_hint();
                    lower = lower.saturating_add(remaining_lower);
                    upper = upper
                        .zip(remaining_upper)
                        .and_then(|(total, remaining)| total.checked_add(remaining));
                }
                (lower, upper)
            }
        }
        impl<I, J> FusedIterator for Product<I, J>
        where
            I: FusedIterator,
            J: FusedIterator + Clone,
            I::Item: Clone,
        {}
    }
    pub use io::{Cin, Cout};
    pub use itertools::{Itertools, Product, Unique, UniqueBy};
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
use std::{collections::HashSet, iter};
use cp_library::{Cin, Cout, End, Itertools};
fn to_digits(x: usize) -> Vec<usize> {
    x.to_string().as_bytes().into_iter().map(|b| (b - b'0') as usize).collect()
}
fn from_digits(ds: &[usize]) -> usize {
    assert!(! ds.is_empty());
    assert!(ds.iter().copied().all(| d | d <= 9));
    ds.into_iter().copied().fold(0, |acc, d| acc * 10 + d)
}
fn solve(a: usize, ds: &[usize]) -> usize {
    assert!(ds.is_sorted());
    assert!(ds.iter().copied().all(| d | d <= 9));
    let ds_set: HashSet<usize> = ds.iter().copied().collect();
    let ads = to_digits(a);
    let mut res = usize::MAX;
    let mut add = |digits: &[usize]| {
        res = res.min(a.abs_diff(from_digits(digits)));
    };
    if ads.len() > 1 {
        let sim = vec![ds[End]; ads.len() - 1];
        add(&sim)
    }
    {
        let mut sim = vec![if ds[0] == 0 && ds.len() > 1 { ds[1] } else { ds[0] }];
        sim.extend(iter::repeat_n(ds[0], ads.len()));
        add(&sim);
    }
    'outer: {
        let mut sim: Vec<usize> = Vec::new();
        for (i, ad) in ads.iter().copied().enumerate() {
            for &d in ds {
                if d < ad {
                    let mut sim = sim.clone();
                    sim.push(d);
                    sim.extend(iter::repeat_n(ds[End], ads.len() - (i + 1)));
                    add(&sim);
                } else if ad < d {
                    let mut sim = sim.clone();
                    sim.push(d);
                    sim.extend(iter::repeat_n(ds[0], ads.len() - (i + 1)));
                    add(&sim);
                }
            }
            if ds_set.contains(&ad) {
                sim.push(ad);
            } else {
                break 'outer;
            }
        }
        add(&sim);
    }
    res
}
fn main() {
    let mut cin = Cin::new();
    let mut cout = Cout::new();
    let t = cin.read();
    for _ in 0..t {
        let a: usize = cin.read();
        let n: usize = cin.read();
        let ds: Vec<usize> = cin.read_vec(n);
        let res = solve(a, &ds);
        cout.println(res);
    }
}
