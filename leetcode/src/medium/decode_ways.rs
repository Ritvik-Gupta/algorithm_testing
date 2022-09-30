crate::solution!();

fn can_be_decoded(prev_token: u8, token: u8) -> bool {
    prev_token == b'1' || (prev_token == b'2' && (b'0'..=b'6').contains(&token))
}

impl Solution {
    pub fn num_decodings(message: String) -> i32 {
        let msg = message.as_bytes();
        let mut dp = [0; 3];
        dp[1] = 1;

        for i in 0..msg.len() {
            if msg[i] != b'0' {
                dp[2] += dp[1];
            }

            match msg.get(i - 1) {
                Some(&ch) if can_be_decoded(ch, msg[i]) => dp[2] += dp[0],
                _ => {}
            }

            dp[0] = dp[1];
            dp[1] = dp[2];
            dp[2] = 0;
        }

        dp[1]
    }
}
