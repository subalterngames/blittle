use crate::L32Surface;
use glam::Vec4;

impl L32Surface {
    pub const fn pixel_to_l8(pixel: &f32) -> u8 {
        Self::f32_to_u8(*pixel)
    }

    pub const fn pixel_to_la8(pixel: &f32) -> [u8; 2] {
        [Self::pixel_to_l8(pixel), 255]
    }

    pub const fn pixel_to_la32(pixel: &f32) -> [f32; 2] {
        [*pixel, 1.]
    }

    pub const fn pixel_to_rgb8(pixel: &f32) -> [u8; 3] {
        [Self::pixel_to_l8(pixel); 3]
    }

    pub const fn pixel_to_rgba8(pixel: &f32) -> [u8; 4] {
        let p = Self::pixel_to_l8(pixel);
        [p, p, p, 255]
    }

    pub const fn pixel_to_zrgb8(pixel: &f32) -> u32 {
        let pixel = Self::pixel_to_l8(pixel) as u32;
        ((pixel << 24) | pixel << 16) | pixel << 8
    }

    pub const fn pixel_to_rgba32(pixel: &f32) -> Vec4 {
        let p = *pixel;
        Vec4::new(p, p, p, 1.)
    }

    pub(super) const fn f32_to_u8(pixel: f32) -> u8 {
        (pixel * 256.) as u8
    }
}
