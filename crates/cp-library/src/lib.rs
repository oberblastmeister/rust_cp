pub mod algebra;
pub mod cartesian_tree;
pub mod cio;
pub mod dsu;
pub mod frac;
pub mod io;
pub mod itertools;
pub mod mod_arith;
pub mod prefix_sum;
pub mod seg_tree;
pub mod driver;

pub use cio::{Cin, Cout};
pub use dsu::Dsu;
pub use frac::{Frac, ParseFracError};
pub use itertools::{Itertools, Product, Unique, UniqueBy};
pub use mod_arith::{ModUsize, bin_exp};
pub use prefix_sum::PrefixSum;
pub use seg_tree::SegTree;
pub use driver::{driver, test_driver, TestKind};


// Works exactly like partition_point in rust std but operates in a "virtual" array
pub fn virtual_partition_point<F>(start: usize, end: usize, mut f: F) -> usize
where
    F: FnMut(usize) -> bool,
{
    assert!(start <= end);
    let mut lo = start;
    let mut hi = end;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if f(mid) {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

pub struct End;

impl<T> std::ops::Index<End> for Vec<T> {
    type Output = T;

    fn index(&self, _: End) -> &Self::Output {
        self.last()
            .expect("cannot index the end of an empty vector")
    }
}

impl<T> std::ops::IndexMut<End> for Vec<T> {
    fn index_mut(&mut self, _: End) -> &mut Self::Output {
        self.last_mut()
            .expect("cannot index the end of an empty vector")
    }
}

impl<T> std::ops::Index<End> for [T] {
    type Output = T;

    fn index(&self, _: End) -> &Self::Output {
        self.last().expect("cannot index the end of an empty slice")
    }
}

impl<T> std::ops::IndexMut<End> for [T] {
    fn index_mut(&mut self, _: End) -> &mut Self::Output {
        self.last_mut()
            .expect("cannot index the end of an empty slice")
    }
}

impl std::ops::Index<End> for str {
    type Output = str;

    fn index(&self, _: End) -> &Self::Output {
        let start = self
            .char_indices()
            .next_back()
            .map(|(index, _)| index)
            .expect("cannot index the end of an empty string slice");
        &self[start..]
    }
}

impl std::ops::IndexMut<End> for str {
    fn index_mut(&mut self, _: End) -> &mut Self::Output {
        let start = self
            .char_indices()
            .next_back()
            .map(|(index, _)| index)
            .expect("cannot index the end of an empty string slice");
        &mut self[start..]
    }
}

impl std::ops::Index<End> for String {
    type Output = str;

    fn index(&self, index: End) -> &Self::Output {
        <str as std::ops::Index<End>>::index(self.as_str(), index)
    }
}

impl std::ops::IndexMut<End> for String {
    fn index_mut(&mut self, index: End) -> &mut Self::Output {
        <str as std::ops::IndexMut<End>>::index_mut(self.as_mut_str(), index)
    }
}
