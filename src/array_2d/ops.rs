use crate::{
    array_2d::Array2D,
    services::vector_2d::{Vector2d, VectorOper},
};

impl<T> std::ops::Index<Vector2d> for Array2D<T> {
    type Output = T;

    fn index(&self, index: Vector2d) -> &Self::Output {
        if !index.is_within(self.dimensions) {
            panic!("lvms");
        }

        &self.arr[index.0 * self.dimensions.1 + index.1]
    }
}

impl<T> std::ops::IndexMut<Vector2d> for Array2D<T> {
    fn index_mut(&mut self, index: Vector2d) -> &mut Self::Output {
        if !index.is_within(self.dimensions) {
            panic!("knkvs");
        }

        &mut self.arr[index.0 * self.dimensions.1 + index.1]
    }
}
