pub mod easy;
pub mod hard;
pub mod medium;

macro_rules! algo {
    () => {
        pub struct Solution;
    };
}

pub(crate) use algo;
