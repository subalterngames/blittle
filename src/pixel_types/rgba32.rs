use crate::{Rgb8Surface, Rgba8Surface, Rgba32Surface, pixel_types::u8_to_f32};
use glam::Vec4;
use std::ops::Deref;

macro_rules! floats_to_bytes {
    ($self:ident, $get:ident, $set:ident, $pixel:ident, $dest:tt) => {
        /// Creates a new surface, converting pixel values.
        pub fn $get(&$self) -> $dest {
            let buffer = $self
                .buffer
                .iter()
                .map(|pixel| Self::$pixel(pixel))
                .collect();
            $dest {
                size: $self.size,
                buffer,
                destination_rect: $self.destination_rect,
                blit_area: $self.blit_area,
            }
        }

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
    pub const fn pixel_to_rgba32(pixel: &[u8; 3], alpha: f32) -> Vec4 {
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

    /// Creates a new surface, converting pixel values, where the alpha channel value is always 1.
    pub fn get_rgba32(&self) -> Rgba32Surface {
        let buffer = self
            .buffer
            .iter()
            .map(|pixel| Self::pixel_to_rgba32(pixel, 1.))
            .collect();
        Rgba32Surface {
            size: self.size,
            buffer,
            destination_rect: self.destination_rect,
            blit_area: self.blit_area,
        }
    }
}

impl Rgba8Surface {
    /// Convert an RGBA8 pixel to an RGBA32 pixel.
    pub const fn pixel_to_rgba32(pixel: &[u8; 4]) -> Vec4 {
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

    /// Creates a new surface, converting pixel values, where the alpha channel value is always 1.
    pub fn get_rgba32(&self) -> Rgba32Surface {
        let buffer = self.buffer.iter().map(Self::pixel_to_rgba32).collect();
        Rgba32Surface {
            size: self.size,
            buffer,
            destination_rect: self.destination_rect,
            blit_area: self.blit_area,
        }
    }
}

impl Rgba32Surface {
    /// Convert an RGBA32 pixel to an RGBA32 pixel.
    #[inline]
    pub fn pixel_to_rgb8(pixel: &Vec4) -> [u8; 3] {
        let color = pixel * 256.;
        let color = color.deref();
        [color.x as u8, color.y as u8, color.z as u8]
    }

    /// Convert an RGBA32 pixel to an RGBA8 pixel.
    #[inline]
    pub fn pixel_to_rgba8(color: &Vec4) -> [u8; 4] {
        let color = color * 256.;
        let color = color.deref();
        [color.x as u8, color.y as u8, color.z as u8, color.w as u8]
    }

    floats_to_bytes!(self, get_rgb8, set_rgb8, pixel_to_rgb8, Rgb8Surface);

    floats_to_bytes!(self, get_rgba8, set_rgba8, pixel_to_rgba8, Rgba8Surface);
}
