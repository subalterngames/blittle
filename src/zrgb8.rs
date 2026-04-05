use crate::{Rgba8Surface, SurfaceRef, Zrgb8Surface};
use bytemuck::cast_slice_mut;

impl Zrgb8Surface {
    /// View this ZRGB surface as a mutable RGBA surface reference.
    /// This is useful for setting per-pixel color values.
    ///
    /// NOTE: that the returned value does *not* contain RGBA values!
    /// This is because ZRGB is meant to be used with [softbuffer](https://docs.rs/softbuffer/latest/softbuffer/).
    /// The pixel layout is: `[z, r, g, b]` where `z` is always 0.
    ///
    /// NOTE: Setting the position, area, etc. will NOT modify the original surface.
    pub fn as_rgba_ref(&mut self) -> SurfaceRef<'_, [u8; 4]> {
        SurfaceRef {
            rect: self.rect,
            buffer: cast_slice_mut::<u32, [u8; 4]>(&mut self.buffer),
            destination_rect: self.destination_rect,
            blit_area: self.blit_area,
        }
    }
}

impl Rgba8Surface {
    /// View this surface as a mutable ZRGB surface reference.
    /// Useful for integration with [softbuffer](https://docs.rs/softbuffer/latest/softbuffer/).
    ///
    /// NOTE: Setting the position, area, etc. will NOT modify the original surface.
    pub fn as_zrgb_ref(&mut self) -> SurfaceRef<'_, u32> {
        SurfaceRef {
            rect: self.rect,
            buffer: cast_slice_mut::<[u8; 4], u32>(&mut self.buffer),
            destination_rect: self.destination_rect,
            blit_area: self.blit_area,
        }
    }
}
