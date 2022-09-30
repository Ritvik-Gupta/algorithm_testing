pub mod iter;
pub mod ops;

use services::vector_2d::{Vector2d, VectorOper};

pub struct Array2D<T> {
    dimensions: Vector2d,
    arr: Vec<T>,
}

fn partition_by_cols(num_cols: usize) -> impl Fn(usize) -> Vector2d {
    move |pos| (pos / num_cols, pos % num_cols)
}

impl<T> Array2D<T> {
    pub fn with_default(dimensions: Vector2d, default_value: T) -> Self
    where
        T: Clone,
    {
        if dimensions.any_is(0) {
            panic!();
        }

        Array2D {
            dimensions,
            arr: vec![default_value; dimensions.rect_size()],
        }
    }

    pub fn with_generator(dimensions: Vector2d, generate_value: impl Fn(Vector2d) -> T) -> Self {
        if dimensions.any_is(0) {
            panic!();
        }

        let partition = partition_by_cols(dimensions.1);
        Array2D {
            dimensions,
            arr: (0..dimensions.rect_size())
                .map(partition)
                .map(generate_value)
                .collect(),
        }
    }

    pub fn num_rows(&self) -> usize {
        self.dimensions.0
    }

    pub fn num_cols(&self) -> usize {
        self.dimensions.1
    }
}
