crate::leetcode::solution!();

trait IsAbsUnit {
    fn is_abs_unit(&self) -> bool;
}

impl IsAbsUnit for i32 {
    fn is_abs_unit(&self) -> bool {
        self.abs() == 1
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Vector(i32, i32);

impl std::ops::Sub for &Vector {
    type Output = Vector;

    fn sub(self, other: Self) -> Self::Output {
        Vector(self.0 - other.0, self.1 - other.1)
    }
}

impl Vector {
    fn from(vector: Vec<i32>) -> Self {
        Self(vector[0], vector[1])
    }

    fn illumination_intensity_by(&self, lamp: &Vector) -> Option<bool> {
        match self - lamp {
            Vector(0, 0) => Some(true),
            Vector(a, 0) | Vector(0, a) => Some(a.is_abs_unit()),
            Vector(x, y) if x.abs() == y.abs() => Some(x.is_abs_unit()),
            _ => None,
        }
    }
}

impl Solution {
    pub fn grid_illumination(
        _n: i32,
        lamp_positions: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        use std::collections::BTreeSet;

        let mut lamps: BTreeSet<_> = lamp_positions.into_iter().map(Vector::from).collect();

        queries
            .into_iter()
            .map(Vector::from)
            .map(|ref query| {
                let mut is_illuminated = 0;
                lamps = lamps
                    .iter()
                    .filter_map(|lamp| {
                        if let Some(has_high_intensity) = query.illumination_intensity_by(lamp) {
                            is_illuminated = 1;
                            if has_high_intensity {
                                return None;
                            }
                        }
                        Some(lamp.clone())
                    })
                    .collect();
                is_illuminated
            })
            .collect()
    }
}
