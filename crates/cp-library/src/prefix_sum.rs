use std::ops::{Bound, RangeBounds};

use crate::algebra::{DefaultMonoid, Monus, Num};

pub struct PrefixSum<G: Monus> {
    data: Box<[G::T]>,
    algebra: G,
}

impl<G: Monus + Clone> Clone for PrefixSum<G>
where
    G::T: Clone,
{
    fn clone(&self) -> Self {
        Self { data: self.data.clone(), algebra: self.algebra.clone() }
    }
}

impl<G: Monus> std::fmt::Debug for PrefixSum<G>
where
    G::T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrefixSum").field("data", &self.data).finish()
    }
}

impl<G: Monus> PrefixSum<G>
where
    G::T: Clone,
{
    pub fn from_vec_with(mut values: Vec<G::T>, algebra: G) -> Self {
        let n = values.len();
        values.resize_with(n + 1, || algebra.empty());
        values.rotate_right(1);
        values.shrink_to_fit();
        for i in 1..=n {
            values[i] = algebra.append(values[i - 1].clone(), values[i].clone());
        }
        Self { data: values.into_boxed_slice(), algebra }
    }

    pub fn from_iter_with<I: IntoIterator<Item = G::T>>(iter: I, algebra: G) -> Self {
        Self::from_vec_with(iter.into_iter().collect(), algebra)
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
            Bound::Included(&end) => end.checked_add(1).expect("prefix sum range end overflow"),
            Bound::Excluded(&end) => end,
            Bound::Unbounded => self.data.len() - 1,
        };
        if start > end {
            return self.algebra.empty();
        }
        self.query_bounds(start, end)
    }

    pub fn query_bounds(&self, start: usize, end: usize) -> G::T {
        self.algebra.monus(self.data[end].clone(), self.data[start].clone())
    }
}

impl<T> PrefixSum<DefaultMonoid<T>>
where
    T: Num + Clone,
{
    pub fn from_vec(values: Vec<T>) -> Self {
        Self::from_vec_with(values, DefaultMonoid::new())
    }

    pub fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from_iter_with(iter, DefaultMonoid::new())
    }
}

impl<T> FromIterator<T> for PrefixSum<DefaultMonoid<T>>
where
    T: Num + Clone,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from_iter_with(iter, DefaultMonoid::new())
    }
}
