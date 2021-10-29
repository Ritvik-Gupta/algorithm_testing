pub struct Solution;

const LETTER_OFFSET: u8 = b'a';
const NUMBER_OFFSET: u8 = b'0';

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let mut coordinates = coordinates.chars();
        let (letter, number) = (
            coordinates.next().unwrap() as u8 - LETTER_OFFSET,
            coordinates.next().unwrap() as u8 - NUMBER_OFFSET,
        );
        letter % 2 == number % 2
    }
}
