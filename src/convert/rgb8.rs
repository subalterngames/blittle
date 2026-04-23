use crate::Surface;
use crate::convert::{PixelConverter, f32_to_u8, grayscale, u8_to_f32};

impl<S: AsRef<[[u8; 3]]> + AsMut<[[u8; 3]]>> PixelConverter<[u8; 3]> for Surface<'_, S, [u8; 3]> {
    fn pixel_to_l8(pixel: &[u8; 3]) -> u8 {
        f32_to_u8(Self::pixel_to_l32(pixel))
    }

    fn pixel_to_la8(pixel: &[u8; 3]) -> [u8; 2] {
        [Self::pixel_to_l8(pixel), 255]
    }

    fn pixel_to_l32(pixel: &[u8; 3]) -> f32 {
        grayscale(pixel[0], pixel[1], pixel[2])
    }

    fn pixel_to_la32(pixel: &[u8; 3]) -> [f32; 2] {
        [Self::pixel_to_l32(pixel), 1.]
    }

    fn pixel_to_rgb8(pixel: &[u8; 3]) -> [u8; 3] {
        *pixel
    }

    fn pixel_to_rgba8(pixel: &[u8; 3]) -> [u8; 4] {
        [pixel[0], pixel[1], pixel[2], 255]
    }

    fn pixel_to_rgba32(pixel: &[u8; 3]) -> [f32; 4] {
        [
            u8_to_f32(pixel[0]),
            u8_to_f32(pixel[1]),
            u8_to_f32(pixel[2]),
            1.,
        ]
    }
}
