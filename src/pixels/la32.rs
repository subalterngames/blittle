use crate::pixels::PixelConverter;
use crate::{L32Surface, La32Surface};
use glam::Vec4;

impl PixelConverter<[f32; 2]> for La32Surface {
    #[inline]
    fn pixel_to_l8(pixel: &[f32; 2]) -> u8 {
        L32Surface::f32_to_u8(pixel[0])
    }

    #[inline]
    fn pixel_to_la8(pixel: &[f32; 2]) -> [u8; 2] {
        [
            L32Surface::f32_to_u8(pixel[0]),
            L32Surface::f32_to_u8(pixel[1]),
        ]
    }

    #[inline]
    fn pixel_to_l32(pixel: &[f32; 2]) -> f32 {
        pixel[0]
    }

    #[inline]
    fn pixel_to_la32(pixel: &[f32; 2]) -> [f32; 2] {
        *pixel
    }

    #[inline]
    fn pixel_to_rgb8(pixel: &[f32; 2]) -> [u8; 3] {
        [L32Surface::f32_to_u8(pixel[0]); 3]
    }

    #[inline]
    fn pixel_to_rgba8(pixel: &[f32; 2]) -> [u8; 4] {
        let p = L32Surface::f32_to_u8(pixel[0]);
        [p, p, p, L32Surface::f32_to_u8(pixel[1])]
    }

    #[inline]
    fn pixel_to_zrgb8(pixel: &[f32; 2]) -> u32 {
        L32Surface::pixel_to_zrgb8(&pixel[0])
    }

    #[inline]
    fn pixel_to_rgba32(pixel: &[f32; 2]) -> Vec4 {
        let p = pixel[0];
        Vec4::new(p, p, p, pixel[1])
    }
}
