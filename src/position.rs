use std::fmt::{Display, Formatter};

/// A signed `(x, y)` pixel position.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PositionI {
    pub x: isize,
    pub y: isize,
}

impl Display for PositionI {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<PositionU> for PositionI {
    fn from(value: PositionU) -> Self {
        Self {
            x: value.x.cast_signed(),
            y: value.y.cast_signed(),
        }
    }
}

/// An unsigned `(x, y)` pixel position.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PositionU {
    pub x: usize,
    pub y: usize,
}

impl Display for PositionU {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<PositionI> for PositionU {
    fn from(value: PositionI) -> Self {
        Self {
            x: value.x.cast_unsigned(),
            y: value.y.cast_unsigned(),
        }
    }
}
