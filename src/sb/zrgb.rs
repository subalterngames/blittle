use bytemuck::{Pod, Zeroable};
use std::ops::{Deref, DerefMut};

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

impl Deref for Zrgb {
    type Target = [u8; 4];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self).cast() }
    }
}

impl DerefMut for Zrgb {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self).cast() }
    }
}
