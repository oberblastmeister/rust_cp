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
    pub use io::{Cin, Cout};
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
use std::iter;
use cp_library::{Cin, Cout, End, ModUsize, mod_arith::FactTable};
const MOD: usize = 998244353;
type musize = ModUsize<MOD>;
fn M(x: usize) -> musize {
    musize::new(x)
}
fn solve(n: usize, a: Vec<usize>) -> musize {
    let &largest = a[1..].iter().max().unwrap();
    let necessary = a[1..]
        .iter()
        .copied()
        .map(|x| (largest - x).saturating_sub(1))
        .sum::<usize>();
    if necessary > a[0] {
        return M(0);
    }
    let fact = FactTable::new(n);
    let left = a[0] - necessary;
    let num_at_max = a[1..].iter().filter(|&&x| x == largest).count();
    let mut res = M(0);
    for k in 0..=left.min(n - num_at_max) {
        let curr = num_at_max * fact.choose(n - num_at_max, k) * fact[num_at_max - 1 + k]
            * fact[n - num_at_max - k];
        res += curr;
    }
    res
}
fn main() {
    let mut cin = Cin::new();
    let mut cout = Cout::new();
    let t: usize = cin.read();
    for _ in 0..t {
        let n = cin.read();
        let a = cin.read_vec(n + 1);
        let res = solve(n, a);
        cout.println(res);
    }
}
