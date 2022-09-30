crate::solution!();

const BIG_NUM_MODULO: i32 = 1e9 as i32 + 7;

struct StrPtr(*const u8);

impl StrPtr {
    fn build(str: &String) -> Self {
        Self(str.as_ptr())
    }

    fn get(&self, index: usize) -> u8 {
        unsafe { *self.0.wrapping_offset(index as isize) }
    }
}

impl Solution {
    pub fn count_texts(pressed_keys: String) -> i32 {
        let keys = StrPtr::build(&pressed_keys);
        let keys_size = pressed_keys.len();

        let mut dp = [1, 1, 1, 1, 1];
        for idx in (0..keys_size).rev() {
            dp[idx % 5] = 0;
            let max_j = usize::min(
                keys_size,
                idx + if b"79".contains(&keys.get(idx)) { 4 } else { 3 },
            );

            let mut forward_idx = idx;
            while forward_idx < max_j && keys.get(idx) == keys.get(forward_idx) {
                dp[idx % 5] = (dp[idx % 5] + dp[(forward_idx + 1) % 5]) % BIG_NUM_MODULO;
                forward_idx += 1;
            }
        }
        dp[0]
    }
}
