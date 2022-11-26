crate::solution!();

use std::collections::BTreeMap;

impl Solution {
    pub fn find_winners(games: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut score_records = BTreeMap::new();

        games.iter().for_each(|game| {
            *score_records.entry(game[0]).or_insert(0) += 0;
            *score_records.entry(game[1]).or_insert(0) += 1;
        });

        let mut score_board = vec![Vec::new(); 2];

        score_records
            .into_iter()
            .filter(|&(_, player_score)| player_score <= 1)
            .for_each(|(player, score)| score_board[score].push(player));

        score_board
    }
}
