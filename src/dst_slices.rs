use crate::to_index;
use std::slice::from_raw_parts_mut;

pub struct DstSlices<'d>(pub Vec<(usize, &'d mut [u8])>);

impl DstSlices<'_> {
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
                    (
                        src_y,
                        from_raw_parts_mut(ptr.add(dst_index), src_w_stride),
                    )
                })
                .collect::<Vec<(usize, &mut [u8])>>(),
        )
    }

    pub fn from_u32(
        dst: &mut [u32],
        dst_x: usize,
        dst_y: usize,
        dst_w: usize,
        src_w: usize,
        src_h: usize,
    ) -> Self {
        Self::from_stride(dst, dst_x, dst_y, dst_w, src_w, src_h, 4)
    }

    pub fn from_pixels<const CHANNELS: usize>(
        dst: &mut [[u8; CHANNELS]],
        dst_x: usize,
        dst_y: usize,
        dst_w: usize,
        src_w: usize,
        src_h: usize,
    ) -> Self {
        Self::from_stride(dst, dst_x, dst_y, dst_w, src_w, src_h, CHANNELS)
    }

    pub fn from_bitmap<const W: usize, const H: usize, const CHANNELS: usize>(
        dst: &mut [[[u8; CHANNELS]; H]; W],
        dst_x: usize,
        dst_y: usize,
        src_w: usize,
        src_h: usize,
    ) -> Self {
        Self::from_stride(dst, dst_x, dst_y, W, src_w, src_h, CHANNELS * H)
    }

    fn from_stride<T: Sized>(
        dst: &mut [T],
        dst_x: usize,
        dst_y: usize,
        dst_w: usize,
        src_w: usize,
        src_h: usize,
        stride: usize,
    ) -> Self {
        let ptr = dst.as_mut_ptr().cast::<u8>();
        let len = dst.len() * stride;
        unsafe {
            let dst = from_raw_parts_mut(ptr, len);
            Self::new(dst, dst_x, dst_y, dst_w, src_w, src_h, stride)
        }
    }
}
