use std::ops::{Index, IndexMut};

/// A two-dimensional, row-major array backed by a single [`Vec`].
///
/// `Grid` is a collection. Its dimensions are
/// specified as `(rows, columns)`, and elements are indexed as `(row, column)`.
/// All rows have the same length.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Grid<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Grid<T> {
    /// Creates a `rows` by `cols` array by calling `f` for each position in
    /// row-major order.
    pub fn from_fn<F>(rows: usize, cols: usize, mut f: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        let len = checked_len(rows, cols);
        let mut data = Vec::with_capacity(len);
        for row in 0..rows {
            for col in 0..cols {
                data.push(f(row, col));
            }
        }
        Self { data, rows, cols }
    }

    /// Wraps contiguous row-major storage as a `rows` by `cols` array.
    ///
    /// # Panics
    ///
    /// Panics if `data.len() != rows * cols` or the product overflows.
    pub fn from_vec(rows: usize, cols: usize, data: Vec<T>) -> Self {
        let len = checked_len(rows, cols);
        assert_eq!(data.len(), len, "Vec2d data length does not match its dimensions");
        Self { data, rows, cols }
    }

    /// Creates an array from a list of rows.
    ///
    /// # Panics
    ///
    /// Panics if the rows do not all have the same length.
    pub fn from_rows(rows: Vec<Vec<T>>) -> Self {
        let row_count = rows.len();
        let cols = rows.first().map_or(0, Vec::len);
        assert!(
            rows.iter().all(|row| row.len() == cols),
            "Grid rows must all have the same length"
        );

        let data = rows.into_iter().flatten().collect();
        Self { data, rows: row_count, cols }
    }

    /// Returns the number of rows.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns in each row.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns `(rows, columns)`.
    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    /// Returns the total number of elements.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.index_of(row, col).map(|index| &self.data[index])
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.index_of(row, col).map(|index| &mut self.data[index])
    }

    pub fn get_row(&self, row: usize) -> Option<&[T]> {
        if row >= self.rows {
            return None;
        }
        let start = row * self.cols;
        Some(&self.data[start..start + self.cols])
    }

    pub fn get_row_mut(&mut self, row: usize) -> Option<&mut [T]> {
        if row >= self.rows {
            return None;
        }
        let start = row * self.cols;
        Some(&mut self.data[start..start + self.cols])
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn into_vec(self) -> Vec<T> {
        self.data
    }

    /// Converts the array into a list of rows.
    pub fn into_rows(self) -> Vec<Vec<T>> {
        let mut data = self.data.into_iter();
        (0..self.rows).map(|_| data.by_ref().take(self.cols).collect()).collect()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }

    pub fn iter_rows(&self) -> impl DoubleEndedIterator<Item = &[T]> + ExactSizeIterator {
        (0..self.rows).map(move |row| {
            let start = row * self.cols;
            &self.data[start..start + self.cols]
        })
    }

    fn index_of(&self, row: usize, col: usize) -> Option<usize> {
        if row < self.rows && col < self.cols { Some(row * self.cols + col) } else { None }
    }
}

impl<T: Clone> Grid<T> {
    /// Creates a `rows` by `cols` array filled with `value`.
    pub fn new(rows: usize, cols: usize, value: T) -> Self {
        Self { data: vec![value; checked_len(rows, cols)], rows, cols }
    }

    pub fn fill(&mut self, value: T) {
        self.data.fill(value);
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        self.get(row, col).unwrap_or_else(|| {
            panic!(
                "Vec2d index ({row}, {col}) out of bounds for dimensions ({}, {})",
                self.rows, self.cols
            )
        })
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        let dimensions = self.dimensions();
        self.get_mut(row, col).unwrap_or_else(|| {
            panic!(
                "Vec2d index ({row}, {col}) out of bounds for dimensions ({}, {})",
                dimensions.0, dimensions.1
            )
        })
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        self.get_row(row)
            .unwrap_or_else(|| panic!("Vec2d row {row} out of bounds for {} rows", self.rows))
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let rows = self.rows;
        self.get_row_mut(row)
            .unwrap_or_else(|| panic!("Vec2d row {row} out of bounds for {rows} rows"))
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Grid<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

fn checked_len(rows: usize, cols: usize) -> usize {
    rows.checked_mul(cols).expect("Vec2d dimensions overflow")
}
