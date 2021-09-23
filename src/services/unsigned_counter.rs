use std::cmp::max;
use std::convert::TryInto;

pub struct UnsignedCounter(u16);

impl UnsignedCounter {
    pub fn at(store: u16) -> Self {
        UnsignedCounter(store)
    }

    pub fn decrement(&mut self) {
        self.0 = max(i32::from(self.0) - 1, 0).try_into().unwrap();
    }
}

impl std::ops::Deref for UnsignedCounter {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for UnsignedCounter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
