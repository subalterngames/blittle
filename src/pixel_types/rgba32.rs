use crate::{Rgb8Surface, Rgba8Surface, Rgba32Surface, pixel_types::u8_to_f32};
use glam::Vec4;
use std::ops::Deref;

macro_rules! floats_to_bytes {
    ($self:ident, $set:ident, $pixel:ident, $dest:tt) => {
        /// Copy data into `other`, converting pixel values.
        pub fn $set(&$self, other: &mut $dest) {
            $self.buffer
                .iter()
                .zip(other.buffer.iter_mut())
                .for_each(|(src, dst)| {
                    *dst = Self::$pixel(src);
                });
            other.size = $self.size;
            other.destination_rect = $self.destination_rect;
            other.blit_area = $self.blit_area;
        }
    };
}

impl Rgb8Surface {
    /// Convert an RGB8 pixel to an RGBA32 pixel.
    const fn pixel_to_rgba32(pixel: &[u8; 3], alpha: f32) -> Vec4 {
        Vec4::new(
            u8_to_f32(pixel[0]),
            u8_to_f32(pixel[1]),
            u8_to_f32(pixel[2]),
            alpha,
        )
    }

    /// Copy data into `other`, converting pixel values.
    ///
    /// `alpha` is the alpha value for the entire surface (0-1)
    pub fn set_rgba32(&self, other: &mut Rgba32Surface, alpha: f32) {
        if alpha <= 0. {
            return;
        }
        self.buffer
            .iter()
            .zip(other.buffer.iter_mut())
            .for_each(|(src, dst)| {
                *dst = Self::pixel_to_rgba32(src, alpha);
            });
        other.size = self.size;
        other.destination_rect = self.destination_rect;
        other.blit_area = self.blit_area;
    }
}

impl Rgba8Surface {
    /// Convert an RGBA8 pixel to an RGBA32 pixel.
    pub(super) const fn pixel_to_rgba32(pixel: &[u8; 4]) -> Vec4 {
        Vec4::new(
            u8_to_f32(pixel[0]),
            u8_to_f32(pixel[1]),
            u8_to_f32(pixel[2]),
            u8_to_f32(pixel[3]),
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

impl Rgba32Surface {
    /// Convert an RGBA32 pixel to an RGBA32 pixel.
    #[inline]
    pub(super) fn pixel_to_rgb8(pixel: &Vec4) -> [u8; 3] {
        let pixel = pixel * 256.;
        let pixel = pixel.deref();
        [pixel.x as u8, pixel.y as u8, pixel.z as u8]
    }

    /// Convert an RGBA32 pixel to an RGBA8 pixel.
    #[inline]
    pub(super) fn pixel_to_rgba8(pixel: &Vec4) -> [u8; 4] {
        let pixel = pixel * 256.;
        let pixel = pixel.deref();
        [pixel.x as u8, pixel.y as u8, pixel.z as u8, pixel.w as u8]
    }

    floats_to_bytes!(self, set_rgb8, pixel_to_rgb8, Rgb8Surface);

    floats_to_bytes!(self, set_rgba8, pixel_to_rgba8, Rgba8Surface);
}
