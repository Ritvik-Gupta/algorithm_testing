crate::leetcode::solution!();

fn gcd(mut a: u32, mut b: u32) -> u32 {
    loop {
        if a == 0 {
            return b;
        }
        let c = b % a;
        b = a;
        a = c;
    }
}

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut deck_cards_frequency = std::collections::BTreeMap::new();

        let first_card_num = deck[0];
        for card_num in deck {
            let card_frequency = deck_cards_frequency.entry(card_num).or_insert(0);
            *card_frequency += 1;
        }

        let mut min_card_group_size = deck_cards_frequency[&first_card_num];
        for (_, card_frequency) in deck_cards_frequency {
            min_card_group_size = gcd(min_card_group_size, card_frequency);
            if min_card_group_size < 2 {
                return false;
            }
        }
        true
    }
}
