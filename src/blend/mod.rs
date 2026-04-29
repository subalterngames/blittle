mod blend_mode;

use crate::Surface;
pub use blend_mode::BlendMode;

/// A hacky optimization.
/// We assume that we're converting to and from pixels with 8-bit channels.
/// So, this value is 254. / 255.
/// This will (hopefully!) help with floating point precision.
const EPSILON_255: f32 = 0.9960784;
// Likewise, this is 1. / 255.
const EPSILON_0: f32 = 0.0039216;

#[cfg(feature = "std")]
enum Mask {
    Pixel(usize),
    Row { i0: usize, i1: usize },
}

pub struct BlendSurface<'s, S: AsRef<[[f32; 4]]> + AsMut<[[f32; 4]]>>{
    surface: Surface<'s, S, [f32; 4]>,
    #[cfg(feature = "std")]
    mask: Option<Vec<Mask>>,
}

impl<'s, S: AsRef<[[f32; 4]]> + AsMut<[[f32; 4]]>> BlendSurface<'s, S> {
    pub const fn new(surface: Surface<'s, S, [f32; 4]>) -> Self {
        Self {
            surface,
            #[cfg(feature = "std")]
            mask: None,
        }
    }

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
                    .all(|p| p[3] < EPSILON_0)
                {
                    // Remember the entire row.
                    mask.push(Mask::Row { i0, i1 })
                } else {
                    // Remember each unmasked pixel.
                    mask.extend((i0..i1).filter_map(|i| {
                        if self.surface.buffer.as_ref()[i][3] < EPSILON_0 {
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
}

impl<'s, S: AsRef<[[f32; 4]]> + AsMut<[[f32; 4]]>> Surface<'s, S, [f32; 4]> {
    pub fn blend<B: AsRef<[[f32; 4]]> + AsMut<[[f32; 4]]>>(&self, other: &mut Surface<'s, B, [f32; 4]>) {

    }
}