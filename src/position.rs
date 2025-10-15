use crate::Size;

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
