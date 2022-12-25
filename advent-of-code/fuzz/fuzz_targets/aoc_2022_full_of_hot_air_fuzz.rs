#![no_main]

use advent_of_code::aoc_2022::full_of_hot_air::{OptimizedDecryption, Snafu};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: u32| {
    let data = data as i64;
    assert_eq!(data, i64::from(Snafu::<OptimizedDecryption>::from(data)));
});
