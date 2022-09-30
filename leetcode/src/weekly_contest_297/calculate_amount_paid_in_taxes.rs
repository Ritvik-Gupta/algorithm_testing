crate::solution!();

impl Solution {
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        let mut lower_bound = 0;
        let mut tax_pay = 0.0;

        for bracket in brackets {
            let (upper_bound, percent) = (bracket[0], bracket[1]);

            tax_pay +=
                (i32::min(upper_bound, income) - lower_bound) as f64 * percent as f64 / 100.0;
            lower_bound = upper_bound;
            if upper_bound > income {
                break;
            }
        }
        tax_pay
    }
}
