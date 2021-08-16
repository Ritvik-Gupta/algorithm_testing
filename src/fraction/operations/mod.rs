mod add;
mod mul;

use super::Fraction;

impl std::ops::Not for Fraction {
    type Output = Fraction;

    fn not(self) -> Fraction {
        Fraction::new(self.denominator, self.numerator)
    }
}

impl std::ops::Neg for Fraction {
    type Output = Fraction;

    fn neg(self) -> Fraction {
        Fraction::new(-self.numerator, self.denominator)
    }
}
