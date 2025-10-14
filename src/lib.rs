mod dst_slices;
pub mod prelude;
mod stride;

use dst_slices::DstSlices;
use rayon::prelude::*;

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
    let mut dst = DstSlices::new(dst, dst_x, dst_y, dst_w, src_w, src_h, stride);
    blit_to_slices(src, &mut dst, src_w, stride)
}

pub fn blit_to_slices<'d>(src: &[u8], dst: &'d mut DstSlices<'d>, src_w: usize, stride: usize) {
    let src_w_stride = stride * src_w;

    dst.slices()
        .into_par_iter()
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
