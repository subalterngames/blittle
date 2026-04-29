mod locker;

use crate::lock::{LockableSurface, LockedIndices};
use locker::MaskLocker;
use crate::{Error, RectU, Surface};
use crate::lock::locker::PixelLocker;

pub type MaskedSurface<'s,
    S: AsRef<[P]> + AsMut<[P]>,
    P: Copy + Clone + Sized + Default + Eq + PartialEq,> = LockableSurface<'s, S, P, MaskLocker<P>>;

impl<'s, S: AsRef<[P]> + AsMut<[P]>,
    P: Copy + Clone + Sized + Default + Eq + PartialEq,> MaskedSurface<'s, S, P> {
    pub const fn new(surface: Surface<'s, S, P>, mask_color: P) -> Self {
        Self {
            surface,
            locker: MaskLocker::new(mask_color),
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
            self.locker = MaskLocker::new(mask_color);
            Ok(())
        }
        #[cfg(not(feature = "std"))]
        {
            self.mask_color = mask_color;
            Ok(())
        }
    }

    /// Blit onto `other`, using a mask.
    ///
    /// This can be called if this masked surface is unlocked, but it'll be slower.
    pub fn blit<B: AsRef<[P]> + AsMut<[P]>>(
        &self,
        other: &mut Surface<'s, B, P>,
    ) -> Result<(), Error> {
        let (destination_rect, blit_area) = self.surface.get_blit_params(other.size)?;
        let dst_offset = other.get_index(destination_rect.position.x, destination_rect.position.y);
        #[cfg(feature = "std")]
        match self.mask.as_ref() {
            Some(mask) => {
                self.blit_locked(mask, dst_offset, other);
            }
            None => {
                self.blit_unlocked(blit_area, dst_offset, other);
            }
        }
        #[cfg(not(feature = "std"))]
        self.blit_unlocked(blit_area, dst_offset, other);
        Ok(())
    }

    fn blit_locked<B: AsRef<[P]> + AsMut<[P]>>(
        &self,
        mask: &[LockedIndices],
        dst_offset: usize,
        other: &mut Surface<'_, B, P>,
    ) {
        mask.iter().for_each(|m| match m {
            LockedIndices::Pixel(i) => {
                let i = *i;
                other.buffer.as_mut()[dst_offset + i] = self.surface.buffer.as_ref()[i];
            }
            LockedIndices::Row { start, end } => {
                let i0 = *start;
                let i1 = *end;
                other.buffer.as_mut()[dst_offset + i0..dst_offset + i1]
                    .copy_from_slice(&self.surface.buffer.as_ref()[i0..i1])
            }
        });
    }

    fn blit_unlocked<B: AsRef<[P]> + AsMut<[P]>>(
        &self,
        blit_area: RectU,
        dst_offset: usize,
        other: &mut Surface<'_, B, P>,
    ) {
        // Iterate per-pixel.
        let len = blit_area.size.width * blit_area.size.height;
        let src_offset = self
            .surface
            .get_index(blit_area.position.x, blit_area.position.y);
        for i in 0..len {
            let src_index = src_offset + i;
            if self.locker.should_blit_pixel(&self.surface.buffer.as_ref()[src_index]) {
                other.buffer.as_mut()[dst_offset + i] = self.surface.buffer.as_ref()[src_index];
            }
        }
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
