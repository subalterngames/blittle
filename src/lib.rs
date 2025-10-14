mod rect;
pub mod stride;

use rayon::prelude::*;
pub use rect::Rect;
use std::slice::from_raw_parts_mut;

pub fn blit(src: &[u8], dst: &mut [u8], src_width: usize, dst_rect: &Rect, stride: usize) {
    let src_height = src.len() / src_width;
    let src_width_stride = src_width * stride;

    get_slices(dst, &dst_rect, stride)
        .into_par_iter()
        .zip((0..src_height).into_par_iter())
        .for_each(|(dst_slice, src_y)| {
            let x = src_width_stride * src_y;
            dst_slice.copy_from_slice(&src[x..x + src_width_stride])
        })
}

fn get_slices<'s>(buffer: &'s mut [u8], rect: &Rect, stride: usize) -> Vec<&'s mut [u8]> {
    let ptr = buffer.as_mut_ptr();
    let x = rect.x * stride;
    let len = rect.w * stride;
    (rect.y..rect.y + rect.h)
        .map(|y| unsafe {
            from_raw_parts_mut(ptr.offset(((x + y * rect.w) * stride) as isize), len)
        })
        .collect()
}
