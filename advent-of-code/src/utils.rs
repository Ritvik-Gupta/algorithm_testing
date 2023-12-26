use derive_more::Add;
use std::fmt::Write;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default, Add)]
pub struct Vector<T>(pub T, pub T);

impl<T> From<(T, T)> for Vector<T> {
    fn from(loc: (T, T)) -> Self {
        Vector(loc.0, loc.1)
    }
}

impl<T> Vector<T> {
    // fn within_box(&self, boundary: Vector<T>) -> Vector<T>
    // where
    //     T: Ord + Copy,
    // {
    //     Vector(self.0.min(boundary.0), self.1.min(boundary.1))
    // }
}

pub fn log_matrix<T>(matrix: &Vec<Vec<T>>)
where
    T: std::fmt::Display,
{
    let mut matrix_str = String::new();
    matrix.iter().for_each(|row| {
        row.iter().for_each(|x| {
            write!(matrix_str, "{x} ").unwrap();
        });
        writeln!(matrix_str).unwrap();
    });

    log::debug!(
        "2D Matrix ({}, {}) :\n{matrix_str}",
        matrix.len(),
        matrix[0].len()
    );
}
