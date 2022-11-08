crate::solution!();

fn ch_to_idx(ch: char) -> usize {
    (ch as u8 - b'a') as usize
}

fn construct_decode_table(key: &str) -> [char; 26] {
    let mut decode_table = [' '; 26];
    let mut decoded_token = b'a';

    key.chars().for_each(|token| {
        if token == ' ' || decode_table[ch_to_idx(token)] != ' ' {
            return;
        }

        decode_table[ch_to_idx(token)] = decoded_token as char;
        decoded_token += 1;
    });

    decode_table
}

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let decode_table = construct_decode_table(&key);

        message
            .chars()
            .map(|token| match token {
                ' ' => ' ',
                _ => decode_table[ch_to_idx(token)],
            })
            .collect()
    }
}
