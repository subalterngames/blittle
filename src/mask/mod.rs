mod blitter;

use crate::lock::LockableSurface;
use crate::{Error, Surface};
use blitter::MaskBlitter;

/// A surface with a mask color. Pixels of the mask color will not blit to the destination.
///
/// A MaskedSurface can be locked or unlocked.
/// If locked, the surface can't be mutated, but blitting will be faster.
pub type MaskedSurface<'s, S, P> = LockableSurface<'s, S, P, MaskBlitter<P>>;

impl<'s, S: AsRef<[P]> + AsMut<[P]>, P: Copy + Clone + Sized + Default + Eq + PartialEq>
    MaskedSurface<'s, S, P>
{
    pub const fn new(surface: Surface<'s, S, P>, mask_color: P) -> Self {
        Self {
            surface,
            blitter: MaskBlitter::new(mask_color),
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
            self.blitter = MaskBlitter::new(mask_color);
            Ok(())
        }
        #[cfg(not(feature = "std"))]
        {
            self.mask_color = mask_color;
            Ok(())
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
