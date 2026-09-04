//
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct SquareMatrix<T> {
    data: Vec<T>,
    len: usize,
}

impl<T: Default + Clone> SquareMatrix<T> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn new(size: usize) -> Self {
        SquareMatrix {
            data: vec![T::default(); size * size],
            len: size,
        }
    }
}

impl<T> Index<(usize, usize)> for SquareMatrix<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * self.len + col]
    }
}

impl<T> IndexMut<(usize, usize)> for SquareMatrix<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row * self.len + col]
    }
}
