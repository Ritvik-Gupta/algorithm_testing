crate::solution!();

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut a, mut b) = (1, 1);
        for _ in 0..n - 1 {
            let next = a + b;
            a = b;
            b = next;
        }
        b
    }
}
