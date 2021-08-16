use super::super::{AsFraction, Fraction};

impl std::ops::Add for Fraction {
    type Output = Fraction;

    fn add(self, other: Fraction) -> Fraction {
        let lcm = Fraction::lcm(self.denominator, other.denominator);
        Fraction::new(
            (self.numerator * lcm) / self.denominator + (other.numerator * lcm) / other.denominator,
            lcm,
        )
    }
}

impl std::ops::Add<i32> for Fraction {
    type Output = Fraction;

    fn add(self, other: i32) -> Fraction {
        self.add(other.fr())
    }
}
