crate::solution!();

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let mut extremes = sentence.split(' ').map(|word| {
            let word = word.as_bytes();
            (*word.first().unwrap(), *word.last().unwrap())
        });

        let (first_start, mut prev_end) = extremes.next().unwrap();
        let mut extremes = extremes.chain(std::iter::once((first_start, prev_end)));

        extremes.all(|(start, end)| {
            let cmp = start == prev_end;
            prev_end = end;
            cmp
        })
    }
}
