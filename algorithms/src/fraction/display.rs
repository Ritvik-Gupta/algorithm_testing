use crate::fraction::Fraction;

impl std::fmt::Display for Fraction {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "{} {}",
            self.numerator,
            if self.denominator != 1 {
                format!("by {}", self.denominator)
            } else {
                "frac".to_string()
            }
        )
    }
}
