crate::solution!();

fn recur(k: u32, n: u32, collected_num: u32, result: &mut Vec<i32>) {
    if n == 0 {
        result.push(collected_num as i32);
        return;
    }

    let digit = collected_num % 10;

    if k == 0 {
        recur(k, n - 1, collected_num * 10 + digit, result);
        return;
    }

    if digit >= k {
        recur(k, n - 1, collected_num * 10 + (digit - k), result);
    }
    if digit + k < 10 {
        recur(k, n - 1, collected_num * 10 + (digit + k), result);
    }
}

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let (n, k) = (n as u32, k as u32);

        let mut result = Vec::new();
        (1..10).for_each(|digit| recur(k, n - 1, digit, &mut result));
        result
    }
}
