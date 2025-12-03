#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod clip;
#[cfg(feature = "rayon")]
mod multi_threaded;
#[cfg(feature = "overlay")]
pub mod overlay;
mod position;
mod size;
pub mod stride;

#[cfg(feature = "rayon")]
pub use multi_threaded::*;
use std::slice::from_raw_parts_mut;

pub use clip::ClippedRect;
pub use position::*;
pub use size::Size;

/// Fill `buffer` with `color`.
pub fn fill<const STRIDE: usize>(buffer: &mut [u8], color: [u8; STRIDE]) {
    let ptr = buffer.as_mut_ptr().cast::<[u8; STRIDE]>();
    let len = buffer.len() / STRIDE;
    unsafe {
        from_raw_parts_mut(ptr, len).fill(color);
    }
}

/// Blit `src` onto `dst`.
///
/// - `src` and `dst` are flat byte slices of images. There are many ways to cast your pixel map to `[u8]`, such as with the `bytemuck` crate.
/// - `rect` is the [`ClippedRect`] defining the blit area.
/// - `stride` is the per-pixel stride length. See `crate::stride` for some common stride values.
pub fn blit(src: &[u8], dst: &mut [u8], rect: &ClippedRect, stride: usize) {
    let src_w = rect.src_size_clipped.w * stride;
    (0..rect.src_size_clipped.h).for_each(|src_y| {
        let src_index = get_index(0, src_y, rect.src_size.w, stride);
        let dst_index = get_index(
            rect.dst_position_clipped.x,
            rect.dst_position_clipped.y + src_y,
            rect.dst_size.w,
            stride,
        );
        dst[dst_index..dst_index + src_w].copy_from_slice(&src[src_index..src_index + src_w]);
    });
}

/// Converts a position, width, and stride to an index in a 1D byte slice.
pub const fn get_index(x: usize, y: usize, w: usize, stride: usize) -> usize {
    (x + y * w) * stride
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stride::RGB;
    use std::{fs::File, io::BufWriter, path::Path};

    const SRC_W: usize = 32;
    const SRC_H: usize = 17;
    const DST_W: usize = 64;
    const DST_H: usize = 64;

    #[test]
    fn test_blit() {
        let src = [255u8; SRC_W * SRC_H * RGB];
        let mut dst = [0u8; DST_W * DST_H * RGB];

        let dst_position = PositionI { x: 2, y: 12 };
        let dst_size = Size { w: DST_W, h: DST_H };
        let src_size = Size { w: SRC_W, h: SRC_H };
        let rect = ClippedRect::new(dst_position, dst_size, src_size).unwrap();

        blit(&src, &mut dst, &rect, RGB);

        save_png("blit.png", &dst, DST_W as u32, DST_H as u32);
    }

    #[test]
    fn test_clip() {
        blit_clipped("clip_positive.png", 42, 16);
        blit_clipped("clip_negative.png", -8, -8);
    }

    fn blit_clipped(name: &str, x: isize, y: isize) {
        let src = [255u8; SRC_W * SRC_H * RGB];
        let mut dst = [0u8; DST_W * DST_H * RGB];

        let dst_position = PositionI { x, y };
        let dst_size = Size { w: DST_W, h: DST_H };
        let src_size = Size { w: SRC_W, h: SRC_H };
        let rect = ClippedRect::new(dst_position, dst_size, src_size).unwrap();

        blit(&src, &mut dst, &rect, RGB);
        save_png(name, &dst, DST_W as u32, DST_H as u32);
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
