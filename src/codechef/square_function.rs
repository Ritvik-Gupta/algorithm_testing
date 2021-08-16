use std::collections::{hash_map::HashMap, hash_set::HashSet};

struct FactoringLeft(HashSet<u32>);

impl FactoringLeft {
    pub fn with(mut num: u32) -> u32 {
        let (mut possible_factor, mut factoring_left) = (2, FactoringLeft(HashSet::new()));
        while 1 < num && possible_factor <= num {
            if num % possible_factor == 0 {
                num /= possible_factor;
                factoring_left.add(possible_factor);
                continue;
            }
            possible_factor += 1;
        }

        factoring_left
            .0
            .into_iter()
            .reduce(|x, y| x * y)
            .unwrap_or(1)
    }

    fn add(&mut self, num: u32) {
        if self.0.contains(&num) {
            self.0.remove(&num);
        } else {
            self.0.insert(num);
        }
    }
}

fn pair_combinations_for(num: usize) -> usize {
    (num * (num - 1)) / 2
}

pub fn calc_non_perfect_squares(nums: Vec<u32>) -> usize {
    let mut left_factor_group = HashMap::<u32, Vec<u32>>::new();

    nums.iter().for_each(|&num| {
        let factoring_left = FactoringLeft::with(num);
        if let Some(common_left_factors) = left_factor_group.get_mut(&factoring_left) {
            common_left_factors.push(num);
        } else {
            left_factor_group.insert(factoring_left, vec![num]);
        }
    });

    pair_combinations_for(nums.len())
        - left_factor_group
            .iter()
            .map(|(_, common_left_factors)| pair_combinations_for(common_left_factors.len()))
            .reduce(|a, b| a + b)
            .unwrap()
}

pub fn test() -> Result<(), Box<dyn std::error::Error>> {
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
