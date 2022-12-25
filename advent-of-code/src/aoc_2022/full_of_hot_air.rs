use derive_more::{Deref, Display};

const fn generate_fives_table() -> [i64; 20] {
    let mut table = [0; 20];
    let (mut i, mut num) = (0, 1);
    while i < 20 {
        table[i] = num;
        num *= 5;
        i += 1;
    }
    table
}

static FIVES_TABLE: [i64; 20] = generate_fives_table();

#[derive(Display, Deref)]
pub struct Snafu(String);

impl From<i64> for Snafu {
    fn from(mut num: i64) -> Self {
        let mut snafu_rep = String::new();

        let mut base = FIVES_TABLE[FIVES_TABLE
            .binary_search(&(num.abs() * 2))
            .map_or_else(|i| i - 1, |i| i)];

        while base > 0 {
            let bit_value = (if num.abs() <= base / 2 {
                0
            } else if num.abs() <= (3 * base) / 2 {
                1
            } else {
                2
            }) * num.signum();

            snafu_rep.push(match bit_value {
                -2 => '=',
                -1 => '-',
                0..=2 => (bit_value as u8 + b'0') as char,
                _ => unreachable!(),
            });

            num -= bit_value * base;
            base /= 5;
        }

        assert_eq!(num, 0);

        Snafu(snafu_rep)
    }
}

impl From<Snafu> for i64 {
    fn from(snafu: Snafu) -> Self {
        snafu
            .chars()
            .rev()
            .scan(1, |power, ch| {
                let bit_value = match ch {
                    '=' => -2,
                    '-' => -1,
                    '0'..='2' => (ch as u8 - b'0') as i64,
                    _ => unreachable!(),
                } * *power;
                *power *= 5;
                Some(bit_value)
            })
            .sum()
    }
}

pub struct FullOfHotAir;

impl crate::AdventDayProblem for FullOfHotAir {
    type Arg = Vec<Snafu>;
    type Ret = Snafu;

    fn get_problem_name() -> &'static str {
        crate::problem_name!()
    }

    fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
        dataset.map(|line| Snafu(line)).collect()
    }

    fn part_1(snafu_nums: Self::Arg) -> Self::Ret {
        snafu_nums.into_iter().map(i64::from).sum::<i64>().into()
    }

    fn part_2(_: Self::Arg) -> Self::Ret {
        panic!("No part two")
    }
}
