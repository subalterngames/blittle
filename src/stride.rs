//! Stride values for various types of pixels.

/// A 1-byte channel.
pub const GRAYSCALE: usize = 1;
/// Three 1-byte channels: red, green, blue.
pub const RGB: usize = 3;
/// Four 1-byte channels: red, green, blue, alpha.
pub const RGBA: usize = 4;
/// Three 4-byte channels, each of which is a f32: red, green, blue.
pub const RGB_F32: usize = 12;
/// Four 4-byte channels, each of which is a f32: red, green, blue, alpha.
pub const RGBA_F32: usize = 16;