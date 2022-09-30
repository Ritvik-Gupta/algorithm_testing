crate::solution!();

use std::collections::HashMap;

impl Solution {
    pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
        let mut seen_card_positions = HashMap::with_capacity(cards.len() / 10);
        let mut minimum_cards = cards.len() + 1;

        for (idx, &card) in cards.iter().enumerate() {
            if let Some(last_seen_idx) = seen_card_positions.get(&card) {
                minimum_cards = minimum_cards.min(idx - last_seen_idx + 1);
            }
            seen_card_positions.insert(card, idx);
        }

        if minimum_cards == cards.len() + 1 {
            -1
        } else {
            minimum_cards as i32
        }
    }
}
