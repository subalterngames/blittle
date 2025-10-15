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
//! use blittle::{*, stride::RGB};
//!
//! // The dimensions and byte data of the source image.
//! let src_w = 32;
//! let src_h = 17;
//! let src = vec![0u8; src_w * src_h * RGB];
//! let src_size = Size { w: src_w, h: src_h };
//!
//! // The dimensions and byte data of the destination image.
//! let dst_w = 64;
//! let dst_h = 64;
//! let mut dst = vec![0u8; dst_w * dst_h * RGB];
//! let dst_position = Position { x: 2, y: 12 };
//! let dst_size = Size { w: dst_w, h: dst_h };
//!
//! // Blit `src` onto `dst`.
//! blit_to_buffer(&src, &src_size, &mut dst, &dst_position, &dst_size, RGB);
//! ```
//!
//! ## Clipping
//!
//! By default, `blittle` won't check whether your source image exceeds the bounds of the
//! destination image. This will cause your program to crash with a very opaque memory error.
//!
//! To trim the source image's blittable region, call [`clip`].
//!
//! ## Blit slices
//!
//! In `blit_to_buffer`, `dst` is divided into per-row slices.
//! Each row of `src` blits onto each of those slices.
//! Internally, [`blit_to_buffer`] calls [`get_blit_slices`]
//! If you know that you're going to blit `src` to `dst` repeatedly (e.g. during an animation),
//! you should instead use [`get_blit_slices`] and [`blit_to_slices`],
//! thereby reusing the destination slices.
//!
//! ```
//! use blittle::{blit_to_slices, get_dst_slices, Position, Size, stride::RGB};
//!
//! // The dimensions and byte data of the source image.
//! let src_w = 32;
//! let src_h = 17;
//! let src = vec![0u8; src_w * src_h * RGB];
//! let src_size = Size { w: src_w, h: src_h };
//!
//! // The dimensions and byte data of the destination image.
//! let dst_w = 64;
//! let dst_h = 64;
//! let mut dst = vec![0u8; dst_w * dst_h * RGB];
//! let dst_position = Position { x: 2, y: 12 };
//! let dst_size = Size { w: dst_w, h: dst_h };
//!
//! // Convert `dst` into predefined slices.
//! let mut dst_slices = get_dst_slices(&src_size, &mut dst, &dst_position, &dst_size, RGB).unwrap();
//!
//! // Blit `src` onto `dst`.
//! // In an animation, the content of `src` would change every iteration.
//! for _ in 0..100 {
//!     blit_to_slices(&src, &src_size, &mut dst_slices, RGB);
//! }
//! ```
//!
//! ## Benchmarks
//!
//! Run `cargo bench` and find out.

mod position;
mod size;
pub mod stride;

pub use position::Position;
pub use size::Size;
use std::slice::from_raw_parts_mut;

/// A vec of slices of a `dst` vec. See: [`get_dst_slices`]
pub type DstSlices<'b> = Vec<&'b mut [u8]>;

/// Blit `src` onto `dst`.
/// Returns true if the blit was successful (i.e. if `dst_position` is within the bounds of `dst_size`).
///
/// - `src` and `dst` are flat byte slices of images. There are many ways to cast your pixel map to `[u8]`, such as with the `bytemuck` crate.
/// - `dst_position` is the top-left position of the region that `src` will blit onto.
/// - `dst_size` and `src_size` are the [`Size`]'s of the destination and source images, respectively.
/// - `stride` is the per-pixel stride length. See `crate::stride` for some common stride values.
///
/// Internally, this crates a [`DstSlices`] from `dst`.
/// If you want to repeatedly blit a pixel map the same size as `src` to the same destination region,
/// (for example, for a sprite animation),
/// consider using [`bit_to_slices`], which recycles the [`DstSlices`] and is therefore faster.
pub fn blit_to_buffer(
    src: &[u8],
    src_size: &Size,
    dst: &mut [u8],
    dst_position: &Position,
    dst_size: &Size,
    stride: usize,
) -> bool {
    if dst_position.is_inside(&dst_size) {
        let src_w_stride = src_size.w * stride;
        (0..src_size.h).for_each(|src_y| {
            let src_index = get_index(0, src_y, src_size.w, stride);
            let dst_index = get_index(dst_position.x, dst_position.y + src_y, dst_size.w, stride);
            dst[dst_index..dst_index + src_w_stride]
                .copy_from_slice(&src[src_index..src_index + src_w_stride]);
        });
        true
    } else {
        false
    }
}

/// Chops of `dst` into multiple slices and returns them.
/// These slices constitute a region onto which a `src` slice can be blitted.
///
/// Use this in conjunction with [`blit_to_slices`].
/// Recycling the same [`DstSlices`] is faster than recreating them,
/// which is what [`blit_to_buffer`] does.
///
/// - `src` and `dst` are flat byte slices of images. There are many ways to cast your pixel map to `[u8]`, such as with the `bytemuck` crate.
/// - `dst_position` is the top-left position of the region that `src` will blit onto.
/// - `dst_size` and `src_size` are the [`Size`]'s of the destination and source images, respectively.
/// - `stride` is the per-pixel stride length. See `crate::stride` for some common stride values.
pub fn get_dst_slices<'d>(
    src_size: &Size,
    dst: &'d mut [u8],
    dst_position: &Position,
    dst_size: &Size,
    stride: usize,
) -> Option<DstSlices<'d>> {
    if dst_position.is_inside(dst_size) {
        let ptr = dst.as_mut_ptr();
        let src_w_stride = src_size.w * stride;
        Some(
            (0..src_size.h)
                .map(|src_y| unsafe {
                    let dst_index =
                        get_index(dst_position.x, dst_position.y + src_y, dst_size.w, stride);
                    from_raw_parts_mut(ptr.add(dst_index), src_w_stride)
                })
                .collect::<Vec<&mut [u8]>>(),
        )
    } else {
        None
    }
}

/// Blit `src` onto `dst`. To create `dst`, see: [`get_dst_slices`].
///
/// - `src_size` is the [`Size`] of the source image.
/// - `stride` is the per-pixel stride length. See `crate::stride` for some common stride values.
pub fn blit_to_slices(src: &[u8], src_size: &Size, dst: &mut DstSlices, stride: usize) {
    let src_w_stride = stride * src_size.w;

    dst.iter_mut().enumerate().for_each(|(src_y, dst_slice)| {
        let src_index = get_index(0, src_y, src_size.w, stride);
        dst_slice.copy_from_slice(&src[src_index..src_index + src_w_stride]);
    });
}

/// Clip `src_size` such that it fits within the rectangle defined by `dst_position` and `dst_size`.
pub fn clip(dst_position: &Position, dst_size: &Size, src_size: &mut Size) {
    // This allows us to do unchecked subtraction.
    // The `blit` methods will also check `is_inside`.
    if dst_position.is_inside(dst_size) {
        src_size.w = src_size.w.min(dst_size.w - dst_position.x);
        src_size.h = src_size.h.min(dst_size.h - dst_position.y);
    }
}

/// Converts a position, width, and stride to an index in a 1D byte slice.
pub const fn get_index(x: usize, y: usize, w: usize, stride: usize) -> usize {
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

        let dst_position = Position { x: 2, y: 12 };
        let dst_size = Size { w: DST_W, h: DST_H };
        let src_size = Size { w: SRC_W, h: SRC_H };

        blit_to_buffer(src, &src_size, dst, &dst_position, &dst_size, RGB);

        save_png("blit.png", dst, DST_W as u32, DST_H as u32);
    }

    #[test]
    fn test_clip() {
        const SRC_W: usize = 64;
        const SRC_H: usize = 128;
        const DST_W: usize = 32;
        const DST_H: usize = 32;
        let src = [[255u8, 0, 0]; SRC_W * SRC_H];
        let mut dst = [[0u8, 0, 255]; DST_W * DST_H];
        let src = cast_slice::<[u8; RGB], u8>(&src);
        let dst = cast_slice_mut::<[u8; RGB], u8>(&mut dst);

        let dst_position = Position { x: 16, y: 16 };
        let dst_size = Size { w: DST_W, h: DST_H };
        let mut src_size = Size { w: SRC_W, h: SRC_H };
        clip(&dst_position, &dst_size, &mut src_size);

        blit_to_buffer(src, &src_size, dst, &dst_position, &dst_size, RGB);
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
