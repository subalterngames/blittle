use crate::{PositionI, PositionU, Size};
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
        let d1 = $position.$c + $size.$c;
        if d1 > $other.size.$c {
            $size.$c = $other.size.$c - d1;
        }
    }};
}

macro_rules! position_size {
    ($t:tt) => {
        pub const fn from_position(position: $t) -> Self {
            Self {
                position,
                size: USizeVec2::ZERO,
            }
        }

        pub const fn from_size(size: USizeVec2) -> Self {
            Self {
                position: $t::ZERO,
                size,
            }
        }

        pub const fn zeroed_position(mut self) -> Self {
            self.position = $t::ZERO;
            self
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

// Required to let the function be const
macro_rules! max_coordinate {
    ($self:expr, $other:expr, $c:ident) => {{
        if $self.position.$c > $other.position.$c {
            $other.position.$c
        } else {
            $self.position.$c
        }
    }};
}

#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct RectI {
    pub position: I64Vec2,
    pub size: USizeVec2,
}

impl RectI {
    position_size!(I64Vec2);

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

#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct RectU {
    pub position: USizeVec2,
    pub size: USizeVec2,
}

impl RectU {
    position_size!(USizeVec2);

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

/// The original destination position and source size, and the position and size used for blitting.
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ClippedRect {
    /// The original top-left position of the where the source image should blit to.
    pub dst_position: PositionI,
    /// The clipped top-left position that is actually used for blitting.
    pub dst_position_clipped: PositionU,
    /// The original size of the source image.
    pub src_size: Size,
    /// The clipped size of the source image, which is used for blitting.
    pub src_size_clipped: Size,
    /// A pixel offset in the source bitmap.
    pub(crate) src_position: PositionU,
    /// The size of the destination image.
    pub dst_size: Size,
}

impl ClippedRect {
    /// Clip `src_size` such that it fits within the rectangle defined by `dst_position` and `dst_size`.
    ///
    /// Returns None if the region is beyond the bounds of `dst_size`
    /// or if the blittable source size would be `(0, 0)`.
    pub const fn new(dst_position: PositionI, dst_size: Size, src_size: Size) -> Option<Self> {
        // Check if the source image is totally out of bounds.
        if dst_position.x + (src_size.w.cast_signed()) < 0
            || dst_position.y + (src_size.h.cast_signed()) < 0
        {
            None
        } else {
            // Get the clipped size and position.
            let mut x = 0;
            let mut y = 0;
            let mut src_size_clipped = src_size;
            if dst_position.x < 0 {
                src_size_clipped.w = src_size.w.saturating_sub(dst_position.x.unsigned_abs());
            } else {
                x = dst_position.x.unsigned_abs();
            }
            if dst_position.y < 0 {
                src_size_clipped.h = src_size.h.saturating_sub(dst_position.y.unsigned_abs());
            } else {
                y = dst_position.y.unsigned_abs();
            }
            let dst_position_clipped = PositionU { x, y };

            // Get the source position.
            let mut src_position = PositionU { x: 0, y: 0 };
            if dst_position.x < 0 {
                src_position.x += dst_position.x.unsigned_abs();
            }
            if dst_position.y < 0 {
                src_position.y += dst_position.y.unsigned_abs();
            }

            // This allows us to do unchecked subtraction.
            // The `blit` methods will also check `is_inside`.
            if dst_position_clipped.x < dst_size.w && dst_position_clipped.y < dst_size.h {
                let w = dst_size.w - dst_position_clipped.x;
                if w < src_size.w {
                    src_size_clipped.w = w;
                }
                let h = dst_size.h - dst_position_clipped.y;
                if h < src_size.h {
                    src_size_clipped.h = h;
                }
                if src_size_clipped.w == 0 || src_size_clipped.h == 0 {
                    None
                } else {
                    Some(Self {
                        dst_position,
                        dst_position_clipped,
                        src_size,
                        src_size_clipped,
                        dst_size,
                        src_position,
                    })
                }
            } else {
                None
            }
        }
    }

    /// Returns true if this rect overlaps with `b`.
    pub const fn overlaps(&self, b: &ClippedRect) -> bool {
        self.dst_position_clipped.x <= b.dst_position_clipped.x + b.src_size_clipped.w
            && self.dst_position_clipped.x + self.src_size_clipped.w > b.dst_position_clipped.x
            && self.dst_position_clipped.y <= b.dst_position_clipped.y + b.src_size_clipped.h
            && self.dst_position_clipped.y + self.src_size_clipped.h > b.dst_position_clipped.y
    }

    /// Set the rect within the source bitmap to blit.
    ///
    /// By default, the entirety of the source bitmap blits.
    /// This sets an internal positional offset. and modifies `self.src_clipped_area`.
    /// If the positional offset would be beyond the original clipped area, this function does nothing.
    ///
    /// - `position` is the position offset from the top-level corner of the source bitmap.
    /// - `size` is the size of the rect. This will be clipped to `self.src_size_clipped` if needed.
    pub const fn set_src_rect(&mut self, position: PositionU, size: Size) {
        // Clip the size.
        let size = Size {
            w: if self.src_size_clipped.w < size.w {
                self.src_size_clipped.w
            } else {
                size.w
            },
            h: if self.src_size_clipped.h < size.h {
                self.src_size_clipped.h
            } else {
                size.h
            },
        };
        // Apply the offset only if it's within the clipped bounds.
        if position.x + size.w <= self.src_size.w && position.y + size.h <= self.src_size.h {
            // Set the offset.
            self.src_position = position;
            // Set the new clipped size.
            self.src_size_clipped = size;
        }
    }
}

impl Display for ClippedRect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "dst position: {}\ndst position (clipped): {}\nsrc size: {}\nsrc size (clipped): {}",
            self.dst_position, self.dst_position_clipped, self.src_size, self.src_size_clipped
        )
    }
}
