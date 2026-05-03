use crate::blend::Rgb;
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
    f: fn(top: &[f32; 4], bottom: &[f32; 4]) -> Rgb,
    /// The alpha channel.
    alpha: f32,
}

impl Blender {
    pub const fn new(f: fn(top: &[f32; 4], bottom: &[f32; 4]) -> Rgb, alpha: f32) -> Self {
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
        const fn lerp(a: f32, b: f32, t: f32) -> f32 {
            if b > a {
                a + t * (b - a)
            }
            else {
                b + t * (b - a)
            }
        }

        let a = top[3];
        if a >= EPSILON_0 && self.alpha >= EPSILON_0 {
            // Blend.
            let rgb = (self.f)(&top, bottom);
            // Blend with composited alpha.
            let ca = (a + bottom[3] * (1. - a)) * self.alpha;
            if ca >= EPSILON_255 {
                bottom[0] = rgb.r;
                bottom[1] = rgb.g;
                bottom[2] = rgb.b;
            }
            else {
                bottom[0] = lerp(top[0], rgb.r, ca);
                bottom[1] = lerp(top[1], rgb.g, ca);
                bottom[2] = lerp(top[2], rgb.b, ca);
            }
        }
    }
}
