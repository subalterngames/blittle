mod blend_mode;
mod locker;

use crate::Surface;
pub use blend_mode::BlendMode;
use crate::blend::locker::BlendLocker;
use crate::lock::LockableSurface;

/// A hacky optimization.
/// We assume that we're converting to and from pixels with 8-bit channels.
/// So, this value is 254. / 255.
/// This will (hopefully!) help with floating point precision.
const EPSILON_255: f32 = 0.9960784;
// Likewise, this is 1. / 255.
const EPSILON_0: f32 = 0.0039216;

pub type BlendableSurface<'s,
    S: AsRef<[[f32; 4]]> + AsMut<[[f32; 4]]>,> = LockableSurface<'s, S, [f32; 4], BlendLocker>;

impl<'s, S: AsRef<[[f32; 4]]> + AsMut<[[f32; 4]]>> BlendableSurface<'s, S> {
    pub fn new(surface: Surface<'s, S, [f32; 4]>) -> Self {
        Self {
            surface,
            locker: BlendLocker::default(),
            #[cfg(feature = "std")]
            mask: None,
        }
    }
}
