crate::solution!();

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let n = strs[0].len();
        let strs = strs.iter().map(|row| row.as_bytes()).collect::<Vec<_>>();

        (0..n)
            .filter(|&i| {
                strs.iter()
                    .map(|row| row[i])
                    .scan(b'a', |prev, ch| {
                        let prev_ch = *prev;
                        *prev = ch;
                        Some((prev_ch, ch))
                    })
                    .any(|(prev, curr)| prev > curr)
            })
            .count() as i32
    }
}
