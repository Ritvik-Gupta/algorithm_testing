crate::leetcode::solution!();

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let pair_sum = k as usize;

        let mut modulo_frequencies = vec![0; pair_sum];
        for num in arr {
            let mut mod_factor = num % k;
            if mod_factor < 0 {
                mod_factor += k;
            }
            modulo_frequencies[mod_factor as usize] += 1;
        }

        if modulo_frequencies[0] % 2 != 0 {
            return false;
        }

        if pair_sum % 2 == 0 && modulo_frequencies[pair_sum / 2] % 2 != 0 {
            return false;
        }

        for i in 1..(pair_sum + 1) / 2 {
            if modulo_frequencies[i] != modulo_frequencies[pair_sum - i] {
                return false;
            }
        }

        true
    }
}
