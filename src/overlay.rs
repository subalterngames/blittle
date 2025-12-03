//! This module contains functions for overlaying a source image onto destination image using alpha (transparency) value(s).
//!
//! To overlay one pixel onto another, the pixels must be converted from three to four 1-byte channels to four 4-byte channels.
//! For example, RGB8 must be converted into RGBA32 (where the value of each channel ranges from 0.0 to 1.0).
//! In this module, `blittle` uses [`Vec4`] to represent an RGBA32 pixel.
//! This allows `blittle` to use glam's SIMD magic.
//!
//! As always, it is more efficient to pre-allocate your `Vec<Vec4>` outputs if you intend to overlay more than once.
//! For example: [`rgb8_to_rgba32`] returns a `Vec<Vec4>` representation of the `src` image,
//! but it would be faster to pre-allocate instead and then call [`rgb8_to_rgba32_in_place`]:
//!
//! ```
//! use blittle::overlay::{rgb8_to_rgba32_in_place, Vec4};
//! use blittle::stride::RGB;
//!
//! let w = 16;
//! let h = 16;
//! let src = vec![200; w * h * RGB];
//! let mut dst = vec![Vec4::default(); w * h];
//! rgb8_to_rgba32_in_place(&src, &mut dst);
//! ```
//!
//! Likewise, it's faster to pre-allocate and then perform the actual overlaying operation:
//!
//! ```
//! use blittle::overlay::*;
//! use blittle::{ClippedRect, PositionI, Size};
//! use blittle::stride::{RGB, RGBA};
//!
//!
//! let w = 16;
//! let h = 16;
//!
//! let src8 = vec![200; w * h * RGB];
//! // Convert src to RGBA32.
//! let mut src32 = vec![Vec4::default(); w * h];
//! rgb8_to_rgba32_in_place(&src8, &mut src32);
//!
//! let mut dst8 = vec![90; w * h * RGBA];
//! /// Convert dst to RGBA32.
//! let mut dst32 = vec![Vec4::default(); w * h];
//! rgba8_to_rgba32_in_place(&dst8, &mut dst32);
//!
//! // Overlay src onto dst.
//! let size = Size { w, h };
//! let position = PositionI::default();
//! let rect = ClippedRect::new(PositionI::default(), size, size).unwrap();
//! overlay_rgba32(&src32, &mut dst32, &rect);
//!
//! // Convert dst32 back into RGBA8.
//! rgba32_to_rgba8_in_place(&dst32, &mut dst8);
//! ```

use crate::clip::ClippedRect;
use crate::stride::{RGB, RGBA};
use bytemuck::{cast_slice, cast_slice_mut};
pub use glam::Vec4;
use std::ops::Deref;

/// Convert a bitmap of RGB8 pixels (1 byte per channel) into a slice of [`Vec4`].
///
/// The size of `dst` must be: `src.len() / 3`.
pub fn rgb8_to_rgba32_in_place(src: &[u8], dst: &mut [Vec4]) {
    cast_slice::<u8, [u8; RGB]>(src)
        .iter()
        .zip(dst)
        .for_each(|(src, dst)| {
            dst.x = src[0] as f32 / 255.;
            dst.y = src[1] as f32 / 255.;
            dst.z = src[2] as f32 / 255.;
            dst.w = 1.;
        })
}

/// Convert a bitmap of RGBA8 pixels (1 byte per channel) into a slice of [`Vec4`].
///
/// The size of `dst` must be: `src.len() / 4`.
pub fn rgba8_to_rgba32_in_place(src: &[u8], dst: &mut [Vec4]) {
    cast_slice::<u8, [u8; RGBA]>(src)
        .iter()
        .zip(dst)
        .for_each(|(src, dst)| {
            dst.x = src[0] as f32 / 255.;
            dst.y = src[1] as f32 / 255.;
            dst.z = src[2] as f32 / 255.;
            dst.w = src[3] as f32 / 255.;
        });
}

/// Convert a bitmap of RGB32 pixels (4 bytes per channel) into a raw RGB8 byte slice.
///
/// The size of `dst` must be: `src.len() * 3`.
pub fn rgba32_to_rgb8_in_place(src: &[Vec4], dst: &mut [u8]) {
    src.iter()
        .zip(cast_slice_mut::<u8, [u8; RGB]>(dst))
        .for_each(|(src, dst)| {
            dst[0] = (src.x * 255.).ceil() as u8;
            dst[1] = (src.y * 255.).ceil() as u8;
            dst[2] = (src.z * 255.).ceil() as u8;
        })
}

/// Convert a bitmap of RGBA32 pixels (4 bytes per channel) into a raw RGBA byte slice.
///
/// The size of `dst` must be: `src.len() * 4`.
pub fn rgba32_to_rgba8_in_place(src: &[Vec4], dst: &mut [u8]) {
    src.iter()
        .zip(cast_slice_mut::<u8, [u8; RGBA]>(dst))
        .for_each(|(src, dst)| {
            dst[0] = (src.x * 255.).ceil() as u8;
            dst[1] = (src.y * 255.).ceil() as u8;
            dst[2] = (src.z * 255.).ceil() as u8;
            dst[3] = (src.w * 255.).ceil() as u8;
        })
}

/// Convert a bitmap of RGB8 pixels (1 byte per channel) into a slice of [`Vec4`].
pub fn rgb8_to_rgba32(src: &[u8]) -> Vec<Vec4> {
    let mut dst = vec![Vec4::default(); src.len() / RGB];
    rgb8_to_rgba32_in_place(src, &mut dst);
    dst
}

/// Convert a bitmap of RGBA8 pixels (1 byte per channel) into a slice of [`Vec4`].
pub fn rgba8_to_rgba32(src: &[u8]) -> Vec<Vec4> {
    let mut dst = vec![Vec4::default(); src.len() / RGBA];
    rgba8_to_rgba32_in_place(src, &mut dst);
    dst
}

/// Convert a bitmap of RGB32 pixels (4 bytes per channel) into a raw RGB8 byte slice.
pub fn rgba32_to_rgb8(src: &[Vec4]) -> Vec<u8> {
    let mut dst = vec![0; src.len() * RGB];
    rgba32_to_rgb8_in_place(src, &mut dst);
    dst
}

/// Convert a bitmap of RGBA32 pixels (4 bytes per channel) into a raw RGBA byte slice.
pub fn rgba32_to_rgba8(src: &[Vec4]) -> Vec<u8> {
    let mut dst = vec![0; src.len() * RGBA];
    rgba32_to_rgba8_in_place(src, &mut dst);
    dst
}

/// Convert an RGB8 color to an RGBA32 color.
pub const fn rgb8_to_rgba32_color(color: &[u8; 3]) -> Vec4 {
    Vec4::new(
        color[0] as f32 / 255.,
        color[1] as f32 / 255.,
        color[2] as f32 / 255.,
        1.,
    )
}

/// Convert an RGB8 color to an RGBA32 color.
pub const fn rgba8_to_rgba32_color(color: &[u8; 4]) -> Vec4 {
    Vec4::new(
        color[0] as f32 / 255.,
        color[1] as f32 / 255.,
        color[2] as f32 / 255.,
        color[3] as f32 / 255.,
    )
}

/// Convert an RGBA32 color to an RGB8 color.
#[inline]
pub fn rgba32_to_rgb8_color(color: &Vec4) -> [u8; 3] {
    let color = color.deref();
    [
        (color.x * 255.).ceil() as u8,
        (color.y * 255.).ceil() as u8,
        (color.z * 255.).ceil() as u8,
    ]
}

/// Convert an RGBA32 color to an RGBA8 color.
#[inline]
pub fn rgba32_to_rgba8_color(color: &Vec4) -> [u8; 4] {
    let color = color.deref();
    [
        (color.x * 255.).ceil() as u8,
        (color.y * 255.).ceil() as u8,
        (color.z * 255.).ceil() as u8,
        (color.w * 255.).ceil() as u8,
    ]
}

/// Overlay `src` onto `dst` using an `alpha` value.
///
/// - `src` is an RGB8 (1 byte per channel) image.
/// - `dst` is an RGBA32 (4 bytes per channel) image.
/// - `rect` is the [`ClippedRect`] defining the blit area.
/// - `alpha` is the alpha channel. 0 is totally transparent.
///
/// Returns `src` as an RGBA32 bitmap (see: [`rgb8_to_rgba32`]).
pub fn overlay_rgb8(src: &[u8], dst: &mut [Vec4], rect: &ClippedRect, alpha: u8) -> Vec<Vec4> {
    if alpha > 0 {
        let src = rgb8_to_rgba32(src);
        (0..rect.src_size_clipped.h).for_each(|src_y| {
            let src_index = get_index(0, src_y, rect.src_size.w);
            let dst_index = get_index(
                rect.dst_position_clipped.x,
                rect.dst_position_clipped.y + src_y,
                rect.dst_size.w,
            );
            src[src_index..src_index + rect.src_size_clipped.w]
                .iter()
                .zip(dst[dst_index..dst_index + rect.src_size_clipped.w].iter_mut())
                .for_each(|(src, dst)| {
                    // Replace the values.
                    if alpha == 255 {
                        dst.x = src.x;
                        dst.y = src.y;
                        dst.z = src.z;
                        dst.w = 1.;
                    } else {
                        overlay_pixel(src, dst);
                    }
                });
        });
        src
    } else {
        Vec::default()
    }
}

/// Overlay `src` onto `dst`.
///
/// - `src` is an RGBA8 (1 byte per channel) image.
/// - `dst` is an RGBA32 (4 bytes per channel) image.
/// - `rect` is the [`ClippedRect`] defining the blit area.
///
/// Returns `src` as an RGBA32 bitmap (see: [`rgba8_to_rgba32`]).
pub fn overlay_rgba8(src: &[u8], dst: &mut [Vec4], rect: &ClippedRect) -> Vec<Vec4> {
    let src = rgba8_to_rgba32(src);
    overlay_rgba32(&src, dst, rect);
    src
}

/// Overlay `src` onto `dst`.
///
/// - `src` is an RGBA32 (4 bytes per channel) image.
/// - `dst` is an RGBA32 (4 bytes per channel) image.
/// - `rect` is the [`ClippedRect`] defining the blit area.
pub fn overlay_rgba32(src: &[Vec4], dst: &mut [Vec4], rect: &ClippedRect) {
    (0..rect.src_size_clipped.h).for_each(|src_y| {
        let src_index = get_index(0, src_y, rect.src_size.w);
        let dst_index = get_index(
            rect.dst_position_clipped.x,
            rect.dst_position_clipped.y + src_y,
            rect.dst_size.w,
        );
        src[src_index..src_index + rect.src_size_clipped.w]
            .iter()
            .zip(dst[dst_index..dst_index + rect.src_size_clipped.w].iter_mut())
            .for_each(|(src, dst)| {
                overlay_pixel(src, dst);
            });
    });
}

/// Overlay a `src` pixel onto a `dst` pixel.
#[inline]
pub fn overlay_pixel(src: &Vec4, dst: &mut Vec4) {
    let src_a = src.w;
    // Source: https://shi-yan.github.io/note_on_alpha_blending/
    *dst = *src * src_a + *dst * (1. - src_a);
}

const fn get_index(x: usize, y: usize, w: usize) -> usize {
    x + y * w
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PositionI, Size};

    #[test]
    fn test_rgb8_to_rgba32() {
        let src = [[100, 70, 200]; 1024];
        let casted = cast_slice::<[u8; 3], u8>(&src);
        let vec4s = rgb8_to_rgba32(casted);
        assert_eq!(vec4s.len(), src.len());
        casted
            .iter()
            .zip(rgba32_to_rgb8(&vec4s))
            .for_each(|(a, b)| assert_eq!(*a, b));
    }

    #[test]
    fn test_rgba8_to_rgba32() {
        let src = [[100, 70, 200, 90]; 1024];
        let casted = cast_slice::<[u8; 4], u8>(&src);
        let vec4s = rgba8_to_rgba32(casted);
        assert_eq!(vec4s.len(), src.len());
        casted
            .iter()
            .zip(rgba32_to_rgba8(&vec4s))
            .for_each(|(a, b)| assert_eq!(*a, b));
    }

    #[test]
    fn test_rgb8_overlay() {
        let src = [[100, 70, 200]; 1024];
        let src_casted = cast_slice::<[u8; 3], u8>(&src);

        let dst = [[100, 70, 200, 90]; 1024];
        let dst_casted = cast_slice::<[u8; 4], u8>(&dst);
        let mut dst_vec4s = rgba8_to_rgba32(dst_casted);

        let size = Size { w: 32, h: 32 };
        let position = PositionI::default();

        // No change.
        //overlay_rgb8(src_casted, &size, &mut dst_vec4s, &position, &size, 0);
        dst_casted
            .iter()
            .zip(rgba32_to_rgba8(&dst_vec4s))
            .for_each(|(a, b)| assert_eq!(*a, b));

        let rect = ClippedRect::new(position, size, size).unwrap();

        // Partial change.
        overlay_rgb8(src_casted, &mut dst_vec4s, &rect, 50);
        cast_slice::<u8, [u8; 4]>(&rgba32_to_rgba8(&dst_vec4s))
            .into_iter()
            .for_each(|pixel| assert_eq!(*pixel, [100, 70, 200, 173]));

        // Total change.
        let src = [[255, 255, 200]; 1024];
        let src_casted = cast_slice::<[u8; 3], u8>(&src);
        overlay_rgb8(src_casted, &mut dst_vec4s, &rect, 255);
        cast_slice::<u8, [u8; 4]>(&rgba32_to_rgba8(&dst_vec4s))
            .into_iter()
            .for_each(|pixel| assert_eq!(*pixel, [255, 255, 200, 255]));
    }
}
