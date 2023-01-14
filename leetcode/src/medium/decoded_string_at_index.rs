crate::solution!();

impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let (encoding, mut k, mut n, mut i) = (s.as_bytes(), k as usize, 0, 0);

        for ch in encoding.iter().map(|&ch| ch as char) {
            n = match ch.to_digit(10) {
                Some(digit) => n * digit as usize,
                _ => n + 1,
            };
            if k <= n {
                break;
            }
            i += 1;
        }

        for ch in encoding[0..=i].iter().rev().map(|&ch| ch as char) {
            if let Some(digit) = ch.to_digit(10) {
                n /= digit as usize;
                k %= n;
            } else {
                if k == n || k == 0 {
                    return format!("{}", ch);
                };
                n -= 1;
            }
        }

        unreachable!()
    }
}
