#[derive(Clone, Copy)]
pub enum BinaryChild {
    LEFT = -1,
    RIGHT = 1,
}

impl BinaryChild {
    pub fn name(&self) -> &str {
        match self {
            BinaryChild::LEFT => "Left",
            BinaryChild::RIGHT => "right",
        }
    }
}
