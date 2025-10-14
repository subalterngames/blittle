pub mod stride;

use rayon::prelude::*;
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
pub fn blit_to_slices<'d>(src: &[u8], dst: &'d mut DstSlices<'d>, src_w: usize, stride: usize) {
    let src_w_stride = stride * src_w;

    dst.into_par_iter()
        .enumerate()
        .for_each(|(src_y, dst_slice)| {
            let src_index = to_index(0, src_y, src_w, stride);
            dst_slice.copy_from_slice(&src[src_index..src_index + src_w_stride]);
        });
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

        let path = Path::new("blit.png");
        let file = File::create(path).unwrap();
        let w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, DST_W as u32, DST_H as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&dst).unwrap();
    }
}
