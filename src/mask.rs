use crate::error::Error;
use crate::{RectU, Surface};
use glam::USizeVec2;

pub struct MaskedSurface<P: Copy + Clone + Sized + Default> {
    pub surface: Surface<P>,
    mask: Vec<bool>,
}

impl<P: Copy + Clone + Sized + Default> MaskedSurface<P> {
    pub fn new(surface: Surface<P>) -> Self {
        let len = surface.size.x * surface.size.y;
        Self {
            surface,
            mask: vec![false; len],
        }
    }

    pub fn new_with_mask(surface: Surface<P>, mask: Vec<bool>) -> Result<Self, Error> {
        if mask.len() == surface.size.x * surface.size.y {
            Ok(Self { surface, mask })
        } else {
            Err(Error::MaskSize {
                actual: mask.len(),
                expected: surface.size.x * surface.size.y,
            })
        }
    }

    pub fn mask_mut(&mut self) -> &mut [bool] {
        &mut self.mask
    }

    /// Blit onto `other`, using a mask.
    ///
    /// Be sure to call [Surface::position] or [Surface::set_position]
    /// before blitting to a *new* `other` surface.
    pub fn blit(&self, other: &mut Surface<P>) -> Result<(), Error> {
        // Try to get the destination rect.
        let destination_rect = self
            .surface
            .destination_rect
            .ok_or(Error::NoDestinationRect)?;

        // Blit either a chunk of the source buffer, or all of it.
        let blit_area = match self.surface.blit_area {
            Some(rect) => rect,
            None => RectU {
                position: USizeVec2::ZERO,
                size: destination_rect.size,
            },
        };

        // Iterate per-pixel.
        let len = blit_area.size.x * blit_area.size.y;
        let src_offset = self
            .surface
            .get_index(blit_area.position.x, blit_area.position.y);
        let dst_offset = other.get_index(destination_rect.position.x, destination_rect.position.y);
        for i in 0..len {
            let src_index = src_offset + i;
            if self.mask[src_index] {
                other.buffer[dst_offset + i] = self.surface.buffer[src_index];
            }
        }
        Ok(())
    }
}

#[cfg(feature = "png")]
#[cfg(test)]
mod tests {
    use super::*;
    use glam::I64Vec2;
    use std::env::current_dir;

    const SRC_W: usize = 32;
    const SRC_H: usize = 17;
    const DST_W: usize = 64;
    const DST_H: usize = 64;

    #[test]
    fn test_blit_mask() {
        let position = I64Vec2 { x: 2, y: 12 };
        let src_size = USizeVec2 { x: SRC_W, y: SRC_H };
        let mut dst = Surface::new_filled(USizeVec2 { x: DST_W, y: DST_H }, [0u8, 0, 0]);

        let src = Surface::new_filled(src_size, [255u8, 255, 255])
            .position(position, &dst)
            .unwrap();
        let mut src = MaskedSurface::new(src);
        for m in src.mask_mut().chunks_exact_mut(2) {
            m[0] = true;
        }

        src.blit(&mut dst).unwrap();

        dst.write_png(current_dir().unwrap().join("test_output/mask.png"))
            .unwrap();
    }
}
