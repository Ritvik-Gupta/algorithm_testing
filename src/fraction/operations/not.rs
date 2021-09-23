use crate::fraction::Fraction;

impl std::ops::Not for Fraction {
    type Output = Fraction;

    fn not(self) -> Fraction {
        Fraction::new(self.denominator, self.numerator)
    }
}
