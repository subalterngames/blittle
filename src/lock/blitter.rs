pub trait PixelBlitter<P> {
    fn should_blit_pixel(&self, pixel: &P) -> bool;

    fn blit_row<B: AsRef<[P]> + AsMut<[P]>>(&self, top: &[P], bottom: &mut [P]);

    fn blit_pixel(&self, top: P, bottom: &mut P);
}
