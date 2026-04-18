#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod convert;
mod error;
mod mask;
#[cfg(feature = "png")]
pub mod png;
mod rect;
#[cfg(feature = "softbuffer")]
pub mod sb;
mod surface;

pub use convert::PixelConverter;
pub use error::Error;
pub use mask::MaskedSurface;
pub use rect::*;
#[cfg(feature = "softbuffer")]
pub use softbuffer;
pub use surface::*;
