use crate::{PositionI, PositionU, Size};
use std::fmt::{Display, Formatter};

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
        let mut size = Size {
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
        if position.x < size.w && position.y < size.h {
            // Set the offset.
            self.src_position = position;
            // Apply the offset.
            size.w -= position.x;
            size.h -= position.y;
            // Set the new clipped size.
            self.src_size_clipped = size;
        }
    }
}

impl Display for ClippedRect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "dst position: {}\ndst position (clipped): {}\nsrc size: {}, src size (clipped): {}",
            self.dst_position, self.dst_position_clipped, self.src_size, self.src_size_clipped
        )
    }
}
