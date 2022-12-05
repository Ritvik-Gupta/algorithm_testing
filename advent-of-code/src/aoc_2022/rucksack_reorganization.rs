use bit_set::BitSet;

fn construct_rucksack(tokens: impl Iterator<Item = char>) -> BitSet<u64> {
    let mut rucksack = BitSet::default();
    tokens.for_each(|token| {
        rucksack.insert(if ('a'..='z').contains(&token) {
            token as u8 - b'a'
        } else {
            token as u8 - b'A' + 26
        } as usize);
    });
    rucksack
}

pub struct RucksackReorganization;

impl crate::AdventDayProblem for RucksackReorganization {
    type Arg = Vec<String>;
    type Ret = i128;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        dataset.collect()
    }

    fn part_1(rucksacks: Self::Arg) -> Self::Ret {
        rucksacks
            .iter()
            .map(|line| {
                (
                    construct_rucksack(line[..line.len() / 2].chars()),
                    construct_rucksack(line[line.len() / 2..].chars()),
                )
            })
            .map(|(mut left_bag, right_bag)| {
                left_bag.intersect_with(&right_bag);
                (left_bag.iter().next().unwrap() + 1) as i128
            })
            .sum()
    }

    fn part_2(rucksacks: Self::Arg) -> Self::Ret {
        (0..rucksacks.len())
            .step_by(3)
            .map(|i| {
                let (mut r1, r2, r3) = (
                    construct_rucksack(rucksacks[i].chars()),
                    construct_rucksack(rucksacks[i + 1].chars()),
                    construct_rucksack(rucksacks[i + 2].chars()),
                );

                r1.intersect_with(&r2);
                r1.intersect_with(&r3);

                (r1.iter().next().unwrap() + 1) as i128
            })
            .sum()
    }
}
