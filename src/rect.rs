use glam::{I64Vec2, USizeVec2};
use std::fmt::{Display, Formatter};

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
    ($position:ident, $size:ident, $other:ident, $c:ident) => {{
        if $position.$c + $size.$c > $other.size.$c {
            $size.$c = $other.size.$c - $position.$c;
        }
    }};
}

macro_rules! size {
    ($t:tt) => {
        pub const fn from_size(size: USizeVec2) -> Self {
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

#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct RectI {
    pub position: I64Vec2,
    pub size: USizeVec2,
}

impl RectI {
    size!(I64Vec2);

    pub const fn new(position: I64Vec2, size: USizeVec2) -> Option<Self> {
        if size.x == 0 || size.y == 0 {
            None
        } else {
            Some(Self { position, size })
        }
    }

    pub const fn overlaps(&self, other: &Self) -> bool {
        let other_w = other.size.x.cast_signed() as i64;
        let other_h = other.size.y.cast_signed() as i64;
        overlaps!(self, other, other_w, other_h)
    }

    pub const fn clip(self, other: Self) -> Option<RectU> {
        let other_w = other.size.x.cast_signed() as i64;
        let other_h = other.size.y.cast_signed() as i64;
        // Don't try clipping if there is no overlap.
        if overlaps!(self, other, other_w, other_h) {
            let mut position = USizeVec2::ZERO;
            clip_top_left!(self, position, x);
            clip_top_left!(self, position, y);
            let mut size = self.size;
            clip_bottom_right!(position, size, other, x);
            clip_bottom_right!(position, size, other, y);
            let rect = RectU { position, size };
            if rect.size.x > 0 && rect.size.y > 0 {
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
            position: USizeVec2 {
                x: self.position.x.cast_unsigned() as usize,
                y: self.position.y.cast_unsigned() as usize,
            },
            size: self.size,
        }
    }
}

impl Display for RectI {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.position, self.size)
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct RectU {
    pub position: USizeVec2,
    pub size: USizeVec2,
}

impl RectU {
    size!(USizeVec2);

    pub const fn overlaps(&self, other: &Self) -> bool {
        overlaps!(self, other, other.size.x, other.size.y)
    }

    pub const fn into_recti(self) -> RectI {
        RectI {
            position: I64Vec2 {
                x: self.position.x.cast_signed() as i64,
                y: self.position.y.cast_signed() as i64,
            },
            size: self.size,
        }
    }
}
