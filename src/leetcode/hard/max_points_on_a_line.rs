pub struct Solution;

fn gcd(mut a: u32, mut b: u32) -> u32 {
    loop {
        if a == 0 {
            return b;
        }
        let c = b % a;
        b = a;
        a = c;
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Default)]
struct SlopeFraction {
    is_negative: bool,
    numerator: u32,
    denominator: u32,
}

impl SlopeFraction {
    fn from(coord_a: &Vec<i32>, coord_b: &Vec<i32>) -> Self {
        let delta_y = i32::abs(coord_a[1] - coord_b[1]) as u32;
        let delta_x = i32::abs(coord_a[0] - coord_b[0]) as u32;

        if delta_y == 0 {
            return Self {
                denominator: 1,
                ..Default::default()
            };
        }
        if delta_x == 0 {
            return Self {
                numerator: 1,
                ..Default::default()
            };
        }

        let gcd = gcd(delta_x, delta_y);
        Self {
            is_negative: (coord_a[1] >= coord_b[1]) ^ (coord_a[0] >= coord_b[0]),
            numerator: delta_y / gcd,
            denominator: delta_x / gcd,
        }
    }
}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 1 {
            return 1;
        }

        let mut max_points_on_a_line = 0;
        for i in 0..points.len() {
            let mut seen_lines = std::collections::BTreeMap::new();
            for j in i + 1..points.len() {
                let points_on_line = seen_lines
                    .entry(SlopeFraction::from(&points[i], &points[j]))
                    .or_insert(1);
                *points_on_line += 1;

                if *points_on_line > max_points_on_a_line {
                    max_points_on_a_line = *points_on_line;
                }
            }
        }
        max_points_on_a_line
    }
}
