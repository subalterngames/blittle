use crate::convert::PixelConverter;
use crate::{L8Surface, Surface};

impl<S: AsRef<[[u8; 2]]> + AsMut<[[u8; 2]]>> PixelConverter<[u8; 2]> for Surface<'_, S, [u8; 2]> {
    fn pixel_to_l8(pixel: &[u8; 2]) -> u8 {
        pixel[0]
    }

    fn pixel_to_la8(pixel: &[u8; 2]) -> [u8; 2] {
        *pixel
    }

    fn pixel_to_l32(pixel: &[u8; 2]) -> f32 {
        L8Surface::u8_to_f32(pixel[0])
    }

    fn pixel_to_la32(pixel: &[u8; 2]) -> [f32; 2] {
        [
            L8Surface::u8_to_f32(pixel[0]),
            L8Surface::u8_to_f32(pixel[1]),
        ]
    }

    fn pixel_to_rgb8(pixel: &[u8; 2]) -> [u8; 3] {
        [pixel[0]; 3]
    }

    fn pixel_to_rgba8(pixel: &[u8; 2]) -> [u8; 4] {
        let p = pixel[0];
        [p, p, p, pixel[1]]
    }

    fn pixel_to_rgba32(pixel: &[u8; 2]) -> [f32; 4] {
        let p = L8Surface::u8_to_f32(pixel[0]);
        let a = L8Surface::u8_to_f32(pixel[1]);
        [p, p, p, a]
    }
}
