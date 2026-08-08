//! Contest-focused iterator extensions based on `rust-itertools` 0.15.0.
//!
//! The Cartesian product and uniqueness adapters are adapted from
//! <https://github.com/rust-itertools/itertools>. They are
//! modified to form a small, dependency-free subset suitable for bundling.
//! See `third-party/itertools-LICENSE-MIT` for the upstream license.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt::{Display, Write as _};
use std::hash::Hash;
use std::iter::FusedIterator;

/// Dependency-free iterator helpers for common competitive-programming tasks.
pub trait Itertools: Iterator + Sized {
    fn collect_vec(self) -> Vec<Self::Item> {
        self.collect()
    }

    fn sorted(self) -> std::vec::IntoIter<Self::Item>
    where
        Self::Item: Ord,
    {
        let mut values = self.collect_vec();
        values.sort();
        values.into_iter()
    }

    fn sorted_by<F>(self, compare: F) -> std::vec::IntoIter<Self::Item>
    where
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        let mut values = self.collect_vec();
        values.sort_by(compare);
        values.into_iter()
    }

    fn sorted_by_key<K, F>(self, key: F) -> std::vec::IntoIter<Self::Item>
    where
        K: Ord,
        F: FnMut(&Self::Item) -> K,
    {
        let mut values = self.collect_vec();
        values.sort_by_key(key);
        values.into_iter()
    }

    fn join(self, separator: &str) -> String
    where
        Self::Item: Display,
    {
        let mut output = String::new();
        for (index, value) in self.enumerate() {
            if index > 0 {
                output.push_str(separator);
            }
            write!(&mut output, "{value}").expect("writing to a String cannot fail");
        }
        output
    }

    fn unique(self) -> Unique<Self>
    where
        Self::Item: Eq + Hash + Clone,
    {
        Unique {
            iterator: self,
            used: HashMap::new(),
        }
    }

    fn unique_by<V, F>(self, key: F) -> UniqueBy<Self, V, F>
    where
        V: Eq + Hash,
        F: FnMut(&Self::Item) -> V,
    {
        UniqueBy {
            iterator: self,
            used: HashMap::new(),
            key,
        }
    }

    fn counts(self) -> HashMap<Self::Item, usize>
    where
        Self::Item: Eq + Hash,
    {
        self.counts_by(|value| value)
    }

    fn counts_by<K, F>(self, mut key: F) -> HashMap<K, usize>
    where
        K: Eq + Hash,
        F: FnMut(Self::Item) -> K,
    {
        let mut counts = HashMap::new();
        for value in self {
            *counts.entry(key(value)).or_insert(0) += 1;
        }
        counts
    }

    fn cartesian_product<J>(self, other: J) -> Product<Self, J::IntoIter>
    where
        Self::Item: Clone,
        J: IntoIterator,
        J::IntoIter: Clone,
    {
        Product::new(self, other.into_iter())
    }

    fn all_equal(mut self) -> bool
    where
        Self::Item: PartialEq,
    {
        let Some(first) = self.next() else {
            return true;
        };
        self.all(|value| value == first)
    }
}

impl<I: Iterator> Itertools for I {}

/// A lazy adapter that retains the first item for each distinct value.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Unique<I: Iterator>
where
    I::Item: Eq + Hash + Clone,
{
    iterator: I,
    used: HashMap<I::Item, ()>,
}

impl<I> Iterator for Unique<I>
where
    I: Iterator,
    I::Item: Eq + Hash + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.by_ref().find_map(|value| {
            if let Entry::Vacant(entry) = self.used.entry(value) {
                let result = entry.key().clone();
                entry.insert(());
                Some(result)
            } else {
                None
            }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iterator.size_hint();
        (usize::from(lower > 0 && self.used.is_empty()), upper)
    }
}

impl<I> FusedIterator for Unique<I>
where
    I: FusedIterator,
    I::Item: Eq + Hash + Clone,
{
}

/// A lazy adapter that retains the first item for each distinct key.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct UniqueBy<I: Iterator, V, F> {
    iterator: I,
    used: HashMap<V, ()>,
    key: F,
}

impl<I, V, F> Iterator for UniqueBy<I, V, F>
where
    I: Iterator,
    V: Eq + Hash,
    F: FnMut(&I::Item) -> V,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            iterator,
            used,
            key,
        } = self;
        iterator.find(|value| used.insert(key(value), ()).is_none())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iterator.size_hint();
        (usize::from(lower > 0 && self.used.is_empty()), upper)
    }
}

impl<I, V, F> FusedIterator for UniqueBy<I, V, F>
where
    I: FusedIterator,
    V: Eq + Hash,
    F: FnMut(&I::Item) -> V,
{
}

/// A lazy iterator over the Cartesian product of two iterators.
///
/// This follows upstream `itertools` ordering: the right iterator advances
/// fastest, producing `(a0, b0), (a0, b1), (a1, b0), ...`.
#[derive(Clone, Debug)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Product<I: Iterator, J: Iterator> {
    left: I,
    current_left: Option<Option<I::Item>>,
    right: J,
    original_right: J,
}

impl<I, J> Product<I, J>
where
    I: Iterator,
    J: Iterator + Clone,
    I::Item: Clone,
{
    fn new(left: I, right: J) -> Self {
        Self {
            left,
            current_left: None,
            right: right.clone(),
            original_right: right,
        }
    }
}

impl<I, J> Iterator for Product<I, J>
where
    I: Iterator,
    J: Iterator + Clone,
    I::Item: Clone,
{
    type Item = (I::Item, J::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let right = match self.right.next() {
            Some(value) => value,
            None => {
                self.right = self.original_right.clone();
                let value = self.right.next()?;
                self.current_left = Some(self.left.next());
                value
            }
        };
        self.current_left
            .get_or_insert_with(|| self.left.next())
            .as_ref()
            .map(|left| (left.clone(), right))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (left_lower, left_upper) = self.left.size_hint();
        let (right_lower, right_upper) = self.original_right.size_hint();
        let mut lower = left_lower.saturating_mul(right_lower);
        let mut upper = left_upper
            .zip(right_upper)
            .and_then(|(left, right)| left.checked_mul(right));
        if matches!(self.current_left, Some(Some(_))) {
            let (remaining_lower, remaining_upper) = self.right.size_hint();
            lower = lower.saturating_add(remaining_lower);
            upper = upper
                .zip(remaining_upper)
                .and_then(|(total, remaining)| total.checked_add(remaining));
        }
        (lower, upper)
    }
}

impl<I, J> FusedIterator for Product<I, J>
where
    I: FusedIterator,
    J: FusedIterator + Clone,
    I::Item: Clone,
{
}
