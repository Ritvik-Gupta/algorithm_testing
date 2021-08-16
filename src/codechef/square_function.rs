fn factoring_left_for(mut num: u32) -> u32 {
    use std::collections::hash_set::HashSet;

    let (mut possible_factor, mut factoring_left) = (2, HashSet::new());
    let mut add_or_rem_factor: Box<dyn FnMut(u32)> = Box::new(|possible_factor| {
        if factoring_left.contains(&possible_factor) {
            factoring_left.remove(&possible_factor);
        } else {
            factoring_left.insert(possible_factor);
        }
    });

    while 1 < num && possible_factor <= num {
        if num % possible_factor == 0 {
            num /= possible_factor;
            add_or_rem_factor(possible_factor);
            continue;
        }
        possible_factor += 1;
    }

    drop(add_or_rem_factor);
    factoring_left.into_iter().reduce(|x, y| x * y).unwrap_or(1)
}

fn pair_combinations_for(num: u32) -> u32 {
    (num * (num - 1)) / 2
}

pub fn calc_non_perfect_squares(nums: Vec<u32>) -> u32 {
    use std::{collections::hash_map::HashMap, convert::TryInto};

    let mut left_factors = HashMap::<u32, u32>::new();

    nums.iter()
        .map(|&num| factoring_left_for(num))
        .for_each(|factoring_left| *left_factors.entry(factoring_left).or_insert(0) += 1);

    left_factors
        .iter()
        .map(|(_, &common_left_factors)| pair_combinations_for(common_left_factors))
        .fold(
            pair_combinations_for(nums.len().try_into().unwrap()),
            |acc, pair_combinations| acc - pair_combinations,
        )
}

use crate::services::Returns;

pub fn test() -> Returns {
    let num_testcases: u32 = read!();
    for _ in 0..num_testcases {
        let list_size: u32 = read!();
        let mut list = Vec::<u32>::new();
        for _ in 0..list_size {
            list.push(read!());
        }

        println!("{}", calc_non_perfect_squares(list));
    }

    Ok(())
}
