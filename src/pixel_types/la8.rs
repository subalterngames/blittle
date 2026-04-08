use crate::{L8Surface, La8Surface};
use glam::Vec4;

impl La8Surface {
    pub const fn pixel_to_l8(pixel: &[u8; 2]) -> u8 {
        pixel[0]
    }

    pub const fn pixel_to_l32(pixel: &[u8; 2]) -> f32 {
        L8Surface::u8_to_f32(pixel[0])
    }

    pub const fn pixel_to_la32(pixel: &[u8; 2]) -> [f32; 2] {
        [
            L8Surface::u8_to_f32(pixel[0]),
            L8Surface::u8_to_f32(pixel[1]),
        ]
    }

    pub const fn pixel_to_rgb8(pixel: &[u8; 2]) -> [u8; 3] {
        [pixel[0]; 3]
    }

    pub const fn pixel_to_rgba8(pixel: &[u8; 2]) -> [u8; 4] {
        let p = pixel[0];
        [p, p, p, pixel[1]]
    }

    pub const fn pixel_to_zrgb8(pixel: &[u8; 2]) -> u32 {
        let pixel = pixel[0] as u32;
        ((pixel << 24) | pixel << 16) | pixel << 8
    }

    pub const fn pixel_to_rgba32(pixel: &[u8; 2]) -> Vec4 {
        let p = L8Surface::u8_to_f32(pixel[0]);
        Vec4::new(p, p, p, L8Surface::u8_to_f32(pixel[1]))
    }
}
