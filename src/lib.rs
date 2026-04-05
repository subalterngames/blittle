#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod error;
#[cfg(feature = "png")]
mod png;
mod rect;
mod rgba32;
mod surface;
mod zrgb8;

pub use rect::*;
pub use surface::*;
