use std::ops::{Bound, RangeBounds};

use crate::algebra::Group;

pub struct PrefixSum<G: Group> {
    data: Box<[G::T]>,
}

impl<G: Group> Clone for PrefixSum<G>
where
    G::T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<G: Group> std::fmt::Debug for PrefixSum<G>
where
    G::T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrefixSum")
            .field("data", &self.data)
            .finish()
    }
}

impl<G: Group> PrefixSum<G>
where
    G::T: Clone,
{
    pub fn from_vec(mut values: Vec<G::T>) -> Self {
        let n = values.len();
        values.resize_with(n + 1, || G::EMPTY);
        values.rotate_right(1);
        values.shrink_to_fit();
        for i in 1..=n {
            values[i] = G::append(values[i - 1].clone(), values[i].clone());
        }
        Self {
            data: values.into_boxed_slice(),
        }
    }

    pub fn get(&self, index: usize) -> G::T {
        assert!(index < self.data.len(), "prefix sum index out of bounds");
        self.data[index].clone()
    }

    pub fn query<R: RangeBounds<usize>>(&self, range: R) -> G::T {
        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start
                .checked_add(1)
                .expect("prefix sum range start overflow"),
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&end) => end.checked_add(1).expect("prefix sum range end overflow"),
            Bound::Excluded(&end) => end,
            Bound::Unbounded => self.data.len() - 1,
        };
        if start > end {
            return G::EMPTY;
        }
        self.query_bounds(start, end)
    }

    pub fn query_bounds(&self, start: usize, end: usize) -> G::T {
        G::append(self.data[end].clone(), G::inverse(self.data[start].clone()))
    }
}

impl<G: Group> FromIterator<G::T> for PrefixSum<G>
where
    G::T: Clone,
{
    fn from_iter<I: IntoIterator<Item = G::T>>(iter: I) -> Self {
        Self::from_vec(iter.into_iter().collect())
    }
}
