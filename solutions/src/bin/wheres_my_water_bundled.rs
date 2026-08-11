#[allow(warnings)]
mod cp_library {
    pub mod algebra {
        use std::{marker::PhantomData, ops::{Add, Div, Mul, Neg, Rem, Sub}};
        pub trait MulInv {
            fn mul_inv(self) -> Self;
        }
        pub trait Monoid {
            type T;
            const EMPTY: Self::T;
            fn append(x: Self::T, y: Self::T) -> Self::T;
        }
        pub trait Group: Monoid {
            fn inverse(x: Self::T) -> Self::T;
        }
        pub trait NumOps<
            Rhs = Self,
            Output = Self,
        >: Add<
                Rhs,
                Output = Output,
            > + Sub<
                Rhs,
                Output = Output,
            > + Mul<
                Rhs,
                Output = Output,
            > + Div<Rhs, Output = Output> + Rem<Rhs, Output = Output> {}
        impl<T, Rhs, Output> NumOps<Rhs, Output> for T
        where
            T: Add<Rhs, Output = Output> + Sub<Rhs, Output = Output>
                + Mul<Rhs, Output = Output> + Div<Rhs, Output = Output>
                + Rem<Rhs, Output = Output>,
        {}
        pub trait Zero: Sized {
            const ZERO: Self;
        }
        pub trait One: Sized {
            const ONE: Self;
        }
        macro_rules! impl_zero_one {
            ($($ty:ty),+ $(,)?) => {
                $(impl Zero for $ty { const ZERO : Self = 0 as Self; } impl One for $ty {
                const ONE : Self = 1 as Self; })+
            };
        }
        impl_zero_one!(
            i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64,
        );
        #[cfg(test)]
        mod tests {
            use super::{One, Zero};
            macro_rules! assert_zero_one {
                ($($ty:ty),+ $(,)?) => {
                    $(assert_eq!(<$ty as Zero >::ZERO, 0 as $ty); assert_eq!(<$ty as One
                    >::ONE, 1 as $ty);)+
                };
            }
            #[test]
            fn primitive_numbers_have_zero_and_one() {
                assert_zero_one!(
                    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32,
                    f64,
                );
            }
        }
        pub trait Num: PartialEq + Zero + One + NumOps {}
        impl<T> Num for T
        where
            T: PartialEq + Zero + One + NumOps,
        {}
        pub struct AddMonoid<T>(PhantomData<T>);
        impl<T: Num> Monoid for AddMonoid<T> {
            type T = T;
            const EMPTY: T = T::ZERO;
            fn append(x: T, y: T) -> T {
                x + y
            }
        }
        impl<T: Num + Neg<Output = T>> Group for AddMonoid<T> {
            fn inverse(x: Self::T) -> Self::T {
                -x
            }
        }
        pub struct MulMonoid<T>(PhantomData<T>);
        impl<T: Num> Monoid for MulMonoid<T> {
            type T = T;
            const EMPTY: T = T::ONE;
            fn append(x: T, y: T) -> T {
                x * y
            }
        }
        impl<T: Num + MulInv> Group for MulMonoid<T> {
            fn inverse(x: Self::T) -> Self::T {
                x.mul_inv()
            }
        }
    }
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
    pub mod prefix_sum {
        use std::ops::{Bound, RangeBounds};
        use crate::cp_library::algebra::Group;
        pub struct PrefixSum<G: Group> {
            data: Box<[G::T]>,
        }
        impl<G: Group> Clone for PrefixSum<G>
        where
            G::T: Clone,
        {
            fn clone(&self) -> Self {
                Self { data: self.data.clone() }
            }
        }
        impl<G: Group> std::fmt::Debug for PrefixSum<G>
        where
            G::T: std::fmt::Debug,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("PrefixSum").field("data", &self.data).finish()
            }
        }
        impl<G: Group> PrefixSum<G>
        where
            G::T: Clone,
        {
            pub fn from_vec(mut values: Vec<G::T>) -> Self {
                let n = values.len();
                values.resize_with(n + 1, || G::EMPTY);
                values.rotate_right(1);
                values.shrink_to_fit();
                for i in 1..=n {
                    values[i] = G::append(values[i - 1].clone(), values[i].clone());
                }
                Self {
                    data: values.into_boxed_slice(),
                }
            }
            pub fn get(&self, index: usize) -> G::T {
                assert!(index < self.data.len(), "prefix sum index out of bounds");
                self.data[index].clone()
            }
            pub fn query<R: RangeBounds<usize>>(&self, range: R) -> G::T {
                let start = match range.start_bound() {
                    Bound::Included(&start) => start,
                    Bound::Excluded(&start) => {
                        start.checked_add(1).expect("prefix sum range start overflow")
                    }
                    Bound::Unbounded => 0,
                };
                let end = match range.end_bound() {
                    Bound::Included(&end) => {
                        end.checked_add(1).expect("prefix sum range end overflow")
                    }
                    Bound::Excluded(&end) => end,
                    Bound::Unbounded => self.data.len() - 1,
                };
                self.query_bounds(start, end)
            }
            pub fn query_bounds(&self, start: usize, end: usize) -> G::T {
                G::append(self.data[end].clone(), G::inverse(self.data[start].clone()))
            }
        }
        impl<G: Group> FromIterator<G::T> for PrefixSum<G>
        where
            G::T: Clone,
        {
            fn from_iter<I: IntoIterator<Item = G::T>>(iter: I) -> Self {
                Self::from_vec(iter.into_iter().collect())
            }
        }
    }
    pub use io::{Cin, Cout};
    pub use itertools::{Itertools, Product, Unique, UniqueBy};
    pub use prefix_sum::PrefixSum;
    pub fn virtual_partition_point<F>(start: usize, end: usize, mut f: F) -> usize
    where
        F: FnMut(usize) -> bool,
    {
        assert!(start <= end);
        let mut lo = start;
        let mut hi = end;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if f(mid) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    }
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
use cp_library::{
    Cin, Cout, End, Itertools, PrefixSum, algebra::AddMonoid, virtual_partition_point,
};
#[derive(Debug)]
struct Info {
    heights: Vec<usize>,
    heights_sum: PrefixSum<AddMonoid<isize>>,
}
impl Info {
    fn new(i: usize, heights: &[usize]) -> Info {
        let mut heights = heights.to_vec();
        for j in (0..i).rev() {
            heights[j] = heights[j].max(heights[j + 1]);
        }
        for j in (i + 1)..heights.len() {
            heights[j] = heights[j].max(heights[j - 1]);
        }
        let heights_sum = heights.clone().into_iter().map(|x| x as isize).collect();
        Info { heights, heights_sum }
    }
}
fn solve(grid_height: usize, heights: Vec<usize>) -> usize {
    if heights.len() == 1 {
        return grid_height - heights[0];
    }
    let n = heights.len();
    let infos = (0..n).map(|i| Info::new(i, &heights)).collect_vec();
    let mut res = n * grid_height;
    for i in 0..n {
        for j in (i + 1)..n {
            let pi = virtual_partition_point(
                i,
                j,
                |k| infos[i].heights[k] <= infos[j].heights[k],
            );
            let new_res = if pi == i {
                infos[j].heights_sum.query(..) as usize
            } else {
                let r1 = infos[i].heights_sum.query(..i) as usize;
                let r2 = infos[i].heights_sum.query(i..pi) as usize;
                let r3 = infos[j].heights_sum.query(pi..j) as usize;
                let r4 = infos[j].heights_sum.query(j..) as usize;
                r1 + r2 + r3 + r4
            };
            res = res.min(new_res);
        }
    }
    n * grid_height - res
}
fn main() {
    let mut cin = Cin::new();
    let mut cout = Cout::new();
    let t: usize = cin.read();
    for _ in 0..t {
        let n: usize = cin.read();
        let h: usize = cin.read();
        let a: Vec<usize> = cin.read_vec(n);
        let res = solve(h, a);
        cout.println(res);
    }
}
