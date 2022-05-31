crate::leetcode::solution!();

fn fetch_bit(str_ptr: *const u8) -> usize {
    (unsafe { *str_ptr } - b'0') as usize
}

impl Solution {
    pub fn has_all_codes(str: String, k: i32) -> bool {
        let k = k as usize;

        let mut combinations = 1 << k;
        let all_one = combinations - 1;

        let mut seen_combinations = vec![false; combinations];
        let mut hash_value = 0;

        let mut str_ptr = str.as_ptr();
        for i in 0..str.len() {
            hash_value = ((hash_value << 1) & all_one) | fetch_bit(str_ptr);

            if i >= k - 1 && seen_combinations[hash_value] == false {
                seen_combinations[hash_value] = true;
                combinations -= 1;
                if combinations == 0 {
                    return true;
                }
            }

            str_ptr = str_ptr.wrapping_offset(1);
        }
        false
    }
}
