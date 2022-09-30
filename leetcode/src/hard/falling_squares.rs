crate::solution!();

#[derive(Hash, PartialEq, Eq)]
struct ShadowArea(i32, i32);

impl ShadowArea {
    fn any_contains(shadow_area_a: &Self, shadow_area_b: &Self) -> bool {
        shadow_area_a.contains(shadow_area_b) || shadow_area_b.contains(shadow_area_a)
    }

    fn contains(&self, other: &Self) -> bool {
        (self.0 <= other.0 && other.0 <= self.1) || (self.0 <= other.1 && other.1 <= self.1)
    }
}

impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        use std::{cmp, collections::HashMap};

        let (mut max_height, mut result, mut recorded_shadow) = (0, Vec::new(), HashMap::new());

        for square in positions.into_iter() {
            let (left, side_length) = (square[0], square[1]);
            let sq_shadow_area = ShadowArea(left, left + side_length - 1);
            let sq_shadow_height = recorded_shadow
                .iter()
                .filter(|(shadow_area, _)| ShadowArea::any_contains(&sq_shadow_area, shadow_area))
                .map(|(_, &shadow_height)| shadow_height)
                .max()
                .unwrap_or(0)
                + side_length;
            recorded_shadow.insert(sq_shadow_area, sq_shadow_height);

            max_height = cmp::max(max_height, sq_shadow_height);
            result.push(max_height);
        }

        result
    }
}
