use crate::lock::blitter::PixelBlitter;

/// A hacky optimization.
/// We assume that we're converting to and from pixels with 8-bit channels.
/// So, this value is 254. / 255.
/// This will (hopefully!) help with floating point precision.
const EPSILON_255: f32 = 0.9960784;
// Likewise, this is 1. / 255.
const EPSILON_0: f32 = 0.0039216;

/// Blend two pixels.
pub struct Blender {
    /// The blend function.
    f: fn(top: [f32; 4], bottom: &mut [f32; 4]),
    /// The alpha channel.
    alpha: f32,
}

impl Blender {
    pub const fn new(f: fn(top: [f32; 4], bottom: &mut [f32; 4]), alpha: f32) -> Self {
        Self { f, alpha }
    }
}

impl PixelBlitter<[f32; 4]> for Blender {
    fn should_blit_pixel(&self, pixel: &[f32; 4]) -> bool {
        pixel[3] >= EPSILON_0
    }

    fn blit_row<B: AsRef<[[f32; 4]]> + AsMut<[[f32; 4]]>>(
        &self,
        top: &[[f32; 4]],
        bottom: &mut [[f32; 4]],
    ) {
        top.iter()
            .zip(bottom.iter_mut())
            .for_each(|(top, bottom)| self.blit_pixel(*top, bottom));
    }

    fn blit_pixel(&self, top: [f32; 4], bottom: &mut [f32; 4]) {
        let a = top[3];
        if a > EPSILON_255 && self.alpha > EPSILON_255 {
            // Blit.
            *bottom = top;
        } else if a >= EPSILON_0 && self.alpha >= EPSILON_0 {
            // Blend.
            (self.f)(top, bottom);
            // Composited alpha.
            bottom[3] = a + bottom[3] * self.alpha * (1. - a);
        }
    }
}
