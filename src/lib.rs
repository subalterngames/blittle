#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
mod mask;
mod pixels;
#[cfg(feature = "png")]
mod png;
mod rect;
mod surface;
#[cfg(feature = "softbuffer")]
mod sb;

pub use error::Error;
pub use mask::MaskedSurface;
pub use pixels::PixelConverter;
pub use rect::*;
pub use surface::*;
#[cfg(feature = "softbuffer")]
pub use softbuffer;
