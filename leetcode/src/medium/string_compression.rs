crate::solution!();

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let (mut idx_ans, mut idx) = (0, 0);
        while idx < chars.len() {
            let group_ch = chars[idx];
            let mut count = 0;
            while idx < chars.len() && chars[idx] == group_ch {
                idx += 1;
                count += 1;
            }
            chars[idx_ans] = group_ch;
            idx_ans += 1;

            if count != 1 {
                count.to_string().chars().for_each(|ch| {
                    chars[idx_ans] = ch;
                    idx_ans += 1;
                });
            }
        }

        idx_ans as i32
    }
}
