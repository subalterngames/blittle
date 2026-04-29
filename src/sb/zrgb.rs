use crate::convert::{f32_to_u8, grayscale, u8_to_f32};
use crate::{PixelConverter, Surface};
use bytemuck::{Pod, Zeroable};

/// A pixel color in a softbuffer Buffer.
///
/// The `Z` in this case refers to the one-byte padding at the start of the struct.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialOrd, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct Zrgb {
    _z: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Zrgb {
    pub const BLACK: Self = Self::new(0, 0, 0);
    pub const WHITE: Self = Self::new(255, 255, 255);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { _z: 0, r, g, b }
    }
}

impl Default for Zrgb {
    fn default() -> Self {
        Self::BLACK
    }
}

impl<S: AsRef<[Zrgb]> + AsMut<[Zrgb]>> PixelConverter<Zrgb> for Surface<'_, S, Zrgb> {
    fn pixel_to_l8(pixel: &Zrgb) -> u8 {
        f32_to_u8(Self::pixel_to_l32(pixel))
    }

    fn pixel_to_la8(pixel: &Zrgb) -> [u8; 2] {
        [Self::pixel_to_l8(pixel), 255]
    }

    fn pixel_to_l32(pixel: &Zrgb) -> f32 {
        grayscale(pixel.r, pixel.g, pixel.b)
    }

    fn pixel_to_la32(pixel: &Zrgb) -> [f32; 2] {
        [Self::pixel_to_l32(pixel), 1.]
    }

    fn pixel_to_rgb8(pixel: &Zrgb) -> [u8; 3] {
        [pixel.r, pixel.g, pixel.b]
    }

    fn pixel_to_rgba8(pixel: &Zrgb) -> [u8; 4] {
        [pixel.r, pixel.g, pixel.b, 255]
    }

    fn pixel_to_rgba32(pixel: &Zrgb) -> [f32; 4] {
        [
            u8_to_f32(pixel.r),
            u8_to_f32(pixel.g),
            u8_to_f32(pixel.b),
            1.,
        ]
    }

    fn pixel_to_zrgb(pixel: &Zrgb) -> Zrgb {
        *pixel
    }
}
