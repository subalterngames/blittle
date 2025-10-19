use std::slice::{from_raw_parts, from_raw_parts_mut};

use crate::{PositionU, Size, get_index};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
pub use rayon::max_num_threads;

/// Blit using multiple threads by dividing `src` and `dst` into chunks and blitting each in parallel.
///
/// This can be either slower or faster than `blit` depending on the size of `src` and the number of threads you want/can use.
/// Adjust `num_threads` accordingly:
///
/// - You don't want this to be more than the nmax number of threads available.
/// - If you use too many threads for small images, this function can be slower than `blit` due to the overhead of spawning/joining threads.
pub fn blit_multi_threaded(
    src: &[u8],
    src_size: &Size,
    dst: &mut [u8],
    dst_position: &PositionU,
    dst_size: &Size,
    stride: usize,
    num_threads: usize,
) {
    if src_size.w > 0 && src_size.h > 0 {
        let src_ptr = src.as_ptr();
        let dst_ptr = dst.as_mut_ptr();
        let src_w_stride = src_size.w * stride;

        // Divide into slices.
        let slices = (0..src_size.h)
            .map(|src_y| {
                let src_index = get_index(0, src_y, src_size.w, stride);
                let dst_index =
                    get_index(dst_position.x, dst_position.y + src_y, dst_size.w, stride);
                unsafe {
                    (
                        from_raw_parts(src_ptr.add(src_index), src_w_stride),
                        from_raw_parts_mut(dst_ptr.add(dst_index), src_w_stride),
                    )
                }
            })
            .collect::<Vec<(&[u8], &mut [u8])>>();

        // Iterate through chunks of slices.
        let chunk_size = src.len() / num_threads;
        slices
            .into_par_iter()
            .chunks(chunk_size)
            .for_each(|slices| {
                slices.into_iter().for_each(|(src, dst)| {
                    dst.copy_from_slice(src);
                });
            });
    }
}
