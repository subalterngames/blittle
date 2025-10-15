use crate::Size;

/// A signed `(x, y)` pixel position.
pub struct PositionI {
    pub x: isize,
    pub y: isize,
}

/// An unsigned `(x, y)` pixel position.
#[derive(Copy, Clone, Default)]
pub struct PositionU {
    pub x: usize,
    pub y: usize,
}

impl PositionU {
    pub const fn is_inside(&self, size: &Size) -> bool {
        self.x < size.w && self.y < size.h
    }
}
