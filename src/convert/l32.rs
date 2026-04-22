use crate::Surface;
use crate::convert::PixelConverter;

impl<S: AsRef<[f32]> + AsMut<[f32]>> PixelConverter<f32> for Surface<'_, S, f32> {
    fn pixel_to_l8(pixel: &f32) -> u8 {
        Self::f32_to_u8(*pixel)
    }

    fn pixel_to_la8(pixel: &f32) -> [u8; 2] {
        [Self::pixel_to_l8(pixel), 255]
    }

    fn pixel_to_l32(pixel: &f32) -> f32 {
        *pixel
    }

    fn pixel_to_la32(pixel: &f32) -> [f32; 2] {
        [*pixel, 1.]
    }

    fn pixel_to_rgb8(pixel: &f32) -> [u8; 3] {
        [Self::pixel_to_l8(pixel); 3]
    }

    fn pixel_to_rgba8(pixel: &f32) -> [u8; 4] {
        let p = Self::pixel_to_l8(pixel);
        [p, p, p, 255]
    }

    fn pixel_to_rgba32(pixel: &f32) -> [f32; 4] {
        let p = *pixel;
       [p, p, p, 1.]
    }
}

impl<S: AsRef<[f32]> + AsMut<[f32]>> Surface<'_, S, f32> {
    pub(crate) const fn f32_to_u8(pixel: f32) -> u8 {
        (pixel * 256.) as u8
    }
}
