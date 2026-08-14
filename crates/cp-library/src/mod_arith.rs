use std::fmt::{self, Display, Formatter};
use std::ops::{Add, AddAssign, BitXor, Div, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign};
use std::str::FromStr;

use crate::algebra::MulInv;

/// Computes `base.pow(exponent) mod modulus` with binary exponentiation.
pub fn bin_exp(mut base: usize, mut exponent: usize, modulus: usize) -> usize {
    assert!(modulus > 0, "modulus must be positive");
    base %= modulus;
    let mut answer = 1 % modulus;

    while exponent > 0 {
        if exponent % 2 == 1 {
            answer = ((answer as u128 * base as u128) % modulus as u128) as usize;
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
        if self.v >= rhs.v { Self::new(self.v - rhs.v) } else { Self::new(MOD - (rhs.v - self.v)) }
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
