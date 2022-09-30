crate::solution!();

impl Solution {
    pub fn longest_subsequence(s: String, max_num: i32) -> i32 {
        let mut binary_word = s.chars().rev();
        let (mut seq_num, mut subsequence_size) = (0, 0);

        for byte_offset in 0..31 {
            let bin = match binary_word.next() {
                Some(bin) => bin,
                _ => break,
            };

            let mut num = seq_num;
            if bin == '1' {
                num |= 1 << byte_offset;
            }
            if num > max_num {
                break;
            }
            seq_num = num;
            subsequence_size += 1;
        }

        subsequence_size + binary_word.filter(|&bin| bin == '0').count() as i32
    }
}
