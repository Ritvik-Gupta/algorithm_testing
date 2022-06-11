crate::leetcode::solution!();

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort();

        spells
            .iter()
            .map(|&spell| spell as i64)
            .map(|spell| {
                let (mut lower_ptr, mut upper_ptr) = (0, potions.len() - 1);
                while lower_ptr <= upper_ptr {
                    let mid_ptr = lower_ptr + (upper_ptr - lower_ptr) / 2;

                    let strength = spell * potions[mid_ptr] as i64;
                    if strength >= success {
                        if mid_ptr == 0 {
                            break;
                        }
                        upper_ptr = mid_ptr - 1;
                    } else if strength < success {
                        lower_ptr = mid_ptr + 1;
                    }
                }
                (potions.len() - lower_ptr) as i32
            })
            .collect()
    }
}
