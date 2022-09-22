crate::leetcode::solution!();

fn recurse_permutations(
    word_left: &[u8],
    collected_word: &mut String,
    permutations: &mut Vec<String>,
) {
    let mut idx = 0;
    while idx < word_left.len() && !(word_left[idx] as char).is_alphabetic() {
        collected_word.push(word_left[idx] as char);
        idx += 1;
    }

    if idx >= word_left.len() {
        permutations.push(collected_word.clone());
    } else {
        collected_word.push((word_left[idx] as char).to_ascii_lowercase());
        recurse_permutations(&word_left[idx + 1..], collected_word, permutations);
        collected_word.pop();

        collected_word.push((word_left[idx] as char).to_ascii_uppercase());
        recurse_permutations(&word_left[idx + 1..], collected_word, permutations);
        collected_word.pop();
    }

    for _ in 0..idx {
        collected_word.pop();
    }
}

impl Solution {
    pub fn letter_case_permutation(word: String) -> Vec<String> {
        let mut permutations =
            Vec::with_capacity(word.chars().filter(|token| token.is_alphabetic()).count());

        recurse_permutations(
            word.as_bytes(),
            &mut String::with_capacity(word.len()),
            &mut permutations,
        );
        permutations
    }
}
