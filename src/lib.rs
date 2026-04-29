#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod blend;
mod convert;
mod error;
mod mask;
#[cfg(feature = "png")]
pub mod png;
mod position;
mod rect;
#[cfg(feature = "softbuffer")]
pub mod sb;
mod size;
mod surface;

pub use blend::*;
pub use convert::PixelConverter;
pub use error::Error;
pub use mask::MaskedSurface;
pub use position::*;
pub use rect::*;
pub use size::Size;
#[cfg(feature = "softbuffer")]
pub use softbuffer;
pub use surface::*;
