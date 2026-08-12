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
            fn read_more(&mut self) -> io::Result<()> {
                if self.free_space() < MIN_FREE_SPACE {
                    self.buffer.copy_within(self.start..self.end, 0);
                    self.end -= self.start;
                    self.start = 0;
                    if self.free_space() < MIN_FREE_SPACE {
                        self.buffer.resize(self.buffer.len() * 2, 0);
                    }
                }
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
                        Err(error) if error.kind() == io::ErrorKind::Interrupted => {
                            continue;
                        }
                        Err(error) => return Err(error),
                    }
                }
            }
            pub fn read_token(&mut self) -> io::Result<Option<&[u8]>> {
                loop {
                    while self.start < self.end
                        && self.buffer[self.start].is_ascii_whitespace()
                    {
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
                let mut off = 1;
                loop {
                    assert!(self.start < self.end);
                    if let Some(end_off) = self
                        .buffer[(self.start + off)..self.end]
                        .iter()
                        .position(|byte| byte.is_ascii_whitespace())
                    {
                        off += end_off;
                        let token_start = self.start;
                        let token_end = self.start + off;
                        self.start = token_end;
                        return Ok(Some(&self.buffer[token_start..token_end]));
                    }
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
        fn write_all_vectored(
            writer: &mut dyn Write,
            mut slices: &mut [IoSlice<'_>],
        ) -> io::Result<()> {
            while !slices.is_empty() {
                match writer.write_vectored(slices) {
                    Ok(0) => {
                        return Err(
                            io::Error::new(
                                io::ErrorKind::WriteZero,
                                "failed to write buffered output",
                            ),
                        );
                    }
                    Ok(bytes_written) => {
                        IoSlice::advance_slices(&mut slices, bytes_written)
                    }
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
                self.write_buffered(input).expect("failed to write buffered output");
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
    }
    pub mod cio {
        use crate::cp_library::io::{Reader, Writer};
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
                    .unwrap_or_else(|| {
                        panic!("expected another {} in input", type_name::< T > ())
                    })
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
                            .unwrap_or_else(|_| {
                                panic!(
                                    "failed to parse `{token}` as {}", type_name::< T > ()
                                )
                            })
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
        use crate::cp_library::algebra::MulInv;
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
            pub fn mul_inv(self) -> Self {
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
                self * rhs.mul_inv()
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
        impl<const MOD: usize> MulInv for ModUsize<MOD> {
            fn mul_inv(self) -> Self {
                self.mul_inv()
            }
        }
        pub struct FactTable<const MOD: usize>(pub Vec<ModUsize<MOD>>);
        impl<const MOD: usize> FactTable<MOD> {
            pub fn new(n: usize) -> Self {
                let mut res = vec![ModUsize::new(1); n + 1];
                for i in 1..=n {
                    res[i] = res[i - 1] * ModUsize::new(i);
                }
                FactTable(res)
            }
            pub fn choose(&self, n: usize, k: usize) -> ModUsize<MOD> {
                self.0[n] / (self.0[n - k] * self.0[k])
            }
        }
        impl<const MOD: usize> std::ops::Index<usize> for FactTable<MOD> {
            type Output = ModUsize<MOD>;
            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
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
    pub use cio::{Cin, Cout};
    pub use itertools::{Itertools, Product, Unique, UniqueBy};
    pub use mod_arith::{ModUsize, bin_exp};
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
use cp_library::{Cin, Cout, End, Frac, Itertools, frac::F, mod_arith::{self, FactTable}};
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
    let fact = FactTable::new(n);
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
