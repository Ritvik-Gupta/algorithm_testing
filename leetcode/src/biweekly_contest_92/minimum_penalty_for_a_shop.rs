crate::solution!();

impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let mut closing_hour = customers.len();
        let mut penalty_incurred = customers.chars().filter(|ch| ch == &'N').count() as i32;
        let mut penalty = penalty_incurred;

        customers.char_indices().rev().for_each(|(hour, ch)| {
            penalty += if ch == 'Y' { 1 } else { -1 };
            if penalty <= penalty_incurred {
                penalty_incurred = penalty;
                closing_hour = hour;
            }
        });

        closing_hour as i32
    }
}
