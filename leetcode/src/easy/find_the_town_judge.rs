crate::solution!();

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut count = vec![0; n + 1];

        for t in trust {
            count[t[0] as usize] -= 1;
            count[t[1] as usize] += 1;
        }

        for i in 1..=n {
            if count[i] == n - 1 {
                return i as i32;
            }
        }
        -1
    }
}
