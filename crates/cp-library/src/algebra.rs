use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

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

pub trait NumOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
    + Rem<Rhs, Output = Output>
{
}

impl<T, Rhs, Output> NumOps<Rhs, Output> for T where
    T: Add<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + Mul<Rhs, Output = Output>
        + Div<Rhs, Output = Output>
        + Rem<Rhs, Output = Output>
{
}

pub trait Zero: Sized {
    const ZERO: Self;
}

pub trait One: Sized {
    const ONE: Self;
}

macro_rules! impl_zero_one {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl Zero for $ty {
                const ZERO: Self = 0 as Self;
            }

            impl One for $ty {
                const ONE: Self = 1 as Self;
            }
        )+
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
            $(
                assert_eq!(<$ty as Zero>::ZERO, 0 as $ty);
                assert_eq!(<$ty as One>::ONE, 1 as $ty);
            )+
        };
    }

    #[test]
    fn primitive_numbers_have_zero_and_one() {
        assert_zero_one!(
            i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64,
        );
    }
}

pub trait Num: PartialEq + Zero + One + NumOps {}

impl<T> Num for T where T: PartialEq + Zero + One + NumOps {}

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
