use crate::matrix::Matrix;

impl std::ops::Add<&dyn Matrix> for &dyn Matrix {
    type Output = Result<Box<dyn Matrix>, String>;

    fn add(self, _other: &dyn Matrix) -> Self::Output {
        Err("".to_string())
    }
}
