#[cfg(feature = "png")]
use png::ColorType;
use thiserror::Error;

use crate::rect::RectI;
use crate::{PositionU, Size};

#[derive(Debug, Error)]
pub enum Error {
    #[error(
        "Tried to create a surface from a buffer of len {} but size.x * size.y != len: {}",
        len,
        size
    )]
    InvalidSize { size: Size, len: usize },
    #[error("The destination hasn't been set.")]
    NoDestinationRect,
    #[error("Source surface doesn't overlap with destination surface.")]
    NoOverlap,
    #[error("Tried to set the area before setting the position relative to the destination.")]
    AreaBeforePosition,
    #[error("Failed to set destination rect. From: {0} To: {1}")]
    InvalidDestinationRect(RectI, RectI),
    #[error("Invalid blit area: {0}")]
    InvalidArea(RectI),
    #[error("Pixel {position} out of bounds of this surface of size: {size}")]
    PixelPosition { position: PositionU, size: Size },
    #[error("Invalid mask size. Expected: {expected} Got: {actual}")]
    MaskSize { actual: usize, expected: usize },
    #[error("Surface is currently locked.")]
    Locked,
    #[cfg(feature = "png")]
    #[error("Failed to write to {0} Reason: {1}")]
    PngFile(std::path::PathBuf, std::io::Error),
    #[cfg(feature = "png")]
    #[error("Failed to write png header: {0}")]
    PngHeader(png::EncodingError),
    #[cfg(feature = "png")]
    #[error("Failed to write png pixel data: {0}")]
    PngPixels(png::EncodingError),
    #[cfg(feature = "png")]
    #[error("png has invalid color type: {:?}. Expected: {:?}", 1, 2)]
    PngColorType(ColorType, ColorType),
}
