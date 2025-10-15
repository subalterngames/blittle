//! **`blittle` is a fast little blitter.**
//!
//! Most blit functions assume that you might want to apply a mask.
//! A mask is typically a certain color.
//! Pixels in the source image that have the mask color aren't blitted to the destination image.
//!
//! **`blittle` is fast because it doesn't apply a mask.**
//! Since `blittle` doesn't have to check each pixel's color,
//! it can copy every row of the source image onto the destination image, rather than each pixel.
//!
//! Note that in all cases, `src` and `dst` are slices of raw bitmaps, *not* png/jpg/etc. data.
//!
//! ```
//! use blittle::{blit_to_buffer, stride::RGB};
//!
//! // The dimensions and byte data of the source image.
//! let src_w = 32;
//! let src_h = 17;
//! let src = vec![0u8; src_w * src_h * RGB];
//!
//! // The dimensions and byte data of the destination image.
//! let dst_w = 64;
//! let dst_h = 64;
//! let mut dst = vec![0u8; dst_w * dst_h * RGB];
//!
//! // Blit `src` onto `dst`.
//! blit_to_buffer(&src, &mut dst, 2, 12, dst_w, src_w, RGB);
//! ```
//!
//! `blittle` has one additional means of making blitting faster:
//!
//! In `blit_to_buffer`, `dst` is divided into per-row slices.
//! Each row of `src` blits onto each of those slices.
//! Internally, [`blit_to_buffer`] calls [`get_blit_slices`]
//! If you know that you're going to blit `src` to `dst` repeatedly
//! (for example, during an animation),
//! you should instead use [`get_blit_slices`] and [`blit_to_slices`],
//! thereby reusing the destination slices.
//!
//! ```
//! use blittle::{blit_to_slices, get_dst_slices, stride::RGB};
//!
//! // The dimensions and byte data of the source image.
//! let src_w = 32;
//! let src_h = 17;
//! let src = vec![0u8; src_w * src_h * RGB];
//!
//! // The dimensions and byte data of the destination image.
//! let dst_w = 64;
//! let dst_h = 64;
//! let mut dst = vec![0u8; dst_w * dst_h * RGB];
//!
//! // Convert `dst` into predefined slices.
//! let mut dst_slices = get_dst_slices(&mut dst, 2, 12, dst_w, src_w, src_h, RGB);
//!
//! // Blit `src` onto `dst`.
//! // In an animation, the content of `src` would change every iteration.
//! for _ in 0..100 {
//!     blit_to_slices(&src, &mut dst_slices, src_w, RGB);
//! }
//! ```

pub mod stride;

use std::slice::from_raw_parts_mut;

/// A vec of slices of a `dst` vec. See: [`get_dst_slices`]
pub type DstSlices<'b> = Vec<&'b mut [u8]>;

/// Blit `src` onto `dst` starting at the top-left position of `(dst_x, dst_y)`.
/// `src` and `dst` are flat byte slices of images.
/// There are many ways to cast your pixel map to `[u8]`, such as with the `bytemuck` crate.
///
/// `dst_w` and `src_w` are the width of the `src` image and `dst` image, respectively.
///
/// `stride` is the per-pixel stride length.
/// For example, an 8-bit RGB pixel has a stride length of 3 (3 channels, 1 byte per channel).
/// See `crate::stride` for some common stride values.
///
/// Internally, this crates a [`DstSlices`] from `dst`.
/// If you want to repeatedly blit a pixel map the same size as `src` to the same destination region,
/// (for example, for a sprite animation),
/// consider using [`bit_to_slices`], which recycles the [`DstSlices`] and is therefore faster.
pub fn blit_to_buffer(
    src: &[u8],
    dst: &mut [u8],
    dst_x: usize,
    dst_y: usize,
    dst_w: usize,
    src_w: usize,
    stride: usize,
) {
    let src_h = (src.len() / stride) / src_w;
    let mut dst = get_dst_slices(dst, dst_x, dst_y, dst_w, src_w, src_h, stride);
    blit_to_slices(src, &mut dst, src_w, stride)
}

/// Chops of `dst` into multiple slices and returns them.
/// These slices constitute a region onto which a `src` slice can be blitted.
///
/// Use this in conjunction with [`blit_to_slices`].
/// Recycling the same [`DstSlices`] is faster than recreating them,
/// which is what [`blit_to_buffer`] does.
///
/// The top-left position of the blittable region is `(dst_x, dst_y)`.
/// The width of the `dst` image is `dst_w`.
/// The width and height of the `src` image is `(src_w, src_h)`.
///
/// `stride` is the per-pixel stride length.
/// For example, an 8-bit RGB pixel has a stride length of 3 (3 channels, 1 byte per channel).
/// See `crate::stride` for some common stride values.
pub fn get_dst_slices(
    dst: &mut [u8],
    dst_x: usize,
    dst_y: usize,
    dst_w: usize,
    src_w: usize,
    src_h: usize,
    stride: usize,
) -> DstSlices {
    let ptr = dst.as_mut_ptr();
    let src_w_stride = src_w * stride;
    let src_h = src_h.min((dst.len() / stride) / dst_w);
    (0..src_h)
        .map(|src_y| unsafe {
            let dst_index = to_index(dst_x, dst_y + src_y, dst_w, stride);
            from_raw_parts_mut(ptr.add(dst_index), src_w_stride)
        })
        .collect::<Vec<&mut [u8]>>()
}

/// Blit `src` onto `dst`. To create `dst`, see: [`get_dst_slices`].
///
/// `src_w` is the width of the `src` image.
///
/// `stride` is the per-pixel stride length.
/// For example, an 8-bit RGB pixel has a stride length of 3 (3 channels, 1 byte per channel).
/// See `crate::stride` for some common stride values.
pub fn blit_to_slices(src: &[u8], dst: &mut DstSlices, src_w: usize, stride: usize) {
    let src_w_stride = stride * src_w;

    dst.iter_mut().enumerate().for_each(|(src_y, dst_slice)| {
        let src_index = to_index(0, src_y, src_w, stride);
        dst_slice.copy_from_slice(&src[src_index..src_index + src_w_stride]);
    });
}

/// Clip `dst_x`, `dst_y`, and `src_w` to be within the bounds of the size of `dst`.
pub fn clip(
    dst: &[u8],
    dst_x: &mut usize,
    dst_y: &mut usize,
    dst_w: usize,
    src_w: &mut usize,
    stride: usize,
) {
    let dst_h = (dst.len() / stride) / dst_w;
    *dst_x = (*dst_x).min(dst_w);
    *dst_y = (*dst_y).min(dst_h);
    *src_w = (*src_w).min(dst_w);
}

const fn to_index(x: usize, y: usize, w: usize, stride: usize) -> usize {
    (x + y * w) * stride
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stride::RGB;
    use bytemuck::{cast_slice, cast_slice_mut};
    use std::{fs::File, io::BufWriter, path::Path};

    #[test]
    fn test_blit() {
        const SRC_W: usize = 32;
        const SRC_H: usize = 17;
        const DST_W: usize = 64;
        const DST_H: usize = 64;
        let src = [[255u8, 0, 0]; SRC_W * SRC_H];
        let mut dst = [[0u8, 0, 255]; DST_W * DST_H];

        let src = cast_slice::<[u8; RGB], u8>(&src);
        let dst = cast_slice_mut::<[u8; RGB], u8>(&mut dst);

        blit_to_buffer(src, dst, 2, 12, DST_W, SRC_W, RGB);
        
        save_png("blit.png", dst, DST_W as u32, DST_H as u32);
    }

    #[test]
    fn test_clip() {
        let dst_w = 32;
        let dst = vec![0u8; dst_w * dst_w];
        let mut dst_x = dst_w + 2;
        let mut dst_y = 0;
        let mut src_w = 64;
        clip(&dst, &mut dst_x, &mut dst_y, dst_w, &mut src_w, 1);
        assert_eq!(dst_x, dst_w);
        assert_eq!(dst_y, 0);
        assert_eq!(src_w, 32);
    }
    
    #[test]
    fn test_clip_blit() {
        const SRC_W: usize = 64;
        const SRC_H: usize = 128;
        const DST_W: usize = 32;
        const DST_H: usize = 32;
        let src = [[255u8, 0, 0]; SRC_W * SRC_H];
        let mut dst = [[0u8, 0, 255]; DST_W * DST_H];
        let src = cast_slice::<[u8; RGB], u8>(&src);
        let dst = cast_slice_mut::<[u8; RGB], u8>(&mut dst);
        
        let mut dst_x = 16;
        let mut dst_y = 16;
        let mut src_w = SRC_W;
        clip(&dst, &mut dst_x, &mut dst_y, DST_W, &mut src_w, RGB);
        blit_to_buffer(src, dst, dst_x, dst_y, DST_W, src_w, RGB);
        save_png("clip.png", dst, DST_W as u32, DST_H as u32);
    }
    
    fn save_png(path: &str, dst: &[u8], dst_w: u32, dst_h: u32) {
        let path = Path::new(path);
        let file = File::create(path).unwrap();
        let w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, dst_w, dst_h);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(dst).unwrap();
    }
}
