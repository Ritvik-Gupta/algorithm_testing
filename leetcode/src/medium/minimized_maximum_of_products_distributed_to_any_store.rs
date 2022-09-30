crate::solution!();

const MAXIMUM_QUANTITY: i32 = 100000;

impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let (mut left, mut right) = (1, MAXIMUM_QUANTITY);
        while left < right {
            let mid = (left + right) / 2;
            match quantities
                .iter()
                .map(|&quantity| (quantity + mid - 1) / mid)
                .sum::<i32>()
                .cmp(&n)
            {
                std::cmp::Ordering::Greater => left = mid + 1,
                _ => right = mid,
            }
        }
        left
    }
}
