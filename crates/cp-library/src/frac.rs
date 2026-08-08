use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::num::ParseIntError;
use std::ops::{Add, AddAssign, BitXor, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
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
        self.num as u128 * rhs.denom as u128 == rhs.num as u128 * self.denom as u128
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
        (self.num as u128 * rhs.denom as u128).cmp(&(rhs.num as u128 * self.denom as u128))
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
                formatter.write_str("expected an integer or a fraction `num/denom`")
            }
            Self::ZeroDenominator => formatter.write_str("fraction denominator must be nonzero"),
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
            checked_mul(self.num / numerator_common, rhs.denom / denominator_common),
            checked_mul(self.denom / denominator_common, rhs.num / numerator_common),
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
        impl $trait<usize> for Frac {
            type Output = Self;

            fn $method(self, rhs: usize) -> Self::Output {
                self.$method(Self::from(rhs))
            }
        }

        impl $trait<Frac> for usize {
            type Output = Frac;

            fn $method(self, rhs: Frac) -> Self::Output {
                Frac::from(self).$method(rhs)
            }
        }
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
    left.checked_add(right)
        .expect("fraction arithmetic overflow")
}

fn checked_mul(left: usize, right: usize) -> usize {
    left.checked_mul(right)
        .expect("fraction arithmetic overflow")
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{Frac, ParseFracError};

    #[test]
    fn constructs_and_reduces_fractions() {
        assert_eq!(Frac::new(6, 8), Frac { num: 3, denom: 4 });
        assert_eq!(Frac::new(0, 99), Frac::ZERO);
        assert_eq!(Frac::default(), Frac::ZERO);
    }

    #[test]
    fn compares_equivalent_and_distinct_fractions() {
        assert_eq!(Frac { num: 1, denom: 2 }, Frac { num: 2, denom: 4 });
        assert!(Frac::new(2, 3) > Frac::new(3, 5));
        assert!(Frac::new(7, 4) > Frac::from(1));

        let set = HashSet::from([Frac { num: 1, denom: 2 }]);
        assert!(set.contains(&Frac { num: 2, denom: 4 }));
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
        assert_eq!("6/8".parse::<Frac>().unwrap(), Frac::new(3, 4));
        assert_eq!("12".parse::<Frac>().unwrap(), Frac::from(12));
        assert_eq!("1/0".parse::<Frac>(), Err(ParseFracError::ZeroDenominator));
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
