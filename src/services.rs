pub type Returns<T = ()> = Result<T, Box<dyn std::error::Error>>;

use std::cmp::max;
use std::convert::TryInto;

pub struct UnsignedCounter(u16);

impl UnsignedCounter {
    pub fn at_zero() -> Self {
        UnsignedCounter(0)
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn decrement(&mut self) {
        self.0 = max(i32::from(self.0) - 1, 0).try_into().unwrap();
    }
}

impl std::ops::Deref for UnsignedCounter {
    type Target = u16;

    fn deref(&self) -> &u16 {
        &self.0
    }
}
