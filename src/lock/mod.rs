pub(crate) mod blitter;
#[cfg(feature = "std")]
mod indices;

use crate::error::Error;
use crate::{RectU, Surface};
use blitter::PixelBlitter;
#[cfg(feature = "std")]
pub(crate) use indices::LockedIndices;

pub struct LockableSurface<
    's,
    S: AsRef<[P]> + AsMut<[P]>,
    P: Copy + Clone + Sized + Default,
    L: PixelBlitter<P>,
> {
    pub(crate) surface: Surface<'s, S, P>,
    pub(crate) blitter: L,
    #[cfg(feature = "std")]
    pub(crate) mask: Option<Vec<LockedIndices>>,
}

impl<'s, S: AsRef<[P]> + AsMut<[P]>, P: Copy + Clone + Sized + Default, L: PixelBlitter<P>>
    LockableSurface<'s, S, P, L>
{
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
                    .all(|p| self.blitter.should_blit_pixel(p))
                {
                    // Remember the entire row.
                    mask.push(LockedIndices::Row { start: i0, end: i1 })
                } else {
                    // Remember each unmasked pixel.
                    mask.extend((i0..i1).filter_map(|i| {
                        if self
                            .blitter
                            .should_blit_pixel(&self.surface.buffer.as_ref()[i])
                        {
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
                if let Some(dst_index) = self.get_dst_index(i, &destination_rect, dst_len, other) {
                    self.blitter.blit_pixel(
                        self.surface.buffer.as_ref()[i],
                        &mut other.buffer_mut()[dst_index],
                    );
                }
            }
            LockedIndices::Row { start, end } => {
                let src_i0 = *start;
                let mut src_i1 = *end;
                if let Some(dst_i0) = self.get_dst_index(src_i0, &destination_rect, dst_len, other)
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
                    self.blitter.blit_row::<B>(
                        &self.surface.buffer.as_ref()[src_i0..src_i1],
                        &mut other.buffer.as_mut()[dst_i0..dst_i1],
                    );
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
                    self.get_indices(x, y, &destination_rect, &blit_area, other);
                if self
                    .blitter
                    .should_blit_pixel(&self.surface.buffer.as_ref()[src_index])
                {
                    self.blitter.blit_pixel(
                        self.surface.buffer.as_ref()[src_index],
                        &mut other.buffer_mut()[dst_index],
                    );
                }
            })
        });
    }

    const fn get_indices<B: AsRef<[P]> + AsMut<[P]>>(
        &self,
        x: usize,
        y: usize,
        destination_rect: &RectU,
        blit_area: &RectU,
        other: &Surface<'_, B, P>,
    ) -> (usize, usize) {
        // Get the start index in the source slice.
        let src_index = self.surface.get_index(
            blit_area.position.x + x, // Blit area offset or zero
            y + blit_area.position.y, // y offset + blit area offset
        );
        let dst_index = other.get_index(
            destination_rect.position.x + x, // Destination position (x)
            y + destination_rect.position.y, // y offset + destination position (y)
        );
        (src_index, dst_index)
    }

    #[cfg(feature = "std")]
    const fn get_dst_index<B: AsRef<[P]> + AsMut<[P]>>(
        &self,
        i: usize,
        destination_rect: &RectU,
        dst_len: usize,
        other: &Surface<'_, B, P>,
    ) -> Option<usize> {
        let src_x = i % self.surface.size.width;
        let src_y = i / self.surface.size.width;
        let dst_index = other.get_index(
            destination_rect.position.x + src_x, // Destination position (x)
            src_y + destination_rect.position.y, // y offset + destination position (y)
        );
        if dst_index < dst_len {
            Some(dst_index)
        } else {
            None
        }
    }
}
