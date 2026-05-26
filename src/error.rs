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
    #[error("Masked surface is currently locked.")]
    Locked,
}
