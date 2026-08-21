use std::ops::{Bound, RangeBounds};

use crate::algebra::{DefaultMonoid, Monoid, Num};

pub struct SegTree<M: Monoid> {
    data: Box<[M::T]>,
    algebra: M,
}

pub type DefaultSegTree<T> = SegTree<DefaultMonoid<T>>;

impl<M: Monoid + Clone> Clone for SegTree<M>
where
    M::T: Clone,
{
    fn clone(&self) -> Self {
        Self { data: self.data.clone(), algebra: self.algebra.clone() }
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
    pub fn new_with(n: usize, algebra: M) -> Self {
        let data = vec![algebra.empty(); 2 * n].into_boxed_slice();
        Self { data, algebra }
    }

    pub fn from_vec_with(mut values: Vec<M::T>, algebra: M) -> Self {
        let n = values.len();
        values.resize_with(2 * n, || algebra.empty());
        values.rotate_right(n);
        values.shrink_to_fit();

        let mut tree = Self { data: values.into_boxed_slice(), algebra };
        tree.build();
        tree
    }

    pub fn from_iter_with<I: IntoIterator<Item = M::T>>(iter: I, algebra: M) -> Self {
        Self::from_vec_with(iter.into_iter().collect(), algebra)
    }

    pub fn len(&self) -> usize {
        self.data.len() / 2
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn build(&mut self) {
        for i in (1..self.len()).rev() {
            self.data[i] =
                self.algebra.append(self.data[i << 1].clone(), self.data[i << 1 | 1].clone());
        }
    }

    pub fn set(&mut self, index: usize, value: M::T) {
        assert!(index < self.len(), "segment tree index out of bounds");

        let mut i = index + self.len();
        self.data[i] = value;
        while i > 1 {
            self.data[i >> 1] =
                self.algebra.append(self.data[i & !1].clone(), self.data[i | 1].clone());
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
            Bound::Excluded(&start) => {
                start.checked_add(1).expect("segment tree range start overflow")
            }
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&end) => end.checked_add(1).expect("segment tree range end overflow"),
            Bound::Excluded(&end) => end,
            Bound::Unbounded => self.len(),
        };

        self.query_bounds(start, end)
    }

    pub fn query_bounds(&self, start: usize, end: usize) -> M::T {
        assert!(start <= end, "invalid segment tree range");
        assert!(end <= self.len(), "segment tree range out of bounds");

        let n = self.len();
        let mut left = start + n;
        let mut right = end + n;
        let mut result_left = self.algebra.empty();
        let mut result_right = self.algebra.empty();

        while left < right {
            if left & 1 == 1 {
                result_left = self.algebra.append(result_left, self.data[left].clone());
                left += 1;
            }
            if right & 1 == 1 {
                right -= 1;
                result_right = self.algebra.append(self.data[right].clone(), result_right);
            }
            left >>= 1;
            right >>= 1;
        }

        self.algebra.append(result_left, result_right)
    }
}

impl<M: Monoid + Default> SegTree<M>
where
    M::T: Clone,
{
    pub fn new(n: usize) -> Self {
        Self::new_with(n, M::default())
    }

    pub fn from_vec(values: Vec<M::T>) -> Self {
        Self::from_vec_with(values, M::default())
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

impl<T> FromIterator<T> for DefaultSegTree<T>
where
    T: Num + Clone,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from_iter_with(iter, DefaultMonoid::new())
    }
}

#[cfg(test)]
mod tests {
    use super::SegTree;
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
        let tree: SegTree<DefaultMonoid<i32>> = [3, 1, 4].into_iter().collect();
        assert_eq!(tree.query(..), 8);
    }

    #[test]
    fn uses_the_supplied_algebra() {
        let mut tree = SegTree::from_iter_with([3, 1, 4], MaxMonoid(i32::MIN));
        assert_eq!(tree.query(..), 4);
        assert_eq!(tree.query(1..2), 1);
        assert_eq!(tree.query(1..1), i32::MIN);

        tree.set(2, 0);
        assert_eq!(tree.query(..), 3);
    }
}
