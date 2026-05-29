use crate::PixelConverter;
use crate::convert::{f32_to_u8, grayscale, u8_to_f32};
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

impl PixelConverter for Zrgb {
    fn to_l8(&self) -> u8 {
        f32_to_u8(self.to_l32())
    }

    fn to_la8(&self) -> [u8; 2] {
        [self.to_l8(), 255]
    }

    fn to_l32(&self) -> f32 {
        grayscale(self.r, self.g, self.b)
    }

    fn to_la32(&self) -> [f32; 2] {
        [self.to_l32(), 1.]
    }

    fn to_rgb8(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }

    fn to_rgba8(&self) -> [u8; 4] {
        [self.r, self.g, self.b, 255]
    }

    fn to_rgba32(&self) -> [f32; 4] {
        [u8_to_f32(self.r), u8_to_f32(self.g), u8_to_f32(self.b), 1.]
    }

    fn to_zrgb(&self) -> Zrgb {
        *self
    }

    fn from_l8(pixel: &u8) -> Self {
        pixel.to_zrgb()
    }

    fn from_l32(pixel: &f32) -> Self {
        pixel.to_zrgb()
    }

    fn from_la8(pixel: &[u8; 2]) -> Self {
        pixel.to_zrgb()
    }

    fn from_la32(pixel: &[f32; 2]) -> Self {
        pixel.to_zrgb()
    }

    fn from_rgb8(pixel: &[u8; 3]) -> Self {
        pixel.to_zrgb()
    }

    fn from_rgba8(pixel: &[u8; 4]) -> Self {
        pixel.to_zrgb()
    }

    fn from_rgba32(pixel: &[f32; 4]) -> Self {
        pixel.to_zrgb()
    }

    fn from_zrgb(pixel: &Zrgb) -> Self {
        *pixel
    }
}
