mod threaded_blit_params;

use std::slice::{from_raw_parts, from_raw_parts_mut};

use crate::{PositionU, Size, get_index};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
pub use threaded_blit_params::*;

pub fn blit_thread_ex(
    src: &[u8],
    src_size: &Size,
    dst: &mut [u8],
    dst_position: &PositionU,
    dst_size: &Size,
    stride: usize,
    params: &ThreadedBlitParams,
) {
    if src_size.w > 0 && src_size.h > 0 {
        let src_ptr = src.as_ptr();
        let dst_ptr = dst.as_mut_ptr();
        let src_w_stride = src_size.w * stride;

        // Divide into slices.
        let slices = (0..src_size.h).map(|src_y| {
            let src_index = get_index(0, src_y, src_size.w, stride);
            let dst_index = get_index(dst_position.x, dst_position.y + src_y, dst_size.w, stride);
            unsafe {
                (
                    from_raw_parts(src_ptr.add(src_index), src_w_stride),
                    from_raw_parts_mut(dst_ptr.add(dst_index), src_w_stride)
                )
            }
        }).collect::<Vec<(&[u8], &mut [u8])>>();

        // Iterate through chunks of slices.
        let chunk_size = params.get_chunk_size(src_size.h) * stride;
        slices.into_par_iter().chunks(chunk_size).for_each(|slices| {
            slices.into_iter().for_each(|(src, dst)| {
                dst.copy_from_slice(src);
            });
        });
        
    }
}
