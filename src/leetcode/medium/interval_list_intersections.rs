crate::leetcode::solution!();

macro_rules! choose {
    ($computation: expr; T => $true_result: expr, F => $false_result: expr) => {
        match $computation {
            true => $true_result,
            false => $false_result,
        }
    };
}

struct Interval(i32, i32);

impl Interval {
    fn from(vector: &Vec<i32>) -> Self {
        Interval(vector[0], vector[1])
    }

    fn as_vector(&self) -> Vec<i32> {
        vec![self.0, self.1]
    }
}

use std::cmp::{max, min};

impl std::ops::BitXor for &Interval {
    type Output = Option<Interval>;

    fn bitxor(self, other: &Interval) -> Self::Output {
        let (start, end) = (max(self.0, other.0), min(self.1, other.1));
        choose!(start <= end; T => Some(Interval(start, end)), F => None)
    }
}

impl Solution {
    pub fn interval_intersection(list_a: Vec<Vec<i32>>, list_b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if list_a.len() == 0 || list_b.len() == 0 {
            return result;
        }

        let (mut idx_a, mut idx_b) = (0, 0);
        while idx_a < list_a.len() && idx_b < list_b.len() {
            let (interval_b, interval_a) = (
                Interval::from(&list_a[idx_a]),
                Interval::from(&list_b[idx_b]),
            );
            if let Some(intersection) = &interval_b ^ &interval_a {
                result.push(intersection.as_vector());
            }

            *choose!(interval_a.1 > interval_b.1; T => &mut idx_a, F => &mut idx_b) += 1;
        }

        result
    }
}
