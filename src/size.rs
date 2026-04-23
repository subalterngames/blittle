#[cfg(not(feature = "std"))]
use core::fmt::{Display, Formatter, Result as DisplayResult};
#[cfg(feature = "std")]
use std::fmt::{Display, Formatter, Result as DisplayResult};

/// Rectangular bounds defined by a width and height.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub const fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
        write!(f, "({}, {})", self.width, self.height)
    }
}
