use crate::{PositionI, PositionU, Size};
#[cfg(not(feature = "std"))]
use core::fmt::{Display, Formatter, Result as DisplayResult};
#[cfg(feature = "std")]
use std::fmt::{Display, Formatter, Result as DisplayResult};

macro_rules! clip_top_left {
    ($self:ident, $position:ident, $c:ident) => {{
        $position.$c = if $self.position.$c < 0 {
            0
        } else {
            $self.position.$c.unsigned_abs() as usize
        };
    }};
}

macro_rules! clip_bottom_right {
    ($position:ident, $size:ident, $other:ident, $c:ident, $d:ident) => {{
        if $position.$c + $size.$d > $other.size.$d {
            $size.$d = $other.size.$d - $position.$c;
        }
    }};
}

macro_rules! size {
    ($t:tt) => {
        pub const fn from_size(size: Size) -> Self {
            Self {
                position: $t::ZERO,
                size,
            }
        }
    };
}

macro_rules! overlaps {
    ($self:ident, $other:ident, $other_w:expr, $other_h:expr) => {{
        $self.position.x <= $other.position.x + $other_w
            && $self.position.x + $other_w > $other.position.x
            && $self.position.y <= $other.position.y + $other_h
            && $self.position.y + $other_h > $other.position.y
    }};
}

macro_rules! impl_display_rect {
    ($r:ident) => {
        impl Display for $r {
            fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
                write!(f, "{}, {}", self.position, self.size)
            }
        }
    };
}

/// A rectangle in which the `position` can have negative values.
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct RectI {
    pub position: PositionI,
    pub size: Size,
}

impl RectI {
    size!(PositionI);

    pub const fn new(position: PositionI, size: Size) -> Option<Self> {
        if size.width == 0 || size.height == 0 {
            None
        } else {
            Some(Self { position, size })
        }
    }

    pub const fn overlaps(&self, other: &Self) -> bool {
        let other_w = other.size.width.cast_signed();
        let other_h = other.size.height.cast_signed();
        overlaps!(self, other, other_w, other_h)
    }

    pub const fn clip(self, other: Self) -> Option<RectU> {
        // Don't try clipping if there is no overlap.
        if self.overlaps(&other) {
            let mut position = PositionU::ZERO;
            clip_top_left!(self, position, x);
            clip_top_left!(self, position, y);
            let mut size = self.size;
            clip_bottom_right!(position, size, other, x, width);
            clip_bottom_right!(position, size, other, y, height);
            let rect = RectU { position, size };
            if rect.size.width > 0 && rect.size.height > 0 {
                Some(rect)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub const fn into_rectu(self) -> RectU {
        RectU {
            position: PositionU {
                x: self.position.x.cast_unsigned(),
                y: self.position.y.cast_unsigned(),
            },
            size: self.size,
        }
    }
}

/// A rectangle defined by a position and size.
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct RectU {
    pub position: PositionU,
    pub size: Size,
}

impl RectU {
    size!(PositionU);

    pub const fn overlaps(&self, other: &Self) -> bool {
        overlaps!(self, other, other.size.width, other.size.height)
    }

    pub const fn into_recti(self) -> RectI {
        RectI {
            position: PositionI {
                x: self.position.x.cast_signed(),
                y: self.position.y.cast_signed(),
            },
            size: self.size,
        }
    }
}

impl_display_rect!(RectI);
impl_display_rect!(RectU);
