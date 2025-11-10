

/// A signed `(x, y)` pixel position.
#[derive(Copy, Clone, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PositionI {
    pub x: isize,
    pub y: isize,
}

/// An unsigned `(x, y)` pixel position.
#[derive(Copy, Clone, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PositionU {
    pub x: usize,
    pub y: usize,
}
