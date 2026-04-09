use crate::pixels::PixelConverter;
use crate::{L8Surface, L32Surface, Rgb8Surface};
use glam::Vec4;

impl PixelConverter<[u8; 3]> for Rgb8Surface {
    #[inline]
    fn pixel_to_l8(pixel: &[u8; 3]) -> u8 {
        L32Surface::f32_to_u8(Self::pixel_to_l32(pixel))
    }

    #[inline]
    fn pixel_to_la8(pixel: &[u8; 3]) -> [u8; 2] {
        [Self::pixel_to_l8(pixel), 255]
    }

    #[inline]
    fn pixel_to_l32(pixel: &[u8; 3]) -> f32 {
        Self::grayscale(pixel[0], pixel[1], pixel[2])
    }

    #[inline]
    fn pixel_to_la32(pixel: &[u8; 3]) -> [f32; 2] {
        [Self::pixel_to_l32(pixel), 1.]
    }

    #[inline]
    fn pixel_to_rgb8(pixel: &[u8; 3]) -> [u8; 3] {
        *pixel
    }

    #[inline]
    fn pixel_to_rgba8(pixel: &[u8; 3]) -> [u8; 4] {
        [pixel[0], pixel[1], pixel[2], 255]
    }

    #[inline]
    fn pixel_to_zrgb8(pixel: &[u8; 3]) -> u32 {
        u32::from_le_bytes([0, pixel[0], pixel[1], pixel[2]])
    }

    #[inline]
    fn pixel_to_rgba32(pixel: &[u8; 3]) -> Vec4 {
        Vec4::new(
            L8Surface::u8_to_f32(pixel[0]),
            L8Surface::u8_to_f32(pixel[1]),
            L8Surface::u8_to_f32(pixel[2]),
            1.,
        )
    }
}

impl Rgb8Surface {
    pub(super) const fn grayscale(r: u8, g: u8, b: u8) -> f32 {
        (L8Surface::u8_to_f32(r) + L8Surface::u8_to_f32(g) + L8Surface::u8_to_f32(b)) / 3.
    }

    pub const fn pixel_and_alpha_to_rgba32(pixel: &[u8; 3], alpha: f32) -> Vec4 {
        Vec4::new(
            L8Surface::u8_to_f32(pixel[0]),
            L8Surface::u8_to_f32(pixel[1]),
            L8Surface::u8_to_f32(pixel[2]),
            alpha,
        )
    }
}
