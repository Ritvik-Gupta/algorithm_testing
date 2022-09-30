use crate::fraction::Fraction;
use services::Pair;

impl std::convert::From<Pair<i32, i32>> for Fraction {
    fn from(pair: Pair<i32, i32>) -> Self {
        Fraction::new(pair.0, pair.1)
    }
}

impl std::convert::From<Fraction> for Pair<i32, i32> {
    fn from(frac: Fraction) -> Self {
        (frac.numerator, frac.denominator)
    }
}
