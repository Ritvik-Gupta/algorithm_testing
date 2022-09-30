mod convert_from;
mod display;
mod operations;

#[derive(Clone, Copy)]
pub struct Fraction {
    numerator: i32,
    denominator: i32,
}

pub trait AsFraction {
    fn fr(self) -> Fraction;
}

impl AsFraction for i32 {
    fn fr(self) -> Fraction {
        Fraction::new(self, 1)
    }
}

impl Fraction {
    pub fn new(mut numerator: i32, mut denominator: i32) -> Fraction {
        if denominator == 0 {
            panic!("Infinite Fraction specified");
        }

        let hcf = Fraction::hcf(numerator, denominator);
        numerator /= hcf;
        denominator /= hcf;
        if denominator < 0 {
            numerator *= -1;
            denominator *= -1;
        }
        Fraction {
            numerator,
            denominator,
        }
    }

    pub fn hcf(mut x: i32, mut y: i32) -> i32 {
        loop {
            let remainder = x % y;
            if remainder == 0 {
                return y;
            }
            x = y;
            y = remainder;
        }
    }

    pub fn lcm(a: i32, b: i32) -> i32 {
        (a * b) / Fraction::hcf(a, b)
    }

    pub fn pair(&self) -> (i32, i32) {
        (self.numerator, self.denominator)
    }

    pub fn value(&self) -> f64 {
        f64::from(self.numerator) / f64::from(self.denominator)
    }
}

pub use {convert_from::*, display::*};
