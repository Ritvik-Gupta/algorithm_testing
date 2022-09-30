crate::solution!();

impl Solution {
    pub fn shifting_letters(mut word: String, mut shifts: Vec<i32>) -> String {
        let mut running_shift = 0;
        shifts.iter_mut().rev().for_each(|shift| {
            running_shift = (running_shift + *shift) % 26;
            *shift = running_shift;
        });

        let word_ptr = word.as_mut_ptr();

        for (idx, token) in word.char_indices() {
            *unsafe { &mut *word_ptr.offset(idx as isize) } =
                b'a' + (token as u8 - b'a' + shifts[idx] as u8) % 26
        }

        word
    }
}
