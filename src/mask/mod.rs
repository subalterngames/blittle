#[cfg(feature = "std")]
mod indices;
#[cfg(feature = "std")]
use crate::lock::get_dst_index;
use crate::lock::get_indices;
use crate::{Error, RectU, Surface};
#[cfg(feature = "std")]
use indices::LockedIndices;

/// A surface with a mask color. Pixels of the mask color will not blit to the destination.
///
/// A MaskedSurface can be locked or unlocked.
/// If locked, the surface can't be mutated, but blitting will be faster.
pub struct MaskedSurface<'s, S: AsRef<[P]> + AsMut<[P]>, P: Copy + Clone + Sized + Default> {
    surface: Surface<'s, S, P>,
    mask_color: P,
    #[cfg(feature = "std")]
    mask: Option<Vec<LockedIndices>>,
}

impl<'s, S: AsRef<[P]> + AsMut<[P]>, P: Copy + Clone + Sized + Default + Eq + PartialEq>
    MaskedSurface<'s, S, P>
{
    pub const fn new(surface: Surface<'s, S, P>, mask_color: P) -> Self {
        Self {
            surface,
            mask_color,
            #[cfg(feature = "std")]
            mask: None,
        }
    }

    /// Set the color of the mask. Returns an error if the surface is locked.
    pub const fn set_mask_color(&mut self, mask_color: P) -> Result<(), Error> {
        #[cfg(feature = "std")]
        if self.is_locked() {
            Err(Error::Locked)
        } else {
            self.mask_color = mask_color;
            Ok(())
        }
        #[cfg(not(feature = "std"))]
        {
            self.mask_color = mask_color;
            Ok(())
        }
    }

    /// Lock the surface, optimizing blit speed while preventing pixel manipulation.
    #[cfg(feature = "std")]
    pub fn lock(&mut self) {
        if self.is_locked() {
            return;
        }

        self.mask = {
            let mut mask = vec![];
            // Iterate through the blit area.
            for y in 0..self.surface.size.height {
                let i0 = self.surface.get_index(0, y);
                let i1 = i0 + self.surface.size.width;
                if self.surface.buffer.as_ref()[i0..i1]
                    .iter()
                    .all(|p| self.should_blit_pixel(*p))
                {
                    // Remember the entire row.
                    mask.push(LockedIndices::Row { start: i0, end: i1 })
                } else {
                    // Remember each unmasked pixel.
                    mask.extend((i0..i1).filter_map(|i| {
                        if self.should_blit_pixel(self.surface.buffer.as_ref()[i]) {
                            Some(LockedIndices::Pixel(i))
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
    #[cfg(feature = "std")]
    pub const fn is_locked(&self) -> bool {
        self.mask.is_some()
    }

    /// Unlock the surface.
    /// Blit speed will be unoptimized while pixel manipulation will be permitted.
    #[cfg(feature = "std")]
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
        #[cfg(feature = "std")]
        if self.is_locked() {
            Err(Error::Locked)
        } else {
            Ok(&mut self.surface)
        }
        #[cfg(not(feature = "std"))]
        Ok(&mut self.surface)
    }

    /// Blit onto `other`.
    ///
    /// This can be called if this masked surface is unlocked, but it'll be slower.
    pub fn blit<B: AsRef<[P]> + AsMut<[P]>>(
        &self,
        other: &mut Surface<'s, B, P>,
    ) -> Result<(), Error> {
        let (destination_rect, blit_area) = self.surface.get_blit_params(other.size)?;
        #[cfg(feature = "std")]
        match self.mask.as_ref() {
            Some(mask) => {
                self.blit_locked(destination_rect, mask, other);
            }
            None => {
                self.blit_unlocked(destination_rect, blit_area, other);
            }
        }
        #[cfg(not(feature = "std"))]
        self.blit_unlocked(destination_rect, blit_area, other);
        Ok(())
    }

    #[cfg(feature = "std")]
    fn blit_locked<B: AsRef<[P]> + AsMut<[P]>>(
        &self,
        destination_rect: RectU,
        mask: &[LockedIndices],
        other: &mut Surface<'_, B, P>,
    ) {
        let dst_len = other.buffer().len();
        mask.iter().for_each(|m| match m {
            LockedIndices::Pixel(i) => {
                let i = *i;
                if let Some(dst_index) =
                    get_dst_index(i, &destination_rect, dst_len, &self.surface, other)
                {
                    self.blit_pixel(
                        self.surface.buffer.as_ref()[i],
                        &mut other.buffer_mut()[dst_index],
                    );
                }
            }
            LockedIndices::Row { start, end } => {
                let src_i0 = *start;
                let mut src_i1 = *end;
                if let Some(dst_i0) =
                    get_dst_index(src_i0, &destination_rect, dst_len, &self.surface, other)
                {
                    // Get the end index of the row.
                    let mut dst_i1 = dst_i0 + (src_i1 - src_i0);
                    // The end index is out of bounds.
                    if dst_i1 >= dst_len {
                        // Clamp to the length of the destination buffer.
                        dst_i1 = dst_len;
                        // Set the end of the source row by the new offset.
                        src_i1 = src_i0 + (dst_i1 - dst_i0);
                    }
                    other.buffer.as_mut()[dst_i0..dst_i1]
                        .copy_from_slice(&self.surface.buffer.as_ref()[src_i0..src_i1]);
                }
            }
        });
    }

    fn blit_unlocked<B: AsRef<[P]> + AsMut<[P]>>(
        &self,
        destination_rect: RectU,
        blit_area: RectU,
        other: &mut Surface<'_, B, P>,
    ) {
        (0..blit_area.size.height).for_each(|y| {
            (0..blit_area.size.width).for_each(|x| {
                let (src_index, dst_index) =
                    get_indices(x, y, &destination_rect, &blit_area, &self.surface, other);
                if self.should_blit_pixel(self.surface.buffer.as_ref()[src_index]) {
                    self.blit_pixel(
                        self.surface.buffer.as_ref()[src_index],
                        &mut other.buffer_mut()[dst_index],
                    );
                }
            })
        });
    }

    fn should_blit_pixel(&self, pixel: P) -> bool {
        pixel != self.mask_color
    }

    fn blit_pixel(&self, top: P, bottom: &mut P) {
        *bottom = top;
    }
}

#[cfg(feature = "png")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::png::Png;
    use crate::{PositionI, Rgb8Surface, Size};
    use std::env::current_dir;

    const SRC_W: usize = 32;
    const SRC_H: usize = 17;
    const DST_W: usize = 64;
    const DST_H: usize = 64;

    #[test]
    fn test_blit_mask() {
        let position = PositionI { x: 2, y: 12 };
        let src_size = Size {
            width: SRC_W,
            height: SRC_H,
        };
        let mut dst = Surface::new_filled(
            Size {
                width: DST_W,
                height: DST_H,
            },
            [0u8, 0, 0],
        );

        let src_color = [255u8, 255, 255];
        let mask_color = [255, 0, 255];
        let mut src = Surface::new_filled(src_size, src_color);
        src.set_position(position, &dst).unwrap();
        for pixel in src.buffer.chunks_exact_mut(3) {
            pixel[0] = mask_color;
        }
        let mut src = MaskedSurface::new(src, mask_color);

        src.blit(&mut dst).unwrap();

        Rgb8Surface::write_png(
            &dst,
            current_dir()
                .unwrap()
                .join("test_output")
                .join("mask_unlocked.png"),
        )
        .unwrap();

        // Lock.
        src.lock();
        src.blit(&mut dst).unwrap();
        Rgb8Surface::write_png(
            &dst,
            current_dir()
                .unwrap()
                .join("test_output")
                .join("mask_locked.png"),
        )
        .unwrap();
    }
}
