use crate::stride::{RGB, RGBA};
use bytemuck::{cast_slice, cast_slice_mut};
use glam::Vec4;

pub fn rgb8_to_rgba32_in_place(src: &[u8], dst: &mut [Vec4]) {
    cast_slice::<u8, [u8; RGB]>(src)
        .iter()
        .zip(dst)
        .for_each(|(src, dst)| {
            dst.x = src[0] as f32 / 255.;
            dst.y = src[1] as f32 / 255.;
            dst.z = src[2] as f32 / 255.;
            dst.w = 1.;
        })
}

pub fn rgba8_to_rgba32_in_place(src: &[u8], dst: &mut [Vec4]) {
    cast_slice::<u8, [u8; RGBA]>(src)
        .iter()
        .zip(dst)
        .for_each(|(src, dst)| {
            dst.x = src[0] as f32 / 255.;
            dst.y = src[1] as f32 / 255.;
            dst.z = src[2] as f32 / 255.;
            dst.w = src[3] as f32 / 255.;
        });
}

pub fn rgba32_to_rgb8_in_place(src: &[Vec4], dst: &mut [u8]) {
    src.iter()
        .zip(cast_slice_mut::<u8, [u8; RGB]>(dst))
        .for_each(|(src, dst)| {
            dst[0] = (src.x * 255.) as u8;
            dst[1] = (src.y * 255.) as u8;
            dst[2] = (src.z * 255.) as u8;
        })
}

pub fn rgba32_to_rgb32_in_place(src: &[Vec4], dst: &mut [u8]) {
    src.iter()
        .zip(cast_slice_mut::<u8, [u8; RGBA]>(dst))
        .for_each(|(src, dst)| {
            dst[0] = (src.x * 255.) as u8;
            dst[1] = (src.y * 255.) as u8;
            dst[2] = (src.z * 255.) as u8;
            dst[3] = (src.w * 255.) as u8;
        })
}
