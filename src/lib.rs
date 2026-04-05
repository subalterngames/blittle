#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
mod mask;
#[cfg(feature = "png")]
mod png;
mod rect;
mod rgba32;
mod surface;
mod zrgb8;

pub use error::Error;
pub use mask::MaskedSurface;
pub use rect::*;
pub use surface::*;
