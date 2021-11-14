#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Vector(i32, i32);

impl std::ops::Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Self::Output {
        Vector(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Self::Output {
        Vector(self.0 - other.0, self.1 - other.1)
    }
}

impl std::ops::Not for Vector {
    type Output = Vector;

    fn not(self) -> Self::Output {
        Vector(self.1, self.0)
    }
}

impl std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector(-self.0, -self.1)
    }
}

impl Into<Vector> for Vec<i32> {
    fn into(self) -> Vector {
        Vector(self[0] as i32, self[1] as i32)
    }
}

use std::collections::HashMap;

struct DetectSquares(HashMap<Vector, i32>);

impl DetectSquares {
    #[allow(dead_code)]
    fn new() -> Self {
        Self(HashMap::new())
    }

    #[allow(dead_code)]
    fn add(&mut self, point: Vec<i32>) {
        *self.0.entry(point.into()).or_insert(0) += 1;
    }

    fn get_num_points(&self, point: &Vector) -> i32 {
        *self.0.get(point).unwrap_or(&0)
    }

    #[allow(dead_code)]
    fn count(&self, point: Vec<i32>) -> i32 {
        let fixed_point: Vector = point.into();
        self.0
            .iter()
            .map(|(&point, &num_points_a)| {
                let delta_difference = match point - fixed_point {
                    Vector(0, 0) => return 0,
                    delta @ Vector(0, _) => !delta,
                    delta @ Vector(_, 0) => -!delta,
                    _ => return 0,
                };
                num_points_a
                    * self.get_num_points(&(fixed_point + delta_difference))
                    * self.get_num_points(&(point + delta_difference))
            })
            .sum()
    }
}
