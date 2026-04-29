pub trait PixelLocker<P> {
    fn should_blit_pixel(&self, pixel: &P) -> bool;
}