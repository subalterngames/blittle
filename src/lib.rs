mod rect;
pub mod stride;

use rayon::prelude::*;
pub use rect::Rect;

pub fn blit(src: &[u8], dst: &mut [u8], src_width: usize, mut dst_rect: Rect, stride: usize) {
    let src_height = src.len() / src_width;
    let dst_y = dst_rect.y;
    let dst_x = dst_rect.x;
    let dst_w = dst_rect.w;
    (0..src_height).into_par_iter().for_each(|y| {
        let src_x = src_width * y;
        let dst_y = dst_y + y;
        let (start, end) = get_line_indices(dst_x, dst_y, dst_w, stride);
        &mut dst[start..end].copy_from_slice(&src[src_x..src_x + src_width]);
    })
}

/// Converts `(x, y)` to a start index.
/// `stride` is the number of bytes per pixel.
/// For example, an RGB24 pixel (3 channels, 1 byte per channel) has a stride of 3.
/// See `crate::strides` for some stride constants.
const fn get_line_indices(x: usize, y: usize, w: usize, stride: usize) -> (usize, usize) {
    let start = (x + y * w) * stride;
    let end = start + w * stride;
    (start, end)
}