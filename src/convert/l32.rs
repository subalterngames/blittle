use crate::L32Surface;
use crate::convert::PixelConverter;
use glam::Vec4;

impl PixelConverter<f32> for L32Surface<'_> {
    #[inline]
    fn pixel_to_l8(pixel: &f32) -> u8 {
        Self::f32_to_u8(*pixel)
    }

    #[inline]
    fn pixel_to_la8(pixel: &f32) -> [u8; 2] {
        [Self::pixel_to_l8(pixel), 255]
    }

    #[inline]
    fn pixel_to_l32(pixel: &f32) -> f32 {
        *pixel
    }

    #[inline]
    fn pixel_to_la32(pixel: &f32) -> [f32; 2] {
        [*pixel, 1.]
    }

    #[inline]
    fn pixel_to_rgb8(pixel: &f32) -> [u8; 3] {
        [Self::pixel_to_l8(pixel); 3]
    }

    #[inline]
    fn pixel_to_rgba8(pixel: &f32) -> [u8; 4] {
        let p = Self::pixel_to_l8(pixel);
        [p, p, p, 255]
    }

    #[inline]
    fn pixel_to_rgba32(pixel: &f32) -> Vec4 {
        let p = *pixel;
        Vec4::new(p, p, p, 1.)
    }
}

impl L32Surface<'_> {
    pub(crate) const fn f32_to_u8(pixel: f32) -> u8 {
        (pixel * 256.) as u8
    }
}
