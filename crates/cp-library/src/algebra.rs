use std::{
    cmp,
    marker::PhantomData,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

pub trait Ordering {
    type T;

    fn compare(&self, x: Self::T, y: Self::T) -> cmp::Ordering;
}

pub trait AddInv {
    fn add_inv(self) -> Self;
}

pub trait MulInv {
    fn mul_inv(self) -> Self;
}

pub trait Monoid {
    type T;

    fn empty(&self) -> Self::T;
    fn append(&self, x: Self::T, y: Self::T) -> Self::T;
}

pub trait Monus: Monoid {
    fn monus(&self, x: Self::T, y: Self::T) -> Self::T;
}

pub trait Group: Monoid {
    fn inverse(&self, x: Self::T) -> Self::T;
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

impl_zero_one!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64,);

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
        assert_zero_one!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64,);
    }
}

pub trait Num: PartialEq + Zero + One + NumOps {}

impl<T> Num for T where T: PartialEq + Zero + One + NumOps {}

#[derive(Clone, Copy)]
pub struct DefaultMonoid<T>(PhantomData<T>);

impl<T> Default for DefaultMonoid<T> {
    fn default() -> Self {
        DefaultMonoid::new()
    }
}

impl<T> DefaultMonoid<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: Num> Monoid for DefaultMonoid<T> {
    type T = T;

    fn empty(&self) -> Self::T {
        T::ZERO
    }

    fn append(&self, x: T, y: T) -> T {
        x + y
    }
}

impl<T: Num> Monus for DefaultMonoid<T> {
    fn monus(&self, x: Self::T, y: Self::T) -> Self::T {
        x - y
    }
}

impl<T: Num + Neg<Output = T>> AddInv for T {
    fn add_inv(self) -> T {
        -self
    }
}

#[derive(Clone, Copy)]
pub struct MulMonoid<T>(PhantomData<T>);

impl<T> MulMonoid<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Default for MulMonoid<T> {
    fn default() -> Self {
        MulMonoid::new()
    }
}

impl<T: Num> Monoid for MulMonoid<T> {
    type T = T;

    fn empty(&self) -> Self::T {
        T::ONE
    }

    fn append(&self, x: T, y: T) -> T {
        x * y
    }
}

impl<T: Num + AddInv> Group for DefaultMonoid<T> {
    fn inverse(&self, x: Self::T) -> Self::T {
        x.add_inv()
    }
}

impl<T: Num + MulInv> Group for MulMonoid<T> {
    fn inverse(&self, x: Self::T) -> Self::T {
        x.mul_inv()
    }
}

#[derive(Clone, Copy)]
pub struct DefaultOrdering<T>(PhantomData<T>);

impl<T> Default for DefaultOrdering<T> {
    fn default() -> Self {
        DefaultOrdering::new()
    }
}

impl<T> DefaultOrdering<T> {
    pub fn new() -> Self {
        DefaultOrdering(PhantomData)
    }
}

impl<T: Ord> Ordering for DefaultOrdering<T> {
    type T = T;

    fn compare(&self, x: Self::T, y: Self::T) -> cmp::Ordering {
        x.cmp(&y)
    }
}

#[derive(Clone, Copy, Default)]
pub struct ReversedOrdering<O>(O);

impl<O> ReversedOrdering<O> {
    pub fn new(o: O) -> Self {
        Self(o)
    }
}

impl<O: Ordering> Ordering for ReversedOrdering<O> {
    type T = O::T;

    fn compare(&self, x: Self::T, y: Self::T) -> cmp::Ordering {
        self.0.compare(x, y).reverse()
    }
}

pub struct FnOrdering<T, F>(PhantomData<T>, F);

impl<T, F> FnOrdering<T, F> {
    pub fn new(f: F) -> Self {
        FnOrdering(PhantomData, f)
    }
}

impl<T, F> Ordering for FnOrdering<T, F>
where
    F: Fn(T, T) -> cmp::Ordering,
{
    type T = T;

    fn compare(&self, x: Self::T, y: Self::T) -> cmp::Ordering {
        (self.1)(x, y)
    }
}
