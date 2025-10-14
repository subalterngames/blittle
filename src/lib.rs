pub mod stride;

use rayon::prelude::*;
use std::slice::from_raw_parts_mut;

pub fn blit(
    src: &[u8],
    dst: &mut [u8],
    src_w: usize,
    dst_w: usize,
    x: usize,
    y: usize,
    stride: usize,
) {
    let src_h = (src.len() / stride) / src_w;
    let src_w_stride = stride * src_w;

    let ptr = dst.as_mut_ptr();
    (0..src_h)
        .map(|src_y| unsafe {
            let dst_index = to_index(x, y + src_y, dst_w, stride);
            (
                src_y,
                from_raw_parts_mut(ptr.offset(dst_index as isize), src_w_stride),
            )
        })
        .collect::<Vec<(usize, &mut [u8])>>()
        .into_par_iter()
        .for_each(|(src_y, dst_slice)| {
            let src_index = to_index(0, src_y, src_w, stride);
            dst_slice.copy_from_slice(&src[src_index..src_index + src_w_stride]);
        });
}

/// Source: https://ennogames.com/blog/3d-and-2d-coordinates-to-1d-indexes
const fn to_index(x: usize, y: usize, w: usize, stride: usize) -> usize {
    (x + y * w) * stride
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stride::RGB;
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::Path;
    use std::slice::{from_raw_parts, from_raw_parts_mut};

    #[test]
    fn test_blit() {
        const SRC_W: usize = 32;
        const SRC_H: usize = 17;
        const DST_W: usize = 64;
        const DST_H: usize = 64;
        let src = [[255u8, 0, 0]; SRC_W * SRC_H];
        let mut dst = [[0u8, 0, 255]; DST_W * DST_H];
        unsafe {
            let src_ptr = src.as_ptr().cast::<u8>();
            let src_len = SRC_W * SRC_H * RGB;
            let src = from_raw_parts(src_ptr, src_len);
            let dst_ptr = dst.as_mut_ptr().cast::<u8>();
            let dst_len = DST_W * DST_H * RGB;
            let dst = from_raw_parts_mut(dst_ptr, dst_len);

            blit(src, dst, SRC_W, DST_W, 2, 12, RGB);

            let path = Path::new("blit.png");
            let file = File::create(path).unwrap();
            let w = BufWriter::new(file);
            let mut encoder = png::Encoder::new(w, DST_W as u32, DST_H as u32);
            encoder.set_color(png::ColorType::Rgb);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&dst).unwrap(); // Save
        }
    }
}
