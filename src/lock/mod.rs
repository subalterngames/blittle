mod indices;
pub(crate) mod locker;

use crate::error::Error;
use crate::{RectU, Surface};
pub(crate) use indices::LockedIndices;
use locker::PixelLocker;

pub struct LockableSurface<
    's,
    S: AsRef<[P]> + AsMut<[P]>,
    P: Copy + Clone + Sized + Default,
    #[cfg(feature = "std")]
    L: PixelLocker<P>
> {
    pub(crate) surface: Surface<'s, S, P>,
    pub(crate) locker: L,
    #[cfg(feature = "std")]
    pub(crate) mask: Option<Vec<LockedIndices>>,
}

impl<
    's,
    S: AsRef<[P]> + AsMut<[P]>,
    P: Copy + Clone + Sized + Default + Eq + PartialEq,
    #[cfg(feature = "std")]
    L: PixelLocker<P>
> LockableSurface<'s, S, P, L> {
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
                    .all(|p| self.locker.should_blit_pixel(p))
                {
                    // Remember the entire row.
                    mask.push(LockedIndices::Row { start: i0, end: i1 })
                } else {
                    // Remember each unmasked pixel.
                    mask.extend((i0..i1).filter_map(|i| {
                        if self.locker.should_blit_pixel(&self.surface.buffer.as_ref()[i]) {
                            Some(LockedIndices::Pixel(i))
                        }
                        else {
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
}