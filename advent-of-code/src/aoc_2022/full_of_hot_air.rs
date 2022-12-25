use crate::{Naive, OptimizationFlag};
use derive_more::Display;
use std::marker::PhantomData;

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

fn int_to_snafu(bit_value: i64) -> char {
    match bit_value {
        -2 => '=',
        -1 => '-',
        0..=2 => (bit_value as u8 + b'0') as char,
        _ => unreachable!(),
    }
}

fn snafu_to_int(bit_value: char) -> i64 {
    match bit_value {
        '=' => -2,
        '-' => -1,
        '0'..='2' => (bit_value as u8 - b'0') as i64,
        _ => unreachable!(),
    }
}

#[derive(Display)]
#[display(fmt = "SNAFU('{}')", rep)]
pub struct Snafu<OF: OptimizationFlag> {
    rep: String,
    _phantom: PhantomData<OF>,
}

impl<OF: OptimizationFlag> Snafu<OF> {
    fn new(snafu_rep: String) -> Self {
        Self {
            rep: snafu_rep,
            _phantom: PhantomData,
        }
    }
}

impl From<i64> for Snafu<Naive> {
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

            snafu_rep.push(int_to_snafu(bit_value));

            num -= bit_value * base;
            base /= 5;
        }

        assert_eq!(num, 0);

        Snafu::new(snafu_rep)
    }
}

impl From<i64> for Snafu<OptimizedDecryption> {
    fn from(mut num: i64) -> Self {
        let mut snafu_rep = String::new();

        while num > 0 {
            let bit_value = ((num + 2) % 5) - 2;
            snafu_rep.push(int_to_snafu(bit_value));

            num -= bit_value;
            num /= 5;
        }

        Snafu::new(snafu_rep.chars().rev().collect())
    }
}

impl<OF: OptimizationFlag> From<Snafu<OF>> for i64 {
    fn from(snafu: Snafu<OF>) -> Self {
        snafu
            .rep
            .chars()
            .rev()
            .scan(1, |power, ch| {
                let bit_value = snafu_to_int(ch) * *power;
                *power *= 5;
                Some(bit_value)
            })
            .sum()
    }
}

pub struct FullOfHotAir;

pub struct OptimizedDecryption;

impl OptimizationFlag for OptimizedDecryption {}

macro_rules! generate_implementation {
    ($optimization_flag: tt) => {
        impl crate::AdventDayProblem<$optimization_flag> for FullOfHotAir {
            type Arg = Vec<Snafu<$optimization_flag>>;
            type Ret = Snafu<$optimization_flag>;

            fn get_problem_name() -> &'static str {
                crate::problem_name!()
            }

            fn construct_arg(dataset: impl Iterator<Item = String>) -> Self::Arg {
                dataset.map(|line| Snafu::new(line)).collect()
            }

            fn part_1(snafu_nums: Self::Arg) -> Self::Ret {
                snafu_nums.into_iter().map(i64::from).sum::<i64>().into()
            }

            fn part_2(_: Self::Arg) -> Self::Ret {
                panic!("No part two")
            }
        }
    };
}

generate_implementation!(Naive);
generate_implementation!(OptimizedDecryption);
