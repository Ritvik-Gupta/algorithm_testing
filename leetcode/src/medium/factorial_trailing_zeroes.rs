crate::solution!();

impl Solution {
    pub fn trailing_zeroes(num: i32) -> i32 {
        let (mut couting_fives, mut multiple_of_5) = (0, 5);
        while multiple_of_5 <= num {
            let mut i = multiple_of_5;
            while i != 0 && i % 5 == 0 {
                couting_fives += 1;
                i /= 5;
            }
            multiple_of_5 += 5;
        }
        couting_fives
    }
}
