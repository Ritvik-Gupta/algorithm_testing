crate::solution!();

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s = s.as_bytes();
        let mut res = Vec::new();
        let mut dp = vec![vec![false; s.len()]; s.len()];

        for i in 0..s.len() {
            for j in 0..=i {
                if s[i] == s[j] && (i - j <= 2 || dp[j + 1][i - 1]) {
                    dp[j][i] = true;
                }
            }
        }

        helper(&mut res, &mut Vec::new(), &dp, s, 0);
        res
    }
}

fn helper<'a>(
    res: &mut Vec<Vec<String>>,
    path: &mut Vec<&'a [u8]>,
    dp: &Vec<Vec<bool>>,
    s: &'a [u8],
    pos: usize,
) {
    if pos == s.len() {
        res.push(
            path.iter()
                .map(|x| String::from_utf8_lossy(x).to_string())
                .collect(),
        );
        return;
    }

    for i in pos..s.len() {
        if dp[pos][i] {
            path.push(&s[pos..i + 1]);
            helper(res, path, dp, s, i + 1);
            path.pop();
        }
    }
}
