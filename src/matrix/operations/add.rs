use crate::matrix::Matrix;

impl<'a, 'b> std::ops::Add<&'b Matrix> for &'a Matrix {
    type Output = Result<Matrix, String>;

    fn add(self, other: &'b Matrix) -> Result<Matrix, String> {
        Err("".to_string())
    }
}
