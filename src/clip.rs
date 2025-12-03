use crate::{PositionI, PositionU, Size};
use std::fmt::{Display, Formatter};

/// The original destination position and source size, and the position and size used for blitting.
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ClippedRect {
    pub dst_position: PositionI,
    pub dst_position_clipped: PositionU,
    pub src_size: Size,
    pub src_size_clipped: Size,
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
            let mut x = 0;
            let mut src_size_clipped = src_size;
            if dst_position.x < 0 {
                src_size_clipped.w = src_size.w.saturating_sub(dst_position.x.unsigned_abs());
            } else {
                x = dst_position.x.unsigned_abs();
            }
            let mut y = 0;
            if dst_position.y < 0 {
                src_size_clipped.h = src_size.h.saturating_sub(dst_position.y.unsigned_abs());
            } else {
                y = dst_position.y.unsigned_abs();
            }
            let dst_position_clipped = PositionU { x, y };
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
                if src_size_clipped.w == 0 && src_size_clipped.h == 0 {
                    None
                }
                else {
                    Some(Self {
                        dst_position,
                        dst_position_clipped,
                        src_size,
                        src_size_clipped,
                        dst_size
                    })
                }
            } else {
                None
            }
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
