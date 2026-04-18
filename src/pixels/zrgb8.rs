use crate::pixels::PixelConverter;
use crate::{L8Surface, L32Surface, Rgb8Surface, Zrgb8Surface};
use glam::Vec4;


impl PixelConverter<u32> for Zrgb8Surface {
    #[inline]
    fn pixel_to_l8(pixel: &u32) -> u8 {
        L32Surface::f32_to_u8(Self::pixel_to_l32(pixel))
    }

    #[inline]
    fn pixel_to_la8(pixel: &u32) -> [u8; 2] {
        [L32Surface::f32_to_u8(Self::pixel_to_l32(pixel)), 255]
    }

    #[inline]
    fn pixel_to_l32(pixel: &u32) -> f32 {
        let p = pixel.to_le_bytes();
        Rgb8Surface::grayscale(p[1], p[2], p[3])
    }

    #[inline]
    fn pixel_to_la32(pixel: &u32) -> [f32; 2] {
        [Self::pixel_to_l32(pixel), 1.]
    }

    #[inline]
    fn pixel_to_rgb8(pixel: &u32) -> [u8; 3] {
        let p = pixel.to_le_bytes();
        [p[1], p[2], p[3]]
    }

    #[inline]
    fn pixel_to_rgba8(pixel: &u32) -> [u8; 4] {
        let p = pixel.to_le_bytes();
        [p[1], p[2], p[3], 255]
    }

    fn pixel_to_zrgb8(pixel: &u32) -> u32 {
        *pixel
    }

    #[inline]
    fn pixel_to_rgba32(pixel: &u32) -> Vec4 {
        let p = pixel.to_le_bytes();
        Vec4::new(
            L8Surface::u8_to_f32(p[1]),
            L8Surface::u8_to_f32(p[2]),
            L8Surface::u8_to_f32(p[3]),
            1.,
        )
    }
}