mod operations;

pub trait Matrix:
    std::ops::IndexMut<crate::services::vector_2d::Vector2d, Output = crate::fraction::Fraction>
{
    fn num_rows(&self) -> usize;
    fn num_cols(&self) -> usize;
}

pub type MatrixNM = crate::array_2d::Array2D<crate::fraction::Fraction>;

impl Matrix for MatrixNM {
    fn num_rows(&self) -> usize {
        self.num_rows()
    }

    fn num_cols(&self) -> usize {
        self.num_cols()
    }
}
