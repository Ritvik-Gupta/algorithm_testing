crate::solution!();

fn combination(mut n: usize, mut r: usize) -> usize {
    let (mut numerator, mut denominator) = (1, 1);
    while r > 0 {
        numerator *= n;
        denominator *= r;
        n -= 1;
        r -= 1;
    }
    numerator / denominator
}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        combination(m + n - 2, usize::min(m - 1, n - 1)) as i32
    }
}
