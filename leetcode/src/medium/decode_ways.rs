crate::solution!();

fn can_be_decoded(prev_token: char, token: char) -> bool {
    prev_token == '1' || (prev_token == '2' && ('0'..='6').contains(&token))
}

impl Solution {
    pub fn num_decodings(message: String) -> i32 {
        let mut dp = [0; 3];
        dp[1] = 1;

        let mut prev_token = '3';
        message.chars().for_each(|token| {
            if token != '0' {
                dp[2] += dp[1];
            }
            if can_be_decoded(prev_token, token) {
                dp[2] += dp[0];
            }

            prev_token = token;

            dp[0] = dp[1];
            dp[1] = dp[2];
            dp[2] = 0;
        });

        dp[1]
    }
}
