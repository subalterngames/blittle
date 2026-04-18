use std::ops::DerefMut;
use bytemuck::cast_slice_mut;
use glam::USizeVec2;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use crate::{Rgba8Surface, Surface, SurfaceRef};
use softbuffer::Buffer;



impl Rgba8Surface {
    /// View this surface as a mutable ZRGB surface reference.
    /// Useful for integration with [softbuffer](https://docs.rs/softbuffer/latest/softbuffer/).
    ///
    /// NOTE: Setting the position, area, etc. will NOT modify the original surface.
    pub fn as_zrgb_ref(&mut self) -> SurfaceRef<'_, u32> {
        SurfaceRef {
            size: self.size,
            buffer: cast_slice_mut::<[u8; 4], u32>(&mut self.buffer),
            destination_rect: self.destination_rect,
            blit_area: self.blit_area,
        }
    }
}

impl<'s> SurfaceRef<'s, u32> {
    pub fn from_softbuffer<D: HasDisplayHandle, W: HasWindowHandle>(buffer: &'s mut Buffer<D, W>) -> Self {
        let size = USizeVec2 {
            x: buffer.width().get() as usize,
            y: buffer.height().get() as usize,
        };
        let buffer = buffer.deref_mut();
        Self {
            size,
            buffer,
            destination_rect: None,
            blit_area: None
        }
    }
}

impl<'s> SurfaceRef<'s, [u8; 4]> {
    pub fn from_softbuffer<D: HasDisplayHandle, W: HasWindowHandle>(buffer: &'s mut Buffer<D, W>) -> Self {
        let size = USizeVec2 {
            x: buffer.width().get() as usize,
            y: buffer.height().get() as usize,
        };
        let buffer = cast_slice_mut::<u32, [u8; 4]>(buffer.deref_mut());
        Self {
            size,
            buffer,
            destination_rect: None,
            blit_area: None
        }
    }
} 
