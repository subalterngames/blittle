use crate::lock::locker::PixelLocker;

pub struct MaskLocker<P: Copy + Clone + Sized + Default + Eq + PartialEq> {
    mask_color: P,
}

impl<P: Copy + Clone + Sized + Default + Eq + PartialEq> MaskLocker<P> {
    pub const fn new(mask_color: P) -> Self {
        Self { mask_color }
    }
}

impl<P: Copy + Clone + Sized + Default + Eq + PartialEq> PixelLocker<P> for MaskLocker<P> {
    fn should_blit_pixel(&self, pixel: &P) -> bool {
        *pixel != self.mask_color
    }
}