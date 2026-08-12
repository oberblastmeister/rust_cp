use std::ops::{Bound, RangeBounds};

use crate::algebra::Monoid;

pub struct SegTree<M: Monoid> {
    data: Box<[M::T]>,
}

impl<M: Monoid> Clone for SegTree<M>
where
    M::T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<M: Monoid> std::fmt::Debug for SegTree<M>
where
    M::T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SegTree").field("data", &self.data).finish()
    }
}

impl<M: Monoid> SegTree<M>
where
    M::T: Clone,
{
    pub fn new(n: usize) -> Self {
        Self {
            data: vec![M::EMPTY; 2 * n].into_boxed_slice(),
        }
    }

    pub fn from_vec(mut values: Vec<M::T>) -> Self {
        let n = values.len();
        values.resize_with(2 * n, || M::EMPTY);
        values.rotate_right(n);
        values.shrink_to_fit();

        let mut tree = Self {
            data: values.into_boxed_slice(),
        };
        tree.build();
        tree
    }

    pub fn len(&self) -> usize {
        self.data.len() / 2
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn build(&mut self) {
        for i in (1..self.len()).rev() {
            self.data[i] = M::append(self.data[i << 1].clone(), self.data[i << 1 | 1].clone());
        }
    }

    pub fn set(&mut self, index: usize, value: M::T) {
        assert!(index < self.len(), "segment tree index out of bounds");

        let mut i = index + self.len();
        self.data[i] = value;
        while i > 1 {
            self.data[i >> 1] = M::append(self.data[i & !1].clone(), self.data[i | 1].clone());
            i >>= 1;
        }
    }

    pub fn get(&self, index: usize) -> M::T {
        assert!(index < self.len(), "segment tree index out of bounds");
        self.data[self.len() + index].clone()
    }

    /// Returns the monoid product over `range`.
    pub fn query<R: RangeBounds<usize>>(&self, range: R) -> M::T {
        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start
                .checked_add(1)
                .expect("segment tree range start overflow"),
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&end) => end.checked_add(1).expect("segment tree range end overflow"),
            Bound::Excluded(&end) => end,
            Bound::Unbounded => self.len(),
        };

        self.query_bounds(start, end)
    }

    fn query_bounds(&self, start: usize, end: usize) -> M::T {
        assert!(start <= end, "invalid segment tree range");
        assert!(end <= self.len(), "segment tree range out of bounds");

        let n = self.len();
        let mut left = start + n;
        let mut right = end + n;
        let mut result_left = M::EMPTY;
        let mut result_right = M::EMPTY;

        while left < right {
            if left & 1 == 1 {
                result_left = M::append(result_left, self.data[left].clone());
                left += 1;
            }
            if right & 1 == 1 {
                right -= 1;
                result_right = M::append(self.data[right].clone(), result_right);
            }
            left >>= 1;
            right >>= 1;
        }

        M::append(result_left, result_right)
    }
}

impl<M: Monoid> std::ops::Index<usize> for SegTree<M> {
    type Output = M::T;

    fn index(&self, index: usize) -> &Self::Output {
        let n = self.data.len() / 2;
        assert!(index < n, "segment tree index out of bounds");
        &self.data[n + index]
    }
}

impl<M: Monoid> FromIterator<M::T> for SegTree<M>
where
    M::T: Clone,
{
    fn from_iter<I: IntoIterator<Item = M::T>>(iter: I) -> Self {
        Self::from_vec(iter.into_iter().collect())
    }
}
