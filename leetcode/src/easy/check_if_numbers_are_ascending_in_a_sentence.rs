crate::solution!();

impl Solution {
    pub fn are_numbers_ascending(sentence: String) -> bool {
        let mut prev_num = -1;

        sentence
            .split(' ')
            .filter_map(|word| word.parse::<i8>().ok())
            .all(|num| {
                let res = prev_num < num;
                prev_num = num;
                res
            })
    }
}
