crate::solution!();

fn ch_to_idx(ch: char) -> usize {
    (ch as u8 - b'a') as usize
}

fn create_keyboard() -> [u8; 26] {
    let mut keyboard = [0; 26];
    "qwertyuiop"
        .chars()
        .for_each(|ch| keyboard[ch_to_idx(ch)] = 1);
    "asdfghjkl"
        .chars()
        .for_each(|ch| keyboard[ch_to_idx(ch)] = 2);
    "zxcvbnm".chars().for_each(|ch| keyboard[ch_to_idx(ch)] = 3);

    keyboard
}

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let keyboard = create_keyboard();

        words
            .into_iter()
            .filter(|word| {
                word.chars()
                    .scan(' ', |prev_ch, ch| {
                        let ch = ch.to_ascii_lowercase();
                        let res = *prev_ch == ' '
                            || keyboard[ch_to_idx(*prev_ch)] == keyboard[ch_to_idx(ch)];
                        *prev_ch = ch;
                        Some(res)
                    })
                    .all(|x| x)
            })
            .collect()
    }
}
