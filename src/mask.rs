use bytemuck::{Pod, Zeroable};
use crate::error::Error;
use crate::{RectU, Surface};
use glam::USizeVec2;

enum Mask {
    Pixel(usize),
    Row { i0: usize, i1: usize },
}

/// A surface that uses a pixel color as a mask.
/// Pixels of the mask color will not be copied to the destination surface.
///
/// Typically, this is used for RGB pixels.
///
/// MaskedSurfaces can be locked or unlocked.
/// If the surface is locked, then pixels can't be manipulated, but blit speed is optimized.
pub struct MaskedSurface<'s, S: AsRef<[P]> + AsMut<[P]>, P: Copy + Clone + Sized + Default + Zeroable + Pod + Eq + PartialEq> {
    surface: Surface<'s, S, P>,
    mask_color: P,
    mask: Option<Vec<Mask>>,
}

impl<'s, S: AsRef<[P]> + AsMut<[P]>, P: Copy + Clone + Sized + Default + Zeroable + Pod + Eq + PartialEq> MaskedSurface<'s, S, P> {
    pub const fn new(surface: Surface<'s, S, P>, mask_color: P) -> Self {
        Self {
            surface,
            mask_color,
            mask: None,
        }
    }

    /// Set the color of the mask. Returns an error if the surface is locked.
    pub const fn set_mask_color(&mut self, mask_color: P) -> Result<(), Error> {
        if self.is_locked() {
            Err(Error::Locked)
        } else {
            self.mask_color = mask_color;
            Ok(())
        }
    }

    /// Lock the surface, optimizing for blit speed while preventing pixel manipulation.
    pub fn lock(&mut self) {
        if self.is_locked() {
            return;
        }
        // Get the top-left and bottom-right coordinates of the blit area.
        let (x0, x1, y0, y1) = match self.surface.blit_area {
            Some(blit_area) => (
                blit_area.position.x,
                blit_area.position.x + blit_area.size.x,
                blit_area.position.y,
                blit_area.position.y + blit_area.size.y,
            ),
            None => (0, self.surface.size.x, 0, self.surface.size.y),
        };
        self.mask = {
            let mut mask = vec![];
            // Iterate through the blit area.
            for y in y0..y1 {
                let i0 = self.surface.get_index(x0, y);
                let i1 = self.surface.get_index(x1, y);
                if self.surface.buffer.as_ref()[i0..i1]
                    .iter()
                    .all(|p| *p != self.mask_color)
                {
                    // Remember the entire row.
                    mask.push(Mask::Row { i0, i1 })
                } else {
                    // Remember each unmasked pixel.
                    mask.extend((i0..i1).filter_map(|i| {
                        if self.surface.buffer.as_ref()[i] != self.mask_color {
                            Some(Mask::Pixel(i))
                        } else {
                            None
                        }
                    }))
                }
            }
            Some(mask)
        };
    }

    /// Returns true if the surface is locked.
    pub const fn is_locked(&self) -> bool {
        self.mask.is_some()
    }

    /// Unlock the surface.
    /// Blit speed will be unoptimized while pixel manipulation will be permitted.
    pub fn unlock(&mut self) {
        self.mask = None;
    }

    /// Returns a reference of the surface.
    ///
    /// Note that `self.blit(&mut destination)` is not the same as `self.surface.blit(&mut destination)`
    /// because the latter won't apply the mask.
    pub const fn surface(&self) -> &Surface<'s, S, P> {
        &self.surface
    }

    /// Returns a mutable reference of the surface.
    /// Returns an error if the masked surface is locked.
    pub const fn surface_mut(&mut self) -> Result<&mut Surface<'s, S, P>, Error> {
        if self.is_locked() {
            Err(Error::Locked)
        } else {
            Ok(&mut self.surface)
        }
    }

    /// Blit onto `other`, using a mask.
    ///
    /// This can be called if this masked surface is unlocked, but it'll be slower.
    pub fn blit(&self, other: &mut Surface<'s, S, P>) -> Result<(), Error> {
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
        let dst_offset = other.get_index(destination_rect.position.x, destination_rect.position.y);
        match self.mask.as_ref() {
            Some(mask) => {
                mask.iter().for_each(|m| match m {
                    Mask::Pixel(i) => {
                        let i = *i;
                        other.buffer.as_mut()[dst_offset + i] = self.surface.buffer.as_ref()[i];
                    }
                    Mask::Row { i0, i1 } => {
                        let i0 = *i0;
                        let i1 = *i1;
                        other.buffer.as_mut()[dst_offset + i0..dst_offset + i1]
                            .copy_from_slice(&self.surface.buffer.as_ref()[i0..i1])
                    }
                });
            }
            None => {
                // Iterate per-pixel.
                let len = blit_area.size.x * blit_area.size.y;
                let src_offset = self
                    .surface
                    .get_index(blit_area.position.x, blit_area.position.y);
                for i in 0..len {
                    let src_index = src_offset + i;
                    if self.surface.buffer.as_ref()[src_index] != self.mask_color {
                        other.buffer.as_mut()[dst_offset + i] = self.surface.buffer.as_ref()[src_index];
                    }
                }
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

        let src_color = [255u8, 255, 255];
        let mask_color = [255, 0, 255];
        let mut src = Surface::new_filled(src_size, src_color);
        src.set_position(position, &dst).unwrap();
        for pixel in src.buffer.chunks_exact_mut(3) {
            pixel[0] = mask_color;
        }
        let mut src = MaskedSurface::new(src, mask_color);

        src.blit(&mut dst).unwrap();

        dst.write_png(current_dir().unwrap().join("test_output/mask_unlocked.png"))
            .unwrap();

        // Lock.
        src.lock();
        src.blit(&mut dst).unwrap();

        dst.write_png(current_dir().unwrap().join("test_output/mask_locked.png"))
            .unwrap();
    }
}
