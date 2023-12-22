use itertools::Itertools;
use once_cell::sync::Lazy;
use std::{
    cmp::Ordering::{self, *},
    collections::HashMap,
};

macro_rules! map {
    ($($key: expr => $value: expr),*) => {
        HashMap::from([$(($key, $value)),*])
    };
}

const CARD_MAP: Lazy<HashMap<char, u64>> = Lazy::new(|| {
    let mut map = map!('A' => 14, 'K' => 13, 'Q' => 12, 'J' => 11, 'T' => 10);
    for i in 2..=9 {
        map.insert((b'0' + i as u8) as char, i);
    }
    map
});

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Hand {
    hand: [char; 5],
    consider_joker: bool,
}

impl Hand {
    fn construct(hand: &str) -> Self {
        let mut hand = hand.chars();
        Hand {
            hand: [
                hand.next().unwrap(),
                hand.next().unwrap(),
                hand.next().unwrap(),
                hand.next().unwrap(),
                hand.next().unwrap(),
            ],
            consider_joker: false,
        }
    }

    fn power(&self) -> u8 {
        let mut card_sets = HashMap::new();
        for card in &self.hand {
            *card_sets.entry(*card).or_insert(0) += 1;
        }

        if self.consider_joker {
            let total_jokers = card_sets.remove(&'J').unwrap_or(0);
        }

        match card_sets.values().sorted().rev().collect_vec()[..] {
            [5] => 6,
            [4, 1] => 5,
            [3, 2] => 4,
            [3, 1, 1] => 3,
            [2, 2, 1] => 2,
            [2, _, _, _] => 1,
            _ => 0,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cmp = self.power().cmp(&other.power());
        if cmp != Equal {
            return Some(cmp);
        }

        for i in 0..5 {
            let cmp = CARD_MAP[&self.hand[i]].cmp(&CARD_MAP[&other.hand[i]]);
            if cmp != Equal {
                return Some(cmp);
            }
        }

        Some(Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct CamelCards;

impl crate::AdventDayProblem for CamelCards {
    type Arg = Vec<(Hand, u64)>;

    type Ret = u64;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        dataset
            .map(|line| {
                let (hand, bet) = line.split_at(5);
                (Hand::construct(hand), bet.trim().parse().unwrap())
            })
            .collect()
    }

    fn part_1(mut games: Self::Arg) -> Self::Ret {
        games.sort_by_key(|(hand, _)| hand.clone());
        games
            .iter()
            .zip(1..)
            .map(|((_, bet), value)| bet * value)
            .sum()
    }

    fn part_2(mut games: Self::Arg) -> Self::Ret {
        games
            .iter_mut()
            .for_each(|(hand, _)| hand.consider_joker = true);

        games.sort_by_key(|(hand, _)| hand.clone());
        games
            .iter()
            .zip(1..)
            .map(|((_, bet), value)| bet * value)
            .sum()
    }
}
