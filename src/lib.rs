#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
mod mask;
mod pixel_types;
#[cfg(feature = "png")]
mod png;
mod rect;
mod set_pixels;
mod surface;
mod surface_ref;

pub use error::Error;
pub use mask::MaskedSurface;
pub use rect::*;
pub use surface::*;
