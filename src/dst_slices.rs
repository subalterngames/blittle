use crate::to_index;
use std::slice::from_raw_parts_mut;

pub struct DstSlices<'d>(pub Vec<&'d mut [u8]>);

impl<'d> DstSlices<'d> {
    pub fn new(
        dst: &mut [u8],
        dst_x: usize,
        dst_y: usize,
        dst_w: usize,
        src_w: usize,
        src_h: usize,
        stride: usize,
    ) -> Self {
        let ptr = dst.as_mut_ptr();
        let src_w_stride = src_w * stride;
        Self(
            (0..src_h)
                .map(|src_y| unsafe {
                    let dst_index = to_index(dst_x, dst_y + src_y, dst_w, stride);

                    from_raw_parts_mut(ptr.add(dst_index), src_w_stride)
                })
                .collect::<Vec<&mut [u8]>>(),
        )
    }

    pub fn slices(&'d mut self) -> &'d mut [&'d mut [u8]] {
        &mut self.0
    }
}
