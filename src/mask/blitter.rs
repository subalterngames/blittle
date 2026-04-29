use crate::lock::blitter::PixelBlitter;

pub struct MaskBlitter<P: Copy + Clone + Sized + Default + Eq + PartialEq> {
    mask_color: P,
}

impl<P: Copy + Clone + Sized + Default + Eq + PartialEq> MaskBlitter<P> {
    pub const fn new(mask_color: P) -> Self {
        Self { mask_color }
    }
}

impl<P: Copy + Clone + Sized + Default + Eq + PartialEq> PixelBlitter<P> for MaskBlitter<P> {
    fn should_blit_pixel(&self, pixel: &P) -> bool {
        *pixel != self.mask_color
    }

    fn blit_pixel(&self, top: P, bottom: &mut P) {
        *bottom = top;
    }

    fn blit_row<B: AsRef<[P]> + AsMut<[P]>>(&self, top: &[P], bottom: &mut [P]) {
        bottom.copy_from_slice(top);
    }
}
