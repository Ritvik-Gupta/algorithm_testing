use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::multispace1,
    character::complete::u32 as integer,
    sequence::{preceded, separated_pair, tuple},
    IResult, Parser,
};

pub struct ScratchCard {
    winning_nums: HashSet<u32>,
    my_nums: Vec<u32>,
}

impl ScratchCard {
    fn construct(input: &str) -> IResult<&str, Self> {
        preceded(
            tuple((tag("Card"), multispace1, integer, tag(": "))),
            separated_pair(
                take_while1(|c: char| c.is_digit(10) || c.is_whitespace()),
                tag("|"),
                take_while1(|c: char| c.is_digit(10) || c.is_whitespace()),
            ),
        )
        .map(|(w, m): (&str, &str)| Self {
            winning_nums: HashSet::from_iter(w.split(' ').filter_map(|s| s.parse().ok())),
            my_nums: Vec::from_iter(m.split(' ').filter_map(|s| s.parse().ok())),
        })
        .parse(input)
    }
}

pub struct Scratchcards;

impl crate::AdventDayProblem for Scratchcards {
    type Arg = Vec<ScratchCard>;

    type Ret = u64;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        dataset
            .map(|line| ScratchCard::construct(&line).unwrap().1)
            .collect()
    }

    fn part_1(scratch_cards: Self::Arg) -> Self::Ret {
        scratch_cards
            .iter()
            .map(|card| {
                let matches = card
                    .my_nums
                    .iter()
                    .filter(|num| card.winning_nums.contains(&num))
                    .count() as u64;

                matches.checked_sub(1).map_or(0, |bit_shf| 1 << bit_shf)
            })
            .sum()
    }

    fn part_2(scratch_cards: Self::Arg) -> Self::Ret {
        let n = scratch_cards.len();
        let mut cards_owned = vec![1; n];

        for (id, card) in scratch_cards.into_iter().enumerate() {
            let matches = card
                .my_nums
                .iter()
                .filter(|num| card.winning_nums.contains(&num))
                .count();

            for next_card_id in id + 1..=usize::min(id + matches, n) {
                cards_owned[next_card_id] += cards_owned[id];
            }
        }

        cards_owned.into_iter().sum()
    }
}
