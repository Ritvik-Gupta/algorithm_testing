use crate::{Pair, PairRefOper};

pub type Vector2d = Pair<usize, usize>;

pub trait VectorOper: PairRefOper<usize, usize> {
    fn rect_size(&self) -> usize {
        self.x() * self.y()
    }

    fn any_is(&self, check_with: usize) -> bool {
        self.any(|a| a == check_with)
    }

    fn any(&self, check_fn: impl Fn(usize) -> bool) -> bool {
        check_fn(self.x()) || check_fn(self.y())
    }

    fn is_within(&self, other: impl VectorOper) -> bool {
        self.x() < other.x() && self.y() < other.y()
    }
}

impl VectorOper for Vector2d {}
