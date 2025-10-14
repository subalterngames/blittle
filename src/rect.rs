use std::fmt::{Display, Formatter};

/// A destination rect.
/// `(x, y)` is the top-left position of the `Rect`.
#[derive(Copy, Clone, Default)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

impl Display for Rect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.w, self.h)
    }
}
