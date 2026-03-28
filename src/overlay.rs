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
//! use blittle::PixelType;
//!
//! let w = 16;
//! let h = 16;
//! let src = vec![200; w * h * PixelType::Rgb8.stride()];
//! let mut dst = vec![Vec4::default(); w * h];
//! rgb8_to_rgba32_in_place(&src, &mut dst);
//! ```
//!
//! Likewise, it's faster to pre-allocate and then perform the actual overlaying operation:
//!
//! ```
//! use blittle::overlay::*;
//! use blittle::{ClippedRect, PixelType, PositionI, Size};
//!
//!
//! let w = 16;
//! let h = 16;
//!
//! let src8 = vec![200; w * h * PixelType::Rgb8.stride()];
//! // Convert src to RGBA32.
//! let mut src32 = vec![Vec4::default(); w * h];
//! rgb8_to_rgba32_in_place(&src8, &mut src32);
//!
//! let mut dst8 = vec![90; w * h * PixelType::Rgba8.stride()];
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

use crate::{ClippedRect, PixelType};
use bytemuck::{cast_slice, cast_slice_mut};
pub use glam::Vec4;
use std::ops::Deref;

/// Convert a bitmap of RGB8 pixels (1 byte per channel) into a slice of [`Vec4`].
///
/// The size of `dst` must be: `src.len() / 3`.
pub fn rgb8_to_rgba32_in_place(src: &[u8], dst: &mut [Vec4]) {
    cast_slice::<u8, [u8; PixelType::Rgb8.stride()]>(src)
        .iter()
        .zip(dst)
        .for_each(|(src, dst)| {
            dst.x = u8_to_f32(src[0]);
            dst.y = u8_to_f32(src[1]);
            dst.z = u8_to_f32(src[2]);
            dst.w = 1.;
        })
}

/// Convert a bitmap of RGBA8 pixels (1 byte per channel) into a slice of [`Vec4`].
///
/// The size of `dst` must be: `src.len() / 4`.
pub fn rgba8_to_rgba32_in_place(src: &[u8], dst: &mut [Vec4]) {
    cast_slice::<u8, [u8; PixelType::Rgba8.stride()]>(src)
        .iter()
        .zip(dst)
        .for_each(|(src, dst)| {
            dst.x = u8_to_f32(src[0]);
            dst.y = u8_to_f32(src[1]);
            dst.z = u8_to_f32(src[2]);
            dst.w = u8_to_f32(src[3]);
        });
}

/// Convert a bitmap of RGB32 pixels (4 bytes per channel) into a raw RGB8 byte slice.
///
/// The size of `dst` must be: `src.len() * 3`.
pub fn rgba32_to_rgb8_in_place(src: &[Vec4], dst: &mut [u8]) {
    src.iter()
        .zip(cast_slice_mut::<u8, [u8; PixelType::Rgb8.stride()]>(dst))
        .for_each(|(src, dst)| {
            let src = src * 256.;
            let src = src.deref();
            dst[0] = src.x as u8;
            dst[1] = src.y as u8;
            dst[2] = src.z as u8;
        })
}

/// Convert a bitmap of RGBA32 pixels (4 bytes per channel) into a raw RGBA byte slice.
///
/// The size of `dst` must be: `src.len() * 4`.
pub fn rgba32_to_rgba8_in_place(src: &[Vec4], dst: &mut [u8]) {
    src.iter()
        .zip(cast_slice_mut::<u8, [u8; PixelType::Rgba8.stride()]>(dst))
        .for_each(|(src, dst)| {
            let src = src * 256.;
            let src = src.deref();
            dst[0] = src.x as u8;
            dst[1] = src.y as u8;
            dst[2] = src.z as u8;
            dst[3] = src.w as u8;
        })
}

/// Convert a bitmap of RGB8 pixels (1 byte per channel) into a slice of [`Vec4`].
pub fn rgb8_to_rgba32(src: &[u8]) -> Vec<Vec4> {
    let mut dst = vec![Vec4::default(); src.len() / PixelType::Rgb8.stride()];
    rgb8_to_rgba32_in_place(src, &mut dst);
    dst
}

/// Convert a bitmap of RGBA8 pixels (1 byte per channel) into a slice of [`Vec4`].
pub fn rgba8_to_rgba32(src: &[u8]) -> Vec<Vec4> {
    let mut dst = vec![Vec4::default(); src.len() / PixelType::Rgba8.stride()];
    rgba8_to_rgba32_in_place(src, &mut dst);
    dst
}

/// Convert a bitmap of RGB32 pixels (4 bytes per channel) into a raw RGB8 byte slice.
pub fn rgba32_to_rgb8(src: &[Vec4]) -> Vec<u8> {
    let mut dst = vec![0; src.len() * PixelType::Rgb8.stride()];
    rgba32_to_rgb8_in_place(src, &mut dst);
    dst
}

/// Convert a bitmap of RGBA32 pixels (4 bytes per channel) into a raw RGBA byte slice.
pub fn rgba32_to_rgba8(src: &[Vec4]) -> Vec<u8> {
    let mut dst = vec![0; src.len() * PixelType::Rgba8.stride()];
    rgba32_to_rgba8_in_place(src, &mut dst);
    dst
}

/// Convert an RGB8 color to an RGBA32 color.
pub const fn rgb8_to_rgba32_color(color: &[u8; 3]) -> Vec4 {
    Vec4::new(
        u8_to_f32(color[0]),
        u8_to_f32(color[1]),
        u8_to_f32(color[2]),
        1.,
    )
}

/// Convert an RGB8 color to an RGBA32 color.
pub const fn rgba8_to_rgba32_color(color: &[u8; 4]) -> Vec4 {
    Vec4::new(
        u8_to_f32(color[0]),
        u8_to_f32(color[1]),
        u8_to_f32(color[2]),
        u8_to_f32(color[3]),
    )
}

/// Convert an RGBA32 color to an RGB8 color.
#[inline]
pub fn rgba32_to_rgb8_color(color: &Vec4) -> [u8; 3] {
    let color = color * 256.;
    let color = color.deref();
    [
        color.x as u8,
        color.y as u8,
        color.z as u8,
    ]
}

/// Convert an RGBA32 color to an RGBA8 color.
#[inline]
pub fn rgba32_to_rgba8_color(color: &Vec4) -> [u8; 4] {
    let color = color * 256.;
    let color = color.deref();
    [
        color.x as u8,
        color.y as u8,
        color.z as u8,
        color.w as u8,
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
            let src_index = get_index(
                rect.src_position.x,
                rect.src_position.y + src_y,
                rect.src_size.w,
            );
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
        let src_index = get_index(
            rect.src_position.x,
            rect.src_position.y + src_y,
            rect.src_size.w,
        );
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

const fn u8_to_f32(v: u8) -> f32 {
    const U8_TO_F32: [f32; 256] = [
        0.0, 0.0039216, 0.0078431, 0.0117647, 0.0156863, 0.0196078, 0.0235294, 0.027451, 0.0313725,
        0.0352941, 0.0392157, 0.0431373, 0.0470588, 0.0509804, 0.054902, 0.0588235, 0.0627451,
        0.0666667, 0.0705882, 0.0745098, 0.0784314, 0.0823529, 0.0862745, 0.0901961, 0.0941176,
        0.0980392, 0.1019608, 0.1058824, 0.1098039, 0.1137255, 0.1176471, 0.1215686, 0.1254902,
        0.1294118, 0.1333333, 0.1372549, 0.1411765, 0.145098, 0.1490196, 0.1529412, 0.1568627,
        0.1607843, 0.1647059, 0.1686275, 0.172549, 0.1764706, 0.1803922, 0.1843137, 0.1882353,
        0.1921569, 0.1960784, 0.2, 0.2039216, 0.2078431, 0.2117647, 0.2156863, 0.2196078,
        0.2235294, 0.227451, 0.2313725, 0.2352941, 0.2392157, 0.2431373, 0.2470588, 0.2509804,
        0.254902, 0.2588235, 0.2627451, 0.2666667, 0.2705882, 0.2745098, 0.2784314, 0.2823529,
        0.2862745, 0.2901961, 0.2941176, 0.2980392, 0.3019608, 0.3058824, 0.3098039, 0.3137255,
        0.3176471, 0.3215686, 0.3254902, 0.3294118, 0.3333333, 0.3372549, 0.3411765, 0.345098,
        0.3490196, 0.3529412, 0.3568627, 0.3607843, 0.3647059, 0.3686275, 0.372549, 0.3764706,
        0.3803922, 0.3843137, 0.3882353, 0.3921569, 0.3960784, 0.4, 0.4039216, 0.4078431,
        0.4117647, 0.4156863, 0.4196078, 0.4235294, 0.427451, 0.4313725, 0.4352941, 0.4392157,
        0.4431373, 0.4470588, 0.4509804, 0.454902, 0.4588235, 0.4627451, 0.4666667, 0.4705882,
        0.4745098, 0.4784314, 0.4823529, 0.4862745, 0.4901961, 0.4941176, 0.4980392, 0.5019608,
        0.5058824, 0.5098039, 0.5137255, 0.5176471, 0.5215686, 0.5254902, 0.5294118, 0.5333333,
        0.5372549, 0.5411765, 0.545098, 0.5490196, 0.5529412, 0.5568627, 0.5607843, 0.5647059,
        0.5686275, 0.572549, 0.5764706, 0.5803922, 0.5843137, 0.5882353, 0.5921569, 0.5960784, 0.6,
        0.6039216, 0.6078431, 0.6117647, 0.6156863, 0.6196078, 0.6235294, 0.627451, 0.6313725,
        0.6352941, 0.6392157, 0.6431373, 0.6470588, 0.6509804, 0.654902, 0.6588235, 0.6627451,
        0.6666667, 0.6705882, 0.6745098, 0.6784314, 0.6823529, 0.6862745, 0.6901961, 0.6941176,
        0.6980392, 0.7019608, 0.7058824, 0.7098039, 0.7137255, 0.7176471, 0.7215686, 0.7254902,
        0.7294118, 0.7333333, 0.7372549, 0.7411765, 0.745098, 0.7490196, 0.7529412, 0.7568627,
        0.7607843, 0.7647059, 0.7686275, 0.772549, 0.7764706, 0.7803922, 0.7843137, 0.7882353,
        0.7921569, 0.7960784, 0.8, 0.8039216, 0.8078431, 0.8117647, 0.8156863, 0.8196078,
        0.8235294, 0.827451, 0.8313725, 0.8352941, 0.8392157, 0.8431373, 0.8470588, 0.8509804,
        0.854902, 0.8588235, 0.8627451, 0.8666667, 0.8705882, 0.8745098, 0.8784314, 0.8823529,
        0.8862745, 0.8901961, 0.8941176, 0.8980392, 0.9019608, 0.9058824, 0.9098039, 0.9137255,
        0.9176471, 0.9215686, 0.9254902, 0.9294118, 0.9333333, 0.9372549, 0.9411765, 0.945098,
        0.9490196, 0.9529412, 0.9568627, 0.9607843, 0.9647059, 0.9686275, 0.972549, 0.9764706,
        0.9803922, 0.9843137, 0.9882353, 0.9921569, 0.9960784, 1.0,
    ];

    U8_TO_F32[v as usize]
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
        dst_casted
            .iter()
            .zip(rgba32_to_rgba8(&dst_vec4s))
            .for_each(|(a, b)| assert_eq!(*a, b));

        let rect = ClippedRect::new(position, size, size).unwrap();

        // Partial change.
        overlay_rgb8(src_casted, &mut dst_vec4s, &rect, 50);
        cast_slice::<u8, [u8; 4]>(&rgba32_to_rgba8(&dst_vec4s))
            .into_iter()
            .for_each(|pixel| assert_eq!(*pixel, [100, 70, 200, 255]));

        // Total change.
        let src = [[255, 255, 200]; 1024];
        let src_casted = cast_slice::<[u8; 3], u8>(&src);
        overlay_rgb8(src_casted, &mut dst_vec4s, &rect, 255);
        cast_slice::<u8, [u8; 4]>(&rgba32_to_rgba8(&dst_vec4s))
            .into_iter()
            .for_each(|pixel| assert_eq!(*pixel, [255, 255, 200, 255]));
    }
}
