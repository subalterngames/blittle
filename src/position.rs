#[cfg(not(feature = "std"))]
use core::fmt::{Display, Formatter, Result as DisplayResult};
#[cfg(feature = "std")]
use std::fmt::{Display, Formatter, Result as DisplayResult};

macro_rules! impl_position {
    ($position:ident, $c:ty) => {
        /// An `(x, y)` pixel position.
        #[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        pub struct $position {
            pub x: $c,
            pub y: $c,
        }

        impl $position {
            pub const ZERO: Self = Self { x: 0, y: 0 };

            pub const fn new(x: $c, y: $c) -> Self {
                Self { x, y }
            }
        }

        impl Display for $position {
            fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
                write!(f, "({}, {})", self.x, self.y)
            }
        }
    };
}

impl_position!(PositionI, isize);

impl_position!(PositionU, usize);

impl From<PositionU> for PositionI {
    fn from(value: PositionU) -> Self {
        Self {
            x: value.x.cast_signed(),
            y: value.y.cast_signed(),
        }
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
