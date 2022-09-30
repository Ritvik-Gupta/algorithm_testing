use crate::fraction::Fraction;

impl std::ops::Neg for Fraction {
    type Output = Fraction;

    fn neg(self) -> Fraction {
        Fraction::new(-self.numerator, self.denominator)
    }
}
