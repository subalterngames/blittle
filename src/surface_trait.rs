use crate::{Error, RectU};
use bytemuck::{Pod, Zeroable, cast_slice, cast_slice_mut};
use glam::USizeVec2;

pub trait SurfaceTrait<P: Copy + Clone + Sized + Default + Zeroable + Pod> {
    fn get_size(&self) -> USizeVec2;

    /// The underlying pixel buffer.
    fn buffer(&self) -> &[P];

    fn buffer_mut(&mut self) -> &mut [P];

    /// The underlying buffer as bytes.
    fn bytes(&self) -> &[u8] {
        cast_slice::<P, u8>(self.buffer())
    }

    /// The underlying mutable buffer as bytes.
    fn bytes_mut(&mut self) -> &mut [u8] {
        cast_slice_mut::<P, u8>(self.buffer_mut())
    }

    fn get_blit_params(&self) -> Result<(RectU, RectU), Error>;
}
