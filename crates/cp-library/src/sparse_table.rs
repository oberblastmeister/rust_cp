use std::ops::{Bound, RangeBounds};

use crate::{
    algebra::{DefaultMonoid, Monoid, Num},
    grid::Grid,
};

/// A static range-query data structure.
///
/// Construction takes `O(n log n)` time and memory. [`query`](Self::query)
/// works for any monoid in `O(log n)`, while
/// [`query_idempotent`](Self::query_idempotent) runs in `O(1)` but is only
/// correct when the monoid operation is idempotent (for example, minimum or
/// maximum).
pub struct SparseTable<M: Monoid> {
    table: Grid<M::T>,
    algebra: M,
}

impl<M: Monoid> SparseTable<M>
where
    M::T: Clone,
{
    pub fn new_with(values: &[M::T], algebra: M) -> Self {
        let n = values.len();
        let levels = if n == 0 { 0 } else { n.ilog2() as usize + 1 };
        let mut table = Grid::new(levels, n, algebra.empty());

        if n == 0 {
            return Self { table, algebra };
        }

        table[0].clone_from_slice(values);
        for level in 1..levels {
            let len = 1usize << level;
            let half = len >> 1;
            for start in 0..=n - len {
                table[(level, start)] = algebra.append(
                    table[(level - 1, start)].clone(),
                    table[(level - 1, start + half)].clone(),
                );
            }
        }

        Self { table, algebra }
    }

    pub fn from_iter_with<I: IntoIterator<Item = M::T>>(iter: I, algebra: M) -> Self {
        let values: Vec<_> = iter.into_iter().collect();
        Self::new_with(&values, algebra)
    }

    pub fn len(&self) -> usize {
        self.table.cols()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> std::slice::Iter<'_, M::T> {
        self.table.get_row(0).unwrap_or(&[]).iter()
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
            return self.algebra.empty();
        }

        let mut result = self.algebra.empty();
        for level in (0..self.table.rows()).rev() {
            let len = 1usize << level;
            if start + len <= end {
                result = self.algebra.append(result, self.table[(level, start)].clone());
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
            return self.algebra.empty();
        }

        if start == end {
            return self.algebra.empty();
        }

        let level = (end - start).ilog2() as usize;
        let len = 1usize << level;
        self.algebra
            .append(self.table[(level, start)].clone(), self.table[(level, end - len)].clone())
    }
}

impl<M: Monoid + Default> SparseTable<M>
where
    M::T: Clone,
{
    pub fn new(values: &[M::T]) -> Self {
        Self::new_with(values, M::default())
    }

    pub fn from_iter<I: IntoIterator<Item = M::T>>(iter: I) -> Self {
        Self::from_iter_with(iter, M::default())
    }
}

impl<M: Monoid> std::ops::Index<usize> for SparseTable<M> {
    type Output = M::T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.table.cols(), "sparse table index out of bounds");
        &self.table[(0, index)]
    }
}

impl<T> FromIterator<T> for SparseTable<DefaultMonoid<T>>
where
    T: Num + Clone,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from_iter_with(iter, DefaultMonoid::new())
    }
}

#[cfg(test)]
mod tests {
    use super::SparseTable;
    use crate::algebra::{DefaultMonoid, Monoid};

    struct MaxMonoid(i32);

    impl Monoid for MaxMonoid {
        type T = i32;

        fn empty(&self) -> Self::T {
            self.0
        }

        fn append(&self, x: Self::T, y: Self::T) -> Self::T {
            x.max(y)
        }
    }

    #[test]
    fn from_iterator_uses_default_algebra() {
        let table: SparseTable<DefaultMonoid<i32>> = [3, 1, 4].into_iter().collect();
        assert_eq!(table.query(..), 8);
    }

    #[test]
    fn uses_the_supplied_algebra() {
        let table = SparseTable::from_iter_with([3, 1, 4, 2], MaxMonoid(i32::MIN));
        assert_eq!(table.query(..), 4);
        assert_eq!(table.query(1..2), 1);
        assert_eq!(table.query(2..2), i32::MIN);
        assert_eq!(table.query_idempotent(1..4), 4);
    }
}
