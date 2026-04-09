use crate::pixels::PixelConverter;
use crate::{L32Surface, Rgba32Surface};
use glam::{Vec4, Vec4Swizzles};
use std::ops::Deref;

impl PixelConverter<Vec4> for Rgba32Surface {
    #[inline]
    fn pixel_to_l8(pixel: &Vec4) -> u8 {
        L32Surface::f32_to_u8(Self::pixel_to_l32(pixel))
    }

    #[inline]
    fn pixel_to_la8(pixel: &Vec4) -> [u8; 2] {
        [
            L32Surface::f32_to_u8(Self::pixel_to_l32(pixel)),
            L32Surface::f32_to_u8(pixel.w),
        ]
    }

    #[inline]
    fn pixel_to_l32(pixel: &Vec4) -> f32 {
        (pixel * 256.).xyz().element_sum() / 3.
    }

    #[inline]
    fn pixel_to_la32(pixel: &Vec4) -> [f32; 2] {
        [Self::pixel_to_l32(pixel), pixel.w]
    }

    #[inline]
    fn pixel_to_rgb8(pixel: &Vec4) -> [u8; 3] {
        let pixel = pixel * 256.;
        let pixel = pixel.deref();
        [pixel.x as u8, pixel.y as u8, pixel.z as u8]
    }

    #[inline]
    fn pixel_to_rgba8(pixel: &Vec4) -> [u8; 4] {
        let pixel = pixel * 256.;
        let pixel = pixel.deref();
        [pixel.x as u8, pixel.y as u8, pixel.z as u8, pixel.w as u8]
    }

    #[inline]
    fn pixel_to_zrgb8(pixel: &Vec4) -> u32 {
        let p = pixel * 256.;
        let p = p.deref();
        u32::from_le_bytes([0, p.x as u8, p.y as u8, p.z as u8])
    }

    #[inline]
    fn pixel_to_rgba32(pixel: &Vec4) -> Vec4 {
        *pixel
    }
}
