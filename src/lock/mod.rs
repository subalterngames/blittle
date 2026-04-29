pub(crate) mod blitter;
mod indices;

use crate::error::Error;
use crate::{RectU, Surface};
use blitter::PixelBlitter;
pub(crate) use indices::LockedIndices;

pub struct LockableSurface<
    's,
    S: AsRef<[P]> + AsMut<[P]>,
    P: Copy + Clone + Sized + Default,
    #[cfg(feature = "std")] L: PixelBlitter<P>,
> {
    pub(crate) surface: Surface<'s, S, P>,
    pub(crate) blitter: L,
    #[cfg(feature = "std")]
    pub(crate) mask: Option<Vec<LockedIndices>>,
}

impl<
    's,
    S: AsRef<[P]> + AsMut<[P]>,
    P: Copy + Clone + Sized + Default,
    #[cfg(feature = "std")] L: PixelBlitter<P>,
> LockableSurface<'s, S, P, L>
{
    /// Lock the surface, optimizing blit speed while preventing pixel manipulation.
    #[cfg(feature = "std")]
    pub fn lock(&mut self) {
        if self.is_locked() {
            return;
        }
        // Get the top-left and bottom-right coordinates of the blit area.
        let (x0, x1, y0, y1) = match self.surface.blit_area {
            Some(blit_area) => (
                blit_area.position.x,
                blit_area.position.x + blit_area.size.width,
                blit_area.position.y,
                blit_area.position.y + blit_area.size.height,
            ),
            None => (0, self.surface.size.width, 0, self.surface.size.height),
        };
        self.mask = {
            let mut mask = vec![];
            // Iterate through the blit area.
            for y in y0..y1 {
                let i0 = self.surface.get_index(x0, y);
                let i1 = self.surface.get_index(x1, y);
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
                self.blitter.blit_pixel(
                    self.surface.buffer.as_ref()[i],
                    &mut other.buffer_mut()[dst_offset + i],
                )
            }
            LockedIndices::Row { start, end } => {
                let i0 = *start;
                let i1 = *end;
                self.blitter.blit_row::<B>(
                    &self.surface.buffer.as_ref()[i0..i1],
                    &mut other.buffer.as_mut()[dst_offset + i0..dst_offset + i1],
                );
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
            if self
                .blitter
                .should_blit_pixel(&self.surface.buffer.as_ref()[src_index])
            {
                self.blitter.blit_pixel(
                    self.surface.buffer.as_ref()[src_index],
                    &mut other.buffer_mut()[dst_offset + i],
                )
            }
        }
    }
}
