use glam::USizeVec2;
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
}
