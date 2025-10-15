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
