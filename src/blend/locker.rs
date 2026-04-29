use crate::blend::EPSILON_0;
use crate::lock::locker::PixelLocker;

#[derive(Default)]
pub struct BlendLocker;

impl PixelLocker<[f32; 4]> for BlendLocker {
    fn should_blit_pixel(&self, pixel: &[f32; 4]) -> bool {
        pixel[3] >= EPSILON_0
    }
}
