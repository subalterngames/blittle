use crate::{L8Surface, L32Surface, Rgb8Surface, Rgba8Surface, Rgba32Surface};
use glam::Vec4;

impl Rgba8Surface {
    pub const fn pixel_to_l8(pixel: &[u8; 4]) -> u8 {
        L32Surface::f32_to_u8(Self::pixel_to_l32(pixel))
    }

    pub const fn pixel_to_la8(pixel: &[u8; 4]) -> [u8; 2] {
        [Self::pixel_to_l8(pixel), pixel[3]]
    }

    pub const fn pixel_to_l32(pixel: &[u8; 4]) -> f32 {
        Rgb8Surface::grayscale(pixel[0], pixel[1], pixel[2])
    }

    pub const fn pixel_to_la32(pixel: &[u8; 4]) -> [f32; 2] {
        [Self::pixel_to_l32(pixel), L8Surface::u8_to_f32(pixel[3])]
    }

    pub const fn pixel_to_rgb8(pixel: &[u8; 4]) -> [u8; 3] {
        [pixel[0], pixel[1], pixel[2]]
    }

    pub const fn pixel_to_zrgb8(pixel: &[u8; 4]) -> u32 {
        u32::from_le_bytes([0, pixel[0], pixel[1], pixel[2]])
    }

    pub const fn pixel_to_rgba32(pixel: &[u8; 4]) -> Vec4 {
        Vec4::new(
            L8Surface::u8_to_f32(pixel[0]),
            L8Surface::u8_to_f32(pixel[1]),
            L8Surface::u8_to_f32(pixel[2]),
            L8Surface::u8_to_f32(pixel[3]),
        )
    }

    /// Copy data into `other`, converting pixel values.
    pub fn set_rgba32(&self, other: &mut Rgba32Surface) {
        self.buffer
            .iter()
            .zip(other.buffer.iter_mut())
            .for_each(|(src, dst)| {
                *dst = Self::pixel_to_rgba32(src);
            });
        other.size = self.size;
        other.destination_rect = self.destination_rect;
        other.blit_area = self.blit_area;
    }
}
