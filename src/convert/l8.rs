use crate::Surface;
use crate::convert::{PixelConverter, u8_to_f32};

impl<S: AsRef<[u8]> + AsMut<[u8]>> PixelConverter<u8> for Surface<'_, S, u8> {
    fn pixel_to_l8(pixel: &u8) -> u8 {
        *pixel
    }

    fn pixel_to_la8(pixel: &u8) -> [u8; 2] {
        [*pixel, 255]
    }

    fn pixel_to_l32(pixel: &u8) -> f32 {
        u8_to_f32(*pixel)
    }

    fn pixel_to_la32(pixel: &u8) -> [f32; 2] {
        [Self::pixel_to_l32(pixel), 1.]
    }

    fn pixel_to_rgb8(pixel: &u8) -> [u8; 3] {
        [*pixel; 3]
    }

    fn pixel_to_rgba8(pixel: &u8) -> [u8; 4] {
        [*pixel; 4]
    }

    fn pixel_to_rgba32(pixel: &u8) -> [f32; 4] {
        let p = Self::pixel_to_l32(pixel);
        [p, p, p, 1.]
    }
}
