crate::leetcode::solution!();

const BIG_NUM_MODULO: i32 = 1e9 as i32 + 7;

impl Solution {
    pub fn count_texts(pressed_keys: String) -> i32 {
        let keys = pressed_keys.as_bytes();
        let keys_size = keys.len();

        let mut table = [1, 1, 1, 1, 1];
        for idx in (0..keys_size).rev() {
            table[idx % 5] = 0;
            let max_j = usize::min(
                keys_size,
                idx + if b"79".contains(&keys[idx]) { 4 } else { 3 },
            );
            let mut forward_idx = idx;
            while forward_idx < max_j && keys[idx] == keys[forward_idx] {
                table[idx % 5] = (table[idx % 5] + table[(forward_idx + 1) % 5]) % BIG_NUM_MODULO;
                forward_idx += 1;
            }
        }
        table[0]
    }
}
