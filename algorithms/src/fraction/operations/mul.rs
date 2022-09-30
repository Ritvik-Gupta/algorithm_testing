use crate::fraction::{AsFraction, Fraction};

impl std::ops::Mul for Fraction {
    type Output = Fraction;

    fn mul(self, other: Fraction) -> Fraction {
        Fraction::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }
}

impl std::ops::Mul<i32> for Fraction {
    type Output = Fraction;

    fn mul(self, other: i32) -> Fraction {
        self.mul(other.fr())
    }
}
