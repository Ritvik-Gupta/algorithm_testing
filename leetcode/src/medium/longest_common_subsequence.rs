crate::solution!();

struct TabulatedSubSeq<'a> {
    text1: &'a [u8],
    text2: &'a [u8],
    memo: Vec<Vec<i32>>,
}

impl<'a> TabulatedSubSeq<'a> {
    fn new(text1: &'a str, text2: &'a str) -> Self {
        let mut memo = vec![vec![-1; text2.len() + 1]; text1.len() + 1];
        for i in 0..text1.len() + 1 {
            memo[i][0] = 0;
        }
        for j in 0..text2.len() + 1 {
            memo[0][j] = 0;
        }

        TabulatedSubSeq {
            text1: text1.as_bytes(),
            text2: text2.as_bytes(),
            memo,
        }
    }

    fn bottom_up(&mut self, m: usize, n: usize) -> i32 {
        if self.memo[m][n] != -1 {
            return self.memo[m][n];
        }

        self.memo[m][n] = match self.text1[m - 1] == self.text2[n - 1] {
            true => 1 + self.bottom_up(m - 1, n - 1),
            _ => i32::max(self.bottom_up(m - 1, n), self.bottom_up(m, n - 1)),
        };
        self.memo[m][n]
    }
}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        TabulatedSubSeq::new(&text1, &text2).bottom_up(text1.len(), text2.len())
    }
}
