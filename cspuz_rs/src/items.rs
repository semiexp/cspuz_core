#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arrow {
    Unspecified,
    Up,
    Down,
    Left,
    Right,
}

pub type NumberedArrow = (Arrow, i32);

/// Values associated with each orthogonal direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FourDirections<T> {
    pub up: T,
    pub down: T,
    pub left: T,
    pub right: T,
}
