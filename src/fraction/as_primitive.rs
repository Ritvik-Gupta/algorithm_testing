use crate::fraction::Fraction;

impl std::clone::Clone for Fraction {
    fn clone(&self) -> Fraction {
        Fraction::new(self.numerator, self.denominator)
    }
}

impl std::marker::Copy for Fraction {}

impl std::fmt::Display for Fraction {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        //"$numerator " + if (denominator != 1L) "by $denominator" else "frac"
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
