#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod error;
#[cfg(feature = "overlay")]
pub mod overlay;
mod rect;
mod surface;

pub use rect::*;
pub use surface::*;
