mod zrgb;

use crate::{Error, Size, Surface};
use bytemuck::cast_slice_mut;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use softbuffer::Buffer;
use std::marker::PhantomData;
use std::ops::DerefMut;
pub use zrgb::Zrgb;

/// Red, green, blue as [Zrgb] pixels, referencing a [Buffer].
pub type SoftbufferSurface<'s> = Surface<'s, &'s mut [Zrgb], Zrgb>;

impl<'s> SoftbufferSurface<'s> {
    /// Convert a softbuffer [Buffer] to a surface.
    ///
    /// The underlying pixel buffer is a mutable reference to `buffer`.
    pub fn from_softbuffer<D: HasDisplayHandle, W: HasWindowHandle>(
        buffer: &'s mut Buffer<D, W>,
    ) -> Self {
        let size = Size::new(
            buffer.width().get() as usize,
            buffer.height().get() as usize,
        );
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

impl<'s, S: AsRef<[Zrgb]> + AsMut<[Zrgb]>> Surface<'s, S, Zrgb> {
    /// Blit to a softbuffer [Buffer].
    pub fn blit_to_softbuffer<D: HasDisplayHandle, W: HasWindowHandle>(
        &self,
        buffer: &'s mut Buffer<D, W>,
    ) -> Result<(), Error> {
        let mut destination = SoftbufferSurface::from_softbuffer(buffer);
        self.blit(&mut destination)
    }
}
