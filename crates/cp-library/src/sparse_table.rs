use std::ops::{Bound, RangeBounds};

use crate::{algebra::Monoid, grid::Grid};

/// A static range-query data structure.
///
/// Construction takes `O(n log n)` time and memory. [`query`](Self::query)
/// works for any monoid in `O(log n)`, while
/// [`query_idempotent`](Self::query_idempotent) runs in `O(1)` but is only
/// correct when the monoid operation is idempotent (for example, minimum or
/// maximum).
pub struct SparseTable<M: Monoid>(Grid<M::T>);

impl<M: Monoid> SparseTable<M>
where
    M::T: Clone,
{
    pub fn new(values: &[M::T]) -> Self {
        let n = values.len();
        let levels = if n == 0 { 0 } else { n.ilog2() as usize + 1 };
        let mut table = Grid::new(levels, n, M::EMPTY);

        if n == 0 {
            return Self(table);
        }

        table[0].clone_from_slice(values);
        for level in 1..levels {
            let len = 1usize << level;
            let half = len >> 1;
            for start in 0..=n - len {
                table[(level, start)] = M::append(
                    table[(level - 1, start)].clone(),
                    table[(level - 1, start + half)].clone(),
                );
            }
        }

        Self(table)
    }

    pub fn len(&self) -> usize {
        self.0.cols()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> std::slice::Iter<'_, M::T> {
        self.0.get_row(0).unwrap_or(&[]).iter()
    }

    /// Returns the monoid product over `range` in `O(log n)` time.
    pub fn query<R: RangeBounds<usize>>(&self, range: R) -> M::T {
        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => {
                start.checked_add(1).expect("sparse table range start overflow")
            }
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&end) => end.checked_add(1).expect("sparse table range end overflow"),
            Bound::Excluded(&end) => end,
            Bound::Unbounded => self.len(),
        };
        self.query_bounds(start, end)
    }

    /// Returns the monoid product over the half-open range `[start, end)` in
    /// `O(log n)` time.
    pub fn query_bounds(&self, mut start: usize, end: usize) -> M::T {
        assert!(start <= self.len(), "sparse table range out of bounds");
        assert!(end <= self.len(), "sparse table range out of bounds");
        if start > end {
            return M::EMPTY;
        }

        let mut result = M::EMPTY;
        for level in (0..self.0.rows()).rev() {
            let len = 1usize << level;
            if start + len <= end {
                result = M::append(result, self.0[(level, start)].clone());
                start += len;
            }
        }
        result
    }

    /// Returns the product over `range` in `O(1)` time by combining two
    /// overlapping blocks.
    ///
    /// This method is onrowsly correct when `M::append` is idempotent, meaning
    /// `M::append(x, x) == x`.
    pub fn query_idempotent<R: RangeBounds<usize>>(&self, range: R) -> M::T {
        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => {
                start.checked_add(1).expect("sparse table range start overflow")
            }
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&end) => end.checked_add(1).expect("sparse table range end overflow"),
            Bound::Excluded(&end) => end,
            Bound::Unbounded => self.len(),
        };
        self.query_idempotent_bounds(start, end)
    }

    /// Returns the product over `[start, end)` in `O(1)` time by combining two
    /// overlapping blocks.
    ///
    /// This method is only correct when `M::append` is idempotent, meaning
    /// `M::append(x, x) == x`.
    pub fn query_idempotent_bounds(&self, start: usize, end: usize) -> M::T {
        assert!(start <= self.len(), "sparse table range out of bounds");
        assert!(end <= self.len(), "sparse table range out of bounds");
        if start > end {
            return M::EMPTY;
        }

        if start == end {
            return M::EMPTY;
        }

        let level = (end - start).ilog2() as usize;
        let len = 1usize << level;
        M::append(self.0[(level, start)].clone(), self.0[(level, end - len)].clone())
    }
}

impl<M: Monoid> std::ops::Index<usize> for SparseTable<M> {
    type Output = M::T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.0.cols(), "sparse table index out of bounds");
        &self.0[(0, index)]
    }
}

impl<M: Monoid> FromIterator<M::T> for SparseTable<M>
where
    M::T: Clone,
{
    fn from_iter<I: IntoIterator<Item = M::T>>(iter: I) -> Self {
        let values: Vec<_> = iter.into_iter().collect();
        Self::new(&values)
    }
}
