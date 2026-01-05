/// Channels and bytes per pixel.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum PixelType {
    /// Grayscale. One channel, one byte.
    Gr8,
    /// Grayscale and alpha channels. One byte per channel.
    GrA8,
    /// Red, green, and blue channels. One byte per channel.
    Rgb8,
    /// Red, green, blue, and alpha channels. One byte per channel.
    Rgba8,
    /// Grayscale. One channel, four bytes (an `f32`).
    Gr32,
    /// Red, green, and blue channels. Four bytes per channel. Each channel is an `f32`.
    Rgb32,
    /// Red, green, blue, and alpha channels. Four bytes per channel. Each channel is an `f32`.
    Rgba32,
}

impl PixelType {
    pub const fn stride(&self) -> usize {
        match self {
            Self::Gr8 => 1,
            Self::GrA8 => 2,
            Self::Rgb8 => 3,
            Self::Rgba8 | Self::Gr32 => 4,
            Self::Rgb32 => 12,
            Self::Rgba32 => 16,
        }
    }
}
