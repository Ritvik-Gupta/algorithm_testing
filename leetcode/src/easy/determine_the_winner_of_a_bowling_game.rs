crate::solution!();

fn tally_score(scores: Vec<i32>) -> i32 {
    let mut strike_ptr = 0;
    scores
        .into_iter()
        .map(|s| {
            let score = s * (1 + (strike_ptr > 0) as i32);
            strike_ptr = if s == 10 { 2 } else { strike_ptr - 1 };
            score
        })
        .sum()
}

impl Solution {
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        use std::cmp::Ordering::*;
        match tally_score(player1).cmp(&tally_score(player2)) {
            Greater => 1,
            Less => 2,
            Equal => 0,
        }
    }
}
