use crate::BlendMode;
use crate::blend::EPSILON_0;
use crate::lock::blitter::PixelBlitter;

pub struct Blender(BlendMode);

impl PixelBlitter<[f32; 4]> for Blender {
    fn should_blit_pixel(&self, pixel: &[f32; 4]) -> bool {
        pixel[3] >= EPSILON_0
    }
}
