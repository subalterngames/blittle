#![doc = include_str!("../README.md")]

mod multi_threaded;
mod position;
mod size;
pub mod stride;
// #[cfg(feature = "rayon")]
pub use multi_threaded::*;

pub use position::*;
pub use size::Size;

/// Blit `src` onto `dst`.
///
/// - `src` and `dst` are flat byte slices of images. There are many ways to cast your pixel map to `[u8]`, such as with the `bytemuck` crate.
/// - `dst_position` is the top-left position of the region that `src` will blit onto.
/// - `dst_size` and `src_size` are the [`Size`]'s of the destination and source images, respectively.
/// - `stride` is the per-pixel stride length. See `crate::stride` for some common stride values.
pub fn blit(
    src: &[u8],
    src_size: &Size,
    dst: &mut [u8],
    dst_position: &PositionU,
    dst_size: &Size,
    stride: usize,
) {
    if src_size.w > 0 && src_size.h > 0 {
        let src_w_stride = src_size.w * stride;
        (0..src_size.h).for_each(|src_y| {
            let src_index = get_index(0, src_y, src_size.w, stride);
            let dst_index = get_index(dst_position.x, dst_position.y + src_y, dst_size.w, stride);
            dst[dst_index..dst_index + src_w_stride]
                .copy_from_slice(&src[src_index..src_index + src_w_stride]);
        });
    }
}

/// Clip `src_size` such that it fits within the rectangle defined by `dst_position` and `dst_size`.
/// Returns `dst_position` as a clipped `PositionU` that can be used in [`blit`].
pub fn clip(dst_position: &PositionI, dst_size: &Size, src_size: &mut Size) -> PositionU {
    // Check if the source image is totally out of bounds.
    if dst_position.x + (src_size.w as isize) < 0 || dst_position.y + (src_size.h as isize) < 0 {
        src_size.w = 0;
        src_size.h = 0;
        PositionU::default()
    } else {
        let mut x = 0;
        if dst_position.x < 0 {
            src_size.w = src_size.w.saturating_sub(dst_position.x.unsigned_abs());
        } else {
            x = dst_position.x.unsigned_abs();
        }
        let mut y = 0;
        if dst_position.y < 0 {
            src_size.h = src_size.h.saturating_sub(dst_position.y.unsigned_abs());
        } else {
            y = dst_position.y.unsigned_abs();
        }
        let dst_position = PositionU { x, y };
        // This allows us to do unchecked subtraction.
        // The `blit` methods will also check `is_inside`.
        if dst_position.x < dst_size.w && dst_position.y < dst_size.h {
            src_size.w = src_size.w.min(dst_size.w - dst_position.x);
            src_size.h = src_size.h.min(dst_size.h - dst_position.y);
            dst_position
        } else {
            *src_size = Size::default();
            PositionU::default()
        }
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
    use std::{fs::File, io::BufWriter, path::Path};

    const SRC_W: usize = 32;
    const SRC_H: usize = 17;
    const DST_W: usize = 64;
    const DST_H: usize = 64;

    #[test]
    fn test_blit() {
        let src = [255u8; SRC_W * SRC_H * RGB];
        let mut dst = [0u8; DST_W * DST_H * RGB];

        let dst_position = PositionU { x: 2, y: 12 };
        let dst_size = Size { w: DST_W, h: DST_H };
        let src_size = Size { w: SRC_W, h: SRC_H };

        blit(&src, &src_size, &mut dst, &dst_position, &dst_size, RGB);

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
        let mut src_size = Size { w: SRC_W, h: SRC_H };
        let dst_position = clip(&dst_position, &dst_size, &mut src_size);

        blit(&src, &src_size, &mut dst, &dst_position, &dst_size, RGB);
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
