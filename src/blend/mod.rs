mod blend;
mod blitter;
mod blend_mode;

use crate::Surface;
use crate::blend::blitter::Blender;
use crate::lock::LockableSurface;
pub use blend_mode::BlendMode;

pub type BlendableSurface<'s, S: AsRef<[[f32; 4]]> + AsMut<[[f32; 4]]>> =
    LockableSurface<'s, S, [f32; 4], Blender>;

impl<'s, S: AsRef<[[f32; 4]]> + AsMut<[[f32; 4]]>> BlendableSurface<'s, S> {
    pub fn new(surface: Surface<'s, S, [f32; 4]>) -> Self {
        Self {
            surface,
            blitter: Blender::default(),
            #[cfg(feature = "std")]
            mask: None,
        }
    }
}
