use std::ops::Deref;
use glam::Vec4;
use crate::{PixelConverter, Rgb8Surface, Rgba8Surface, Rgba32Surface};

impl Rgb8Surface {
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
                *dst = Self::pixel_and_alpha_to_rgba32(src, alpha);
            });
        other.size = self.size;
        other.destination_rect = self.destination_rect;
        other.blit_area = self.blit_area;
    }
}

impl Rgba8Surface {
    /// Copy data into `other`, converting pixel values.
    pub fn set_rgba32(&self, other: &mut Rgba32Surface) {
        self.buffer
            .iter()
            .zip(other.buffer.iter_mut())
            .for_each(|(src, dst)| {
                if src[3] > 0 {
                    *dst = Self::pixel_to_rgba32(src);
                }
            });
        other.size = self.size;
        other.destination_rect = self.destination_rect;
        other.blit_area = self.blit_area;
    }
}

/// 1. / 256.
const ONE_256: f32 = 0.0039216;

macro_rules! floats_to_bytes {
    ($self:ident, $set:ident, $pixel:ident, $dest:tt) => {
        /// Copy data into `other`, converting pixel values.
        pub fn $set(&$self, other: &mut $dest) {
            $self.buffer
                .iter()
                .zip(other.buffer.iter_mut())
                .for_each(|(src, dst)| {
                    if src.w >= ONE_256 {
                        *dst = Self::$pixel(src);
                    }
                });
            other.size = $self.size;
            other.destination_rect = $self.destination_rect;
            other.blit_area = $self.blit_area;
        }
    };
}

impl Rgba32Surface {
    floats_to_bytes!(self, set_rgb8, pixel_to_rgb8, Rgb8Surface);

    floats_to_bytes!(self, set_rgba8, pixel_to_rgba8, Rgba8Surface);

    fn set_rgba8_pixel(src: &Vec4, dst: &mut [u8; 4]) {
        let pixel = src * 256.;
        let pixel = pixel.deref();
        if pixel.w > 1. {
            *dst = [pixel.x as u8, pixel.y as u8, pixel.z as u8, pixel.w as u8]
        }
    }
}
