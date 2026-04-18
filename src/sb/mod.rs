mod from;
mod to;
mod zrgb;

use crate::Surface;
use bytemuck::cast_slice_mut;
pub use from::FromZrgb;
use glam::USizeVec2;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use softbuffer::Buffer;
use std::marker::PhantomData;
use std::ops::DerefMut;
pub use to::ToZrgb;
pub use zrgb::Zrgb;

/// Red, green, blue as [Zrgb] pixels, referencing a [Buffer].
pub type ZrgbSurfaceRef<'s> = Surface<'s, &'s mut [Zrgb], Zrgb>;

impl<'s> ZrgbSurfaceRef<'s> {
    pub fn from_softbuffer<D: HasDisplayHandle, W: HasWindowHandle>(
        buffer: &'s mut Buffer<D, W>,
    ) -> Self {
        let size = USizeVec2 {
            x: buffer.width().get() as usize,
            y: buffer.height().get() as usize,
        };
        let buffer = cast_slice_mut::<u32, Zrgb>(buffer.deref_mut());
        Self {
            size,
            buffer,
            destination_rect: None,
            blit_area: None,
            _p: PhantomData,
        }
    }
}
