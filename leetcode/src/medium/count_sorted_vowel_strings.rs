crate::solution!();

const NUM_VOWELS: usize = 5;

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut vowels_combinations = [1; NUM_VOWELS];
        for _ in 1..n {
            for i in 1..NUM_VOWELS {
                vowels_combinations[i] += vowels_combinations[i - 1];
            }
        }
        vowels_combinations.iter().sum()
    }
}
