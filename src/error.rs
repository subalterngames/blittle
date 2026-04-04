use thiserror::Error;

use crate::rect::RectI;

#[derive(Debug, Error)]
pub enum Error {
    #[error("The destination hasn't been set.")]
    NoDestinationRect,
    #[error("Invalid blit area: {0}")]
    InvalidArea(RectI)
}
