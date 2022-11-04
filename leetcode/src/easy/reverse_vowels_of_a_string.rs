crate::solution!();

impl Solution {
    pub fn reverse_vowels(word: String) -> String {
        let mut vowels_found = word.chars().filter(|&ch| "aeiouAEIOU".contains(ch)).rev();

        word.chars()
            .map(|ch| {
                if "aeiouAEIOU".contains(ch) {
                    vowels_found.next().unwrap()
                } else {
                    ch
                }
            })
            .collect::<String>()
    }
}
