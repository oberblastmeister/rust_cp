pub mod prefix_sum;
pub mod algebra;
pub mod dsu;
pub mod frac;
pub mod io;
pub mod itertools;
pub mod mod_arith;
pub mod seg_tree;

pub use dsu::Dsu;
pub use frac::{Frac, ParseFracError};
pub use io::{Cin, Cout};
pub use prefix_sum::PrefixSum;
pub use itertools::{Itertools, Product, Unique, UniqueBy};
pub use mod_arith::{ModUsize, bin_exp};
pub use seg_tree::SegTree;

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

#[cfg(test)]
mod end_index_tests {
    use super::End;

    #[test]
    fn indexes_final_unicode_character_in_strings() {
        let string = String::from("helloé");
        let string_slice: &str = &string;

        assert_eq!(&string[End], "é");
        assert_eq!(&string_slice[End], "é");
    }

    #[test]
    fn mutably_indexes_final_character_in_strings() {
        let mut string = String::from("helloa");
        string[End].make_ascii_uppercase();
        assert_eq!(string, "helloA");

        let mut string = String::from("worldb");
        let string_slice: &mut str = string.as_mut_str();
        string_slice[End].make_ascii_uppercase();
        assert_eq!(string, "worldB");
    }
}
