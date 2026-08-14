pub mod algebra;
pub mod binary_search;
pub mod cartesian_tree;
pub mod cio;
pub mod driver;
pub mod dsu;
pub mod frac;
pub mod grid;
pub mod io;
pub mod itertools;
pub mod mod_arith;
pub mod multiset;
pub mod prefix_sum;
pub mod segtree;
pub mod sparse_table;

pub use cio::{Cin, Cout};
pub use driver::{TestKind, driver, test_driver};
pub use itertools::Itertools;

pub mod prelude {
    pub use crate::{Cin, Cout, End, Itertools, TestKind, driver, test_driver};
    pub use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
    pub use std::iter;
}

pub struct End;

impl<T> std::ops::Index<End> for Vec<T> {
    type Output = T;

    fn index(&self, _: End) -> &Self::Output {
        self.last().expect("cannot index the end of an empty vector")
    }
}

impl<T> std::ops::IndexMut<End> for Vec<T> {
    fn index_mut(&mut self, _: End) -> &mut Self::Output {
        self.last_mut().expect("cannot index the end of an empty vector")
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
        self.last_mut().expect("cannot index the end of an empty slice")
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
