use crate::convert::PixelConverter;
use crate::{L8Surface, La8Surface};
use glam::Vec4;

impl PixelConverter<[u8; 2]> for La8Surface<'_> {
    #[inline]
    fn pixel_to_l8(pixel: &[u8; 2]) -> u8 {
        pixel[0]
    }

    #[inline]
    fn pixel_to_la8(pixel: &[u8; 2]) -> [u8; 2] {
        *pixel
    }

    #[inline]
    fn pixel_to_l32(pixel: &[u8; 2]) -> f32 {
        L8Surface::u8_to_f32(pixel[0])
    }

    #[inline]
    fn pixel_to_la32(pixel: &[u8; 2]) -> [f32; 2] {
        [
            L8Surface::u8_to_f32(pixel[0]),
            L8Surface::u8_to_f32(pixel[1]),
        ]
    }

    #[inline]
    fn pixel_to_rgb8(pixel: &[u8; 2]) -> [u8; 3] {
        [pixel[0]; 3]
    }

    #[inline]
    fn pixel_to_rgba8(pixel: &[u8; 2]) -> [u8; 4] {
        let p = pixel[0];
        [p, p, p, pixel[1]]
    }

    #[inline]
    fn pixel_to_rgba32(pixel: &[u8; 2]) -> Vec4 {
        let p = L8Surface::u8_to_f32(pixel[0]);
        Vec4::new(p, p, p, L8Surface::u8_to_f32(pixel[1]))
    }
}
