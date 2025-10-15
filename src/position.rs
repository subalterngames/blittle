use crate::Size;

/// An `(x, y)` pixel position.
#[derive(Copy, Clone, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub const fn is_inside(&self, size: &Size) -> bool {
        self.x < size.w && self.y < size.h
    }
}
