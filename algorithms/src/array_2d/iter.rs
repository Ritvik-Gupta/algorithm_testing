use crate::array_2d::{partition_by_cols, Array2D};
use services::{vector_2d::Vector2d, Pair, PairMutOper};

impl<T> Array2D<T> {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (Vector2d, &'a T)> {
        let partition = partition_by_cols(self.num_cols());
        self.arr
            .iter()
            .enumerate()
            .map(move |(pos, val)| (partition(pos), val))
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = (Vector2d, &'a mut T)> {
        let partition = partition_by_cols(self.num_cols());
        self.arr
            .iter_mut()
            .enumerate()
            .map(move |(pos, val)| (partition(pos), val))
    }
}

impl<T> Array2D<T> {
    pub fn col_iter<'a>(&'a self, col_idx: usize) -> impl Iterator<Item = &'a T> {
        self.iter()
            .filter(move |&((_, y), _)| y == col_idx)
            .map(Pair::to_second)
    }

    pub fn col_iter_mut<'a>(&'a mut self, col_idx: usize) -> impl Iterator<Item = &'a mut T> {
        self.iter_mut()
            .filter(move |&((_, y), _)| y == col_idx)
            .map(Pair::to_second)
    }
}

impl<T> Array2D<T> {
    pub fn row_iter<'a>(&'a self, row_idx: usize) -> impl Iterator<Item = &'a T> {
        self.iter()
            .skip_while(move |&((x, _), _)| x != row_idx)
            .take_while(move |&((x, _), _)| x == row_idx)
            .map(Pair::to_second)
    }

    pub fn row_iter_mut<'a>(&'a mut self, row_idx: usize) -> impl Iterator<Item = &'a mut T> {
        self.iter_mut()
            .skip_while(move |&((x, _), _)| x != row_idx)
            .take_while(move |&((x, _), _)| x == row_idx)
            .map(Pair::to_second)
    }
}

impl<T> IntoIterator for Array2D<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.arr.into_iter()
    }
}
