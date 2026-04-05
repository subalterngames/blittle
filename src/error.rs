use glam::USizeVec2;
use std::path::PathBuf;
use thiserror::Error;

use crate::rect::RectI;

#[derive(Debug, Error)]
pub enum Error {
    #[error("The destination hasn't been set.")]
    NoDestinationRect,
    #[error("Failed to set destination rect. From: {0} To: {1}")]
    InvalidDestinationRect(RectI, RectI),
    #[error("Invalid blit area: {0}")]
    InvalidArea(RectI),
    #[error("Pixel ({x}, {y}) out of bounds of this surface of size: {size}")]
    PixelPosition { x: usize, y: usize, size: USizeVec2 },
    #[cfg(feature = "png")]
    #[error("Failed to write to {0} Reason: {1}")]
    PngFile(PathBuf, std::io::Error),
    #[cfg(feature = "png")]
    #[error("Failed to write png header: {0}")]
    PngHeader(png::EncodingError),
    #[error("Failed to write png pixel data: {0}")]
    PngPixels(png::EncodingError),
}
