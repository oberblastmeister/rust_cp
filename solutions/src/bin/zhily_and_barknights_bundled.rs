#[allow(warnings)]
mod cp_library {
    pub mod frac {
        use std::cmp::Ordering;
        use std::fmt::{self, Display, Formatter};
        use std::hash::{Hash, Hasher};
        use std::num::ParseIntError;
        use std::ops::{
            Add, AddAssign, BitXor, Div, DivAssign, Mul, MulAssign, Sub, SubAssign,
        };
        use std::str::FromStr;
        /// A nonnegative rational number stored in reduced form.
        ///
        /// The denominator is always positive. Construct fractions with [`Frac::new`]
        /// to preserve these invariants.
        #[derive(Clone, Copy, Debug)]
        pub struct Frac {
            pub num: usize,
            pub denom: usize,
        }
        pub fn F(x: usize) -> Frac {
            Frac::new(x, 1)
        }
        impl Frac {
            pub const ZERO: Self = Self { num: 0, denom: 1 };
            pub const ONE: Self = Self { num: 1, denom: 1 };
            /// Creates and reduces `num / denom`.
            ///
            /// # Panics
            ///
            /// Panics if `denom` is zero.
            pub fn new(num: usize, denom: usize) -> Self {
                assert!(denom != 0, "fraction denominator must be nonzero");
                if num == 0 {
                    return Self::ZERO;
                }
                let divisor = gcd(num, denom);
                Self {
                    num: num / divisor,
                    denom: denom / divisor,
                }
            }
            pub fn reciprocal(self) -> Self {
                assert!(self.num != 0, "zero has no reciprocal");
                Self::new(self.denom, self.num)
            }
            pub fn pow(self, mut exponent: usize) -> Self {
                let mut base = self;
                let mut result = Self::ONE;
                while exponent > 0 {
                    if exponent % 2 == 1 {
                        result *= base;
                    }
                    exponent /= 2;
                    if exponent > 0 {
                        base *= base;
                    }
                }
                result
            }
            pub fn is_zero(self) -> bool {
                self.num == 0
            }
            pub fn is_integer(self) -> bool {
                self.denom == 1
            }
        }
        impl Default for Frac {
            fn default() -> Self {
                Self::ZERO
            }
        }
        impl PartialEq for Frac {
            fn eq(&self, rhs: &Self) -> bool {
                self.num as u128 * rhs.denom as u128
                    == rhs.num as u128 * self.denom as u128
            }
        }
        impl Eq for Frac {}
        impl Hash for Frac {
            fn hash<H: Hasher>(&self, state: &mut H) {
                let reduced = Self::new(self.num, self.denom);
                reduced.num.hash(state);
                reduced.denom.hash(state);
            }
        }
        impl PartialOrd for Frac {
            fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
                Some(self.cmp(rhs))
            }
        }
        impl Ord for Frac {
            fn cmp(&self, rhs: &Self) -> Ordering {
                (self.num as u128 * rhs.denom as u128)
                    .cmp(&(rhs.num as u128 * self.denom as u128))
            }
        }
        impl From<usize> for Frac {
            fn from(value: usize) -> Self {
                Self::new(value, 1)
            }
        }
        impl Display for Frac {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
                if self.denom == 1 {
                    self.num.fmt(formatter)
                } else {
                    write!(formatter, "{}/{}", self.num, self.denom)
                }
            }
        }
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum ParseFracError {
            InvalidInteger(ParseIntError),
            InvalidFormat,
            ZeroDenominator,
        }
        impl Display for ParseFracError {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
                match self {
                    Self::InvalidInteger(error) => error.fmt(formatter),
                    Self::InvalidFormat => {
                        formatter
                            .write_str("expected an integer or a fraction `num/denom`")
                    }
                    Self::ZeroDenominator => {
                        formatter.write_str("fraction denominator must be nonzero")
                    }
                }
            }
        }
        impl std::error::Error for ParseFracError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    Self::InvalidInteger(error) => Some(error),
                    Self::InvalidFormat | Self::ZeroDenominator => None,
                }
            }
        }
        impl FromStr for Frac {
            type Err = ParseFracError;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let mut parts = value.split('/');
                let num = parts
                    .next()
                    .ok_or(ParseFracError::InvalidFormat)?
                    .parse()
                    .map_err(ParseFracError::InvalidInteger)?;
                let denom = match parts.next() {
                    Some(part) => part.parse().map_err(ParseFracError::InvalidInteger)?,
                    None => 1,
                };
                if parts.next().is_some() {
                    return Err(ParseFracError::InvalidFormat);
                }
                if denom == 0 {
                    return Err(ParseFracError::ZeroDenominator);
                }
                Ok(Self::new(num, denom))
            }
        }
        impl Add for Frac {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                let common = gcd(self.denom, rhs.denom);
                let lhs_scale = rhs.denom / common;
                let rhs_scale = self.denom / common;
                let lhs_num = checked_mul(self.num, lhs_scale);
                let rhs_num = checked_mul(rhs.num, rhs_scale);
                Self::new(
                    checked_add(lhs_num, rhs_num),
                    checked_mul(self.denom, lhs_scale),
                )
            }
        }
        impl Sub for Frac {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                let common = gcd(self.denom, rhs.denom);
                let lhs_scale = rhs.denom / common;
                let rhs_scale = self.denom / common;
                let lhs_num = checked_mul(self.num, lhs_scale);
                let rhs_num = checked_mul(rhs.num, rhs_scale);
                assert!(lhs_num >= rhs_num, "fraction subtraction would be negative");
                Self::new(lhs_num - rhs_num, checked_mul(self.denom, lhs_scale))
            }
        }
        impl Mul for Frac {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                let left_common = gcd(self.num, rhs.denom);
                let right_common = gcd(rhs.num, self.denom);
                Self::new(
                    checked_mul(self.num / left_common, rhs.num / right_common),
                    checked_mul(self.denom / right_common, rhs.denom / left_common),
                )
            }
        }
        impl Div for Frac {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                assert!(rhs.num != 0, "cannot divide by zero");
                let numerator_common = gcd(self.num, rhs.num);
                let denominator_common = gcd(rhs.denom, self.denom);
                Self::new(
                    checked_mul(
                        self.num / numerator_common,
                        rhs.denom / denominator_common,
                    ),
                    checked_mul(
                        self.denom / denominator_common,
                        rhs.num / numerator_common,
                    ),
                )
            }
        }
        impl AddAssign for Frac {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }
        impl SubAssign for Frac {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }
        impl MulAssign for Frac {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }
        impl DivAssign for Frac {
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs;
            }
        }
        impl BitXor<usize> for Frac {
            type Output = Self;
            fn bitxor(self, exponent: usize) -> Self::Output {
                self.pow(exponent)
            }
        }
        macro_rules! impl_integer_operation {
            ($trait:ident, $method:ident) => {
                impl $trait < usize > for Frac { type Output = Self; fn $method (self,
                rhs : usize) -> Self::Output { self.$method (Self::from(rhs)) } } impl
                $trait < Frac > for usize { type Output = Frac; fn $method (self, rhs :
                Frac) -> Self::Output { Frac::from(self).$method (rhs) } }
            };
        }
        impl_integer_operation!(Add, add);
        impl_integer_operation!(Sub, sub);
        impl_integer_operation!(Mul, mul);
        impl_integer_operation!(Div, div);
        const fn gcd(mut left: usize, mut right: usize) -> usize {
            while right != 0 {
                let remainder = left % right;
                left = right;
                right = remainder;
            }
            left
        }
        fn checked_add(left: usize, right: usize) -> usize {
            left.checked_add(right).expect("fraction arithmetic overflow")
        }
        fn checked_mul(left: usize, right: usize) -> usize {
            left.checked_mul(right).expect("fraction arithmetic overflow")
        }
        #[cfg(test)]
        mod tests {
            use std::collections::HashSet;
            use super::{Frac, ParseFracError};
            #[test]
            fn constructs_and_reduces_fractions() {
                assert_eq!(Frac::new(6, 8), Frac { num : 3, denom : 4 });
                assert_eq!(Frac::new(0, 99), Frac::ZERO);
                assert_eq!(Frac::default(), Frac::ZERO);
            }
            #[test]
            fn compares_equivalent_and_distinct_fractions() {
                assert_eq!(Frac { num : 1, denom : 2 }, Frac { num : 2, denom : 4 });
                assert!(Frac::new(2, 3) > Frac::new(3, 5));
                assert!(Frac::new(7, 4) > Frac::from(1));
                let set = HashSet::from([Frac { num: 1, denom: 2 }]);
                assert!(set.contains(& Frac { num : 2, denom : 4 }));
            }
            #[test]
            fn performs_arithmetic_and_assignment() {
                let mut value = Frac::new(1, 2);
                value += Frac::new(1, 3);
                value *= Frac::new(6, 5);
                value -= Frac::new(1, 4);
                value /= Frac::new(3, 2);
                assert_eq!(value, Frac::new(1, 2));
                assert_eq!(Frac::new(2, 3) + 1, Frac::new(5, 3));
                assert_eq!(2 * Frac::new(3, 4), Frac::new(3, 2));
                assert_eq!(2 / Frac::new(3, 4), Frac::new(8, 3));
            }
            #[test]
            fn exponentiates_and_takes_reciprocals() {
                assert_eq!(Frac::new(2, 3) ^ 3, Frac::new(8, 27));
                assert_eq!(Frac::new(2, 3).pow(0), Frac::ONE);
                assert_eq!(Frac::new(2, 3).reciprocal(), Frac::new(3, 2));
            }
            #[test]
            fn displays_and_parses() {
                assert_eq!(Frac::new(8, 4).to_string(), "2");
                assert_eq!(Frac::new(3, 4).to_string(), "3/4");
                assert_eq!("6/8".parse::< Frac > ().unwrap(), Frac::new(3, 4));
                assert_eq!("12".parse::< Frac > ().unwrap(), Frac::from(12));
                assert_eq!(
                    "1/0".parse::< Frac > (), Err(ParseFracError::ZeroDenominator)
                );
            }
            #[test]
            #[should_panic(expected = "fraction subtraction would be negative")]
            fn rejects_negative_results() {
                let _ = Frac::new(1, 3) - Frac::new(1, 2);
            }
            #[test]
            #[should_panic(expected = "cannot divide by zero")]
            fn rejects_division_by_zero() {
                let _ = Frac::ONE / Frac::ZERO;
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
    pub mod mod_arith {
        use std::fmt::{self, Display, Formatter};
        use std::ops::{
            Add, AddAssign, BitXor, Div, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign,
        };
        use std::str::FromStr;
        /// Computes `base.pow(exponent) mod modulus` with binary exponentiation.
        pub fn bin_exp(mut base: usize, mut exponent: usize, modulus: usize) -> usize {
            assert!(modulus > 0, "modulus must be positive");
            base %= modulus;
            let mut answer = 1 % modulus;
            while exponent > 0 {
                if exponent % 2 == 1 {
                    answer = ((answer as u128 * base as u128) % modulus as u128)
                        as usize;
                }
                exponent /= 2;
                base = ((base as u128 * base as u128) % modulus as u128) as usize;
            }
            answer
        }
        /// An integer modulo the positive compile-time constant `MOD`.
        ///
        /// Division uses Fermat's little theorem and therefore requires `MOD` to be
        /// prime and the divisor to be nonzero.
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
        pub struct ModUsize<const MOD: usize> {
            pub v: usize,
        }
        impl<const MOD: usize> ModUsize<MOD> {
            pub fn new(value: usize) -> Self {
                assert!(MOD > 0, "modulus must be positive");
                Self { v: value % MOD }
            }
            pub fn value(self) -> usize {
                self.v
            }
            pub fn pow(self, exponent: usize) -> Self {
                Self::new(bin_exp(self.v, exponent, MOD))
            }
            pub fn inv(self) -> Self {
                assert!(MOD > 1, "modulus must be greater than one for division");
                assert!(self.v != 0, "zero has no modular inverse");
                self.pow(MOD - 2)
            }
        }
        impl<const MOD: usize> From<usize> for ModUsize<MOD> {
            fn from(value: usize) -> Self {
                Self::new(value)
            }
        }
        impl<const MOD: usize> Display for ModUsize<MOD> {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
                self.v.fmt(formatter)
            }
        }
        impl<const MOD: usize> FromStr for ModUsize<MOD> {
            type Err = <usize as FromStr>::Err;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                value.parse::<usize>().map(Self::new)
            }
        }
        impl<const MOD: usize> Add for ModUsize<MOD> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self::new(((self.v as u128 + rhs.v as u128) % MOD as u128) as usize)
            }
        }
        impl<const MOD: usize> Sub for ModUsize<MOD> {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                if self.v >= rhs.v {
                    Self::new(self.v - rhs.v)
                } else {
                    Self::new(MOD - (rhs.v - self.v))
                }
            }
        }
        impl<const MOD: usize> Mul for ModUsize<MOD> {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                Self::new(((self.v as u128 * rhs.v as u128) % MOD as u128) as usize)
            }
        }
        impl<const MOD: usize> Div for ModUsize<MOD> {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                self * rhs.inv()
            }
        }
        impl<const MOD: usize> AddAssign for ModUsize<MOD> {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }
        impl<const MOD: usize> SubAssign for ModUsize<MOD> {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }
        impl<const MOD: usize> MulAssign for ModUsize<MOD> {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }
        impl<const MOD: usize> DivAssign for ModUsize<MOD> {
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs;
            }
        }
        impl<const MOD: usize> BitXor<usize> for ModUsize<MOD> {
            type Output = Self;
            fn bitxor(self, exponent: usize) -> Self::Output {
                self.pow(exponent)
            }
        }
        impl<const MOD: usize> BitXor for ModUsize<MOD> {
            type Output = Self;
            fn bitxor(self, exponent: Self) -> Self::Output {
                self.pow(exponent.v)
            }
        }
        impl<const MOD: usize> Rem<usize> for ModUsize<MOD> {
            type Output = usize;
            fn rem(self, modulus: usize) -> Self::Output {
                self.v % modulus
            }
        }
        impl<const MOD: usize> Rem for ModUsize<MOD> {
            type Output = usize;
            fn rem(self, modulus: Self) -> Self::Output {
                self.v % modulus.v
            }
        }
        impl<const MOD: usize> Add<usize> for ModUsize<MOD> {
            type Output = Self;
            fn add(self, rhs: usize) -> Self::Output {
                self + Self::new(rhs)
            }
        }
        impl<const MOD: usize> Add<ModUsize<MOD>> for usize {
            type Output = ModUsize<MOD>;
            fn add(self, rhs: ModUsize<MOD>) -> Self::Output {
                ModUsize::new(self) + rhs
            }
        }
        impl<const MOD: usize> Mul<usize> for ModUsize<MOD> {
            type Output = Self;
            fn mul(self, rhs: usize) -> Self::Output {
                self * Self::new(rhs)
            }
        }
        impl<const MOD: usize> Mul<ModUsize<MOD>> for usize {
            type Output = ModUsize<MOD>;
            fn mul(self, rhs: ModUsize<MOD>) -> Self::Output {
                ModUsize::new(self) * rhs
            }
        }
        pub fn create_fact_table<const MOD: usize>(n: usize) -> Vec<ModUsize<MOD>> {
            let mut res = vec![ModUsize::new(1); n + 1];
            for i in 1..=n {
                res[i] = res[i - 1] * ModUsize::new(i);
            }
            res
        }
        #[cfg(test)]
        mod tests {
            use super::{ModUsize, bin_exp};
            type Mod = ModUsize<1_000_000_007>;
            #[test]
            fn normalizes_values_and_parsed_input() {
                assert_eq!(Mod::new(1_000_000_006).v, 1_000_000_006);
                assert_eq!("1000000008".parse::< Mod > ().unwrap(), Mod::new(1));
            }
            #[test]
            fn performs_arithmetic_and_assignment() {
                let mut value = Mod::new(10);
                value += Mod::new(5);
                value *= Mod::new(3);
                value -= Mod::new(4);
                assert_eq!(value, Mod::new(41));
                assert_eq!(2 + value, Mod::new(43));
                assert_eq!(3 * value, Mod::new(123));
                assert_eq!(Mod::new(2) - Mod::new(3), Mod::new(1_000_000_006));
            }
            #[test]
            fn exponentiates_and_divides() {
                assert_eq!(bin_exp(2, 10, 1_000_000_007), 1024);
                assert_eq!(Mod::new(2) ^ 10_usize, Mod::new(1024));
                assert_eq!(Mod::new(10) / Mod::new(2), Mod::new(5));
            }
            #[test]
            fn avoids_overflow_during_multiplication() {
                type LargeMod = ModUsize<{ usize::MAX - 58 }>;
                let a = LargeMod::new(usize::MAX - 66);
                assert_eq!(a * a, LargeMod::new(64));
            }
        }
    }
    pub use frac::{Frac, ParseFracError};
    pub use io::{Cin, Cout};
    pub use itertools::{Itertools, Product, Unique, UniqueBy};
    pub use mod_arith::{ModUsize, bin_exp};
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

use cp_library::{Cin, Cout, End, Frac, Itertools, frac::F, mod_arith};
const MOD: usize = 998244353;
type musize = cp_library::ModUsize<MOD>;
fn M(x: usize) -> musize {
    musize::new(x)
}

fn solve(n: usize, a: Vec<usize>, b: Vec<usize>) -> musize {
    let mut pairs = (0..n)
        .cartesian_product(0..n)
        .filter(|(i, j)| i != j)
        .map(|(i, j)| F(b[i]) / F(b[j]))
        .collect_vec();
    pairs.sort_unstable();
    let fact = mod_arith::create_fact_table::<MOD>(n);
    let mut res = M(0);
    for i in 0..n {
        for j in (i + 1)..n {
            let k = F(a[j]) / F(a[i]);
            let pi = pairs.partition_point(|&x| x <= k);
            let num = pairs[pi..].len();
            let rest = fact[n - 2];
            res += num * rest;
        }
    }
    res / fact[n]
}
fn main() {
    let mut cin = Cin::new();
    let mut cout = Cout::new();
    let t = cin.read();
    for _ in 0..t {
        let n: usize = cin.read();
        let a: Vec<usize> = cin.read_vec(n);
        let b: Vec<usize> = cin.read_vec(n);
        let res = solve(n, a, b);
        cout.println(res);
    }
}
