use crate::convert::PixelConverter;
use crate::{L8Surface, L32Surface, Rgb8Surface, Surface};

impl<S: AsRef<[[u8; 4]]> + AsMut<[[u8; 4]]>> PixelConverter<[u8; 4]> for Surface<'_, S, [u8; 4]> {
    fn pixel_to_l8(pixel: &[u8; 4]) -> u8 {
        L32Surface::f32_to_u8(Self::pixel_to_l32(pixel))
    }

    fn pixel_to_la8(pixel: &[u8; 4]) -> [u8; 2] {
        [Self::pixel_to_l8(pixel), pixel[3]]
    }

    fn pixel_to_l32(pixel: &[u8; 4]) -> f32 {
        Rgb8Surface::grayscale(pixel[0], pixel[1], pixel[2])
    }

    fn pixel_to_la32(pixel: &[u8; 4]) -> [f32; 2] {
        [Self::pixel_to_l32(pixel), L8Surface::u8_to_f32(pixel[3])]
    }

    fn pixel_to_rgb8(pixel: &[u8; 4]) -> [u8; 3] {
        [pixel[0], pixel[1], pixel[2]]
    }

    fn pixel_to_rgba8(pixel: &[u8; 4]) -> [u8; 4] {
        *pixel
    }

    fn pixel_to_rgba32(pixel: &[u8; 4]) -> [f32; 4] {
        [
            L8Surface::u8_to_f32(pixel[0]),
            L8Surface::u8_to_f32(pixel[1]),
            L8Surface::u8_to_f32(pixel[2]),
            L8Surface::u8_to_f32(pixel[3]),
        ]
    }
}
