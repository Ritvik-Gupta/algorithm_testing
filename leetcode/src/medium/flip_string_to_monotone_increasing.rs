crate::solution!();

impl Solution {
    pub fn min_flips_mono_incr(binary: String) -> i32 {
        let (mut counter_one, mut counter_flip) = (0, 0);
        for ch in binary.chars() {
            match ch {
                '1' => counter_one += 1,
                _ => counter_flip += 1,
            }
            counter_flip = i32::min(counter_one, counter_flip);
        }
        counter_flip
    }
}
