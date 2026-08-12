//! An ordered multiset with an API modeled after [`std::collections::BTreeSet`].
//!
//! Iteration yields every occurrence in ascending order.

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::{Chain, FlatMap, FusedIterator, Once};
use std::ops::RangeBounds;
use std::slice;
use std::vec;

/// An ordered multiset based on a B-tree.
///
/// Equal values may occur more than once. All iterators visit every occurrence
/// in ascending order.
pub struct MultiSet<T> {
    // The key is the representative occurrence. The vector owns all additional
    // occurrences, allowing ownership-returning methods to avoid a `Clone` bound.
    map: BTreeMap<T, Vec<T>>,
    len: usize,
}

type EntryIter<'a, T> = Chain<Once<&'a T>, slice::Iter<'a, T>>;
type MapIter<'a, T> = std::collections::btree_map::Iter<'a, T, Vec<T>>;
type MapRange<'a, T> = std::collections::btree_map::Range<'a, T, Vec<T>>;
type FlatIter<'a, T, I> = FlatMap<I, EntryIter<'a, T>, fn((&'a T, &'a Vec<T>)) -> EntryIter<'a, T>>;

fn entry_iter<'a, T>((value, duplicates): (&'a T, &'a Vec<T>)) -> EntryIter<'a, T> {
    std::iter::once(value).chain(duplicates.iter())
}

/// An iterator over the values of a [`MultiSet`].
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct Iter<'a, T> {
    inner: FlatIter<'a, T, MapIter<'a, T>>,
    remaining: usize,
}

/// An iterator over a sub-range of values in a [`MultiSet`].
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Range<'a, T> {
    inner: FlatIter<'a, T, MapRange<'a, T>>,
    remaining: usize,
}

/// An owning iterator over the values of a [`MultiSet`].
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct IntoIter<T> {
    inner: vec::IntoIter<T>,
}

impl<T> MultiSet<T> {
    /// Makes a new, empty multiset.
    pub const fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            len: 0,
        }
    }

    /// Returns the number of values, including duplicate occurrences.
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the multiset contains no values.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Gets an iterator that visits every occurrence in ascending order.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            inner: self.map.iter().flat_map(entry_iter::<T>),
            remaining: self.len,
        }
    }

    /// Clears the multiset, removing all values.
    pub fn clear(&mut self) {
        self.map.clear();
        self.len = 0;
    }
}

impl<T: Ord> MultiSet<T> {
    /// Constructs a double-ended iterator over a sub-range of values.
    pub fn range<'a, K, R>(&'a self, range: R) -> Range<'a, T>
    where
        K: Ord + ?Sized,
        T: Borrow<K>,
        R: RangeBounds<K>,
    {
        let inner = self
            .map
            .range(range)
            .flat_map(entry_iter::<T> as fn((&'a T, &'a Vec<T>)) -> EntryIter<'a, T>);
        let remaining = inner.clone().count();
        Range { inner, remaining }
    }

    /// Returns `true` if the multisets have no value in common.
    pub fn is_disjoint(&self, other: &Self) -> bool {
        let (small, large) = if self.map.len() <= other.map.len() {
            (self, other)
        } else {
            (other, self)
        };
        small.map.keys().all(|value| !large.map.contains_key(value))
    }

    /// Returns `true` if every value occurs at least as often in `other`.
    pub fn is_subset(&self, other: &Self) -> bool {
        self.map.iter().all(|(value, duplicates)| {
            other
                .map
                .get(value)
                .is_some_and(|other_duplicates| duplicates.len() <= other_duplicates.len())
        })
    }

    /// Returns `true` if every value in `other` occurs at least as often in `self`.
    pub fn is_superset(&self, other: &Self) -> bool {
        other.is_subset(self)
    }

    /// Returns `true` if at least one equal value is present.
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.map.contains_key(value)
    }

    /// Returns the number of occurrences equal to `value`.
    pub fn count<Q>(&self, value: &Q) -> usize
    where
        T: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.map
            .get(value)
            .map_or(0, |duplicates| duplicates.len() + 1)
    }

    /// Returns a reference to an equal value, if present.
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.map.get_key_value(value).map(|(value, _)| value)
    }

    /// Returns the first (minimum) value, if any.
    pub fn first(&self) -> Option<&T> {
        self.map.first_key_value().map(|(value, _)| value)
    }

    /// Returns the last (maximum) value, if any.
    pub fn last(&self) -> Option<&T> {
        self.map.last_key_value().map(|(value, _)| value)
    }

    /// Removes and returns one minimum occurrence, if any.
    pub fn pop_first(&mut self) -> Option<T> {
        let value = match self.map.first_entry()? {
            mut entry if !entry.get().is_empty() => entry.get_mut().pop().unwrap(),
            entry => entry.remove_entry().0,
        };
        self.len -= 1;
        Some(value)
    }

    /// Removes and returns one maximum occurrence, if any.
    pub fn pop_last(&mut self) -> Option<T> {
        let value = match self.map.last_entry()? {
            mut entry if !entry.get().is_empty() => entry.get_mut().pop().unwrap(),
            entry => entry.remove_entry().0,
        };
        self.len -= 1;
        Some(value)
    }

    /// Adds one occurrence. Returns whether its equivalence class was new.
    pub fn insert(&mut self, value: T) -> bool {
        self.len += 1;
        if let Some(duplicates) = self.map.get_mut(&value) {
            duplicates.push(value);
            false
        } else {
            self.map.insert(value, Vec::new());
            true
        }
    }

    /// Replaces the representative equal value, preserving its multiplicity.
    pub fn replace(&mut self, value: T) -> Option<T> {
        if let Some((old, duplicates)) = self.map.remove_entry(&value) {
            self.map.insert(value, duplicates);
            Some(old)
        } else {
            self.map.insert(value, Vec::new());
            self.len += 1;
            None
        }
    }

    /// Removes one equal occurrence. Returns whether one was present.
    pub fn remove<Q>(&mut self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let Some(entry) = self.map.get_mut(value) else {
            return false;
        };
        if entry.pop().is_none() {
            self.map.remove(value);
        }
        self.len -= 1;
        true
    }

    /// Removes and returns one equal occurrence, if present.
    pub fn take<Q>(&mut self, value: &Q) -> Option<T>
    where
        T: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let result = if let Some(duplicates) = self.map.get_mut(value) {
            duplicates.pop()
        } else {
            return None;
        };
        let result = result.or_else(|| self.map.remove_entry(value).map(|(value, _)| value));
        self.len -= 1;
        result
    }

    /// Retains only occurrences for which the predicate returns `true`.
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        let old = std::mem::take(&mut self.map);
        self.len = 0;
        for (value, duplicates) in old {
            for value in std::iter::once(value).chain(duplicates) {
                if f(&value) {
                    self.insert(value);
                }
            }
        }
    }

    /// Moves all occurrences from `other` into `self`, leaving it empty.
    pub fn append(&mut self, other: &mut Self) {
        let map = std::mem::take(&mut other.map);
        self.len += other.len;
        other.len = 0;
        for (value, mut duplicates) in map {
            if let Some(existing) = self.map.get_mut(&value) {
                existing.push(value);
                existing.append(&mut duplicates);
            } else {
                self.map.insert(value, duplicates);
            }
        }
    }

    /// Splits off all occurrences greater than or equal to `value`.
    pub fn split_off<Q>(&mut self, value: &Q) -> Self
    where
        Q: Ord + ?Sized,
        T: Borrow<Q>,
    {
        let map = self.map.split_off(value);
        let len = map.values().map(|duplicates| duplicates.len() + 1).sum();
        self.len -= len;
        Self { map, len }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.inner.next();
        self.remaining -= usize::from(value.is_some());
        value
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T> DoubleEndedIterator for Iter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let value = self.inner.next_back();
        self.remaining -= usize::from(value.is_some());
        value
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {}
impl<T> FusedIterator for Iter<'_, T> {}

impl<'a, T> Iterator for Range<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.inner.next();
        self.remaining -= usize::from(value.is_some());
        value
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T> DoubleEndedIterator for Range<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let value = self.inner.next_back();
        self.remaining -= usize::from(value.is_some());
        value
    }
}

impl<T> ExactSizeIterator for Range<'_, T> {}
impl<T> FusedIterator for Range<'_, T> {}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.inner.next_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}
impl<T> FusedIterator for IntoIter<T> {}

impl<T> Default for MultiSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for MultiSet<T> {
    fn clone(&self) -> Self {
        Self {
            map: self.map.clone(),
            len: self.len,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.map.clone_from(&source.map);
        self.len = source.len;
    }
}

impl<T: fmt::Debug> fmt::Debug for MultiSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<T: Ord> PartialEq for MultiSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.iter().eq(other.iter())
    }
}

impl<T: Ord> Eq for MultiSet<T> {}

impl<T: Ord> PartialOrd for MultiSet<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Ord for MultiSet<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other.iter())
    }
}

impl<T: Hash> Hash for MultiSet<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len.hash(state);
        for value in self.iter() {
            value.hash(state);
        }
    }
}

impl<T: Ord> Extend<T> for MultiSet<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for value in iter {
            self.insert(value);
        }
    }
}

impl<'a, T: 'a + Ord + Copy> Extend<&'a T> for MultiSet<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().copied());
    }
}

impl<T: Ord> FromIterator<T> for MultiSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::new();
        set.extend(iter);
        set
    }
}

impl<T: Ord, const N: usize> From<[T; N]> for MultiSet<T> {
    fn from(values: [T; N]) -> Self {
        values.into_iter().collect()
    }
}

impl<'a, T> IntoIterator for &'a MultiSet<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> IntoIterator for MultiSet<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let mut values = Vec::with_capacity(self.len);
        for (value, duplicates) in self.map {
            values.push(value);
            values.extend(duplicates);
        }
        IntoIter {
            inner: values.into_iter(),
        }
    }
}
