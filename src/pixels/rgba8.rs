use crate::pixels::PixelConverter;
use crate::{L8Surface, L32Surface, Rgb8Surface, Rgba8Surface};
use glam::Vec4;

impl PixelConverter<[u8; 4]> for Rgba8Surface {
    #[inline]
    fn pixel_to_l8(pixel: &[u8; 4]) -> u8 {
        L32Surface::f32_to_u8(Self::pixel_to_l32(pixel))
    }

    #[inline]
    fn pixel_to_la8(pixel: &[u8; 4]) -> [u8; 2] {
        [Self::pixel_to_l8(pixel), pixel[3]]
    }

    #[inline]
    fn pixel_to_l32(pixel: &[u8; 4]) -> f32 {
        Rgb8Surface::grayscale(pixel[0], pixel[1], pixel[2])
    }

    #[inline]
    fn pixel_to_la32(pixel: &[u8; 4]) -> [f32; 2] {
        [Self::pixel_to_l32(pixel), L8Surface::u8_to_f32(pixel[3])]
    }

    #[inline]
    fn pixel_to_rgb8(pixel: &[u8; 4]) -> [u8; 3] {
        [pixel[0], pixel[1], pixel[2]]
    }

    #[inline]
    fn pixel_to_rgba8(pixel: &[u8; 4]) -> [u8; 4] {
        *pixel
    }

    #[inline]
    fn pixel_to_zrgb8(pixel: &[u8; 4]) -> u32 {
        u32::from_le_bytes([0, pixel[0], pixel[1], pixel[2]])
    }

    #[inline]
    fn pixel_to_rgba32(pixel: &[u8; 4]) -> Vec4 {
        Vec4::new(
            L8Surface::u8_to_f32(pixel[0]),
            L8Surface::u8_to_f32(pixel[1]),
            L8Surface::u8_to_f32(pixel[2]),
            L8Surface::u8_to_f32(pixel[3]),
        )
    }
}
