crate::leetcode::solution!();

fn lcs(word1: &[u8], word2: &[u8], m: usize, n: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
    if m == 0 || n == 0 {
        return 0;
    }
    if memo[m][n] > 0 {
        return memo[m][n];
    }

    memo[m][n] = if word1[m - 1] == word2[n - 1] {
        1 + lcs(word1, word2, m - 1, n - 1, memo)
    } else {
        i32::max(
            lcs(word1, word2, m - 1, n, memo),
            lcs(word1, word2, m, n - 1, memo),
        )
    };
    memo[m][n]
}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut memo = vec![vec![0; word2.len() + 1]; word1.len() + 1];
        (word1.len() + word2.len()) as i32
            - 2 * lcs(
                word1.as_bytes(),
                word2.as_bytes(),
                word1.len(),
                word2.len(),
                &mut memo,
            )
    }
}
