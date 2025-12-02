use crate::stride::{RGB, RGBA};
use crate::{PositionU, Size};
use bytemuck::{cast_slice, cast_slice_mut};
pub use glam::Vec4;

/// Convert a bitmap of RGB8 pixels (1 byte per channel) into a slice of [`Vec4`].
///
/// The size of `dst` must be: `src.len() / 3`.
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

/// Convert a bitmap of RGBA8 pixels (1 byte per channel) into a slice of [`Vec4`].
///
/// The size of `dst` must be: `src.len() / 4`.
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

/// Convert a bitmap of RGB32 pixels (4 bytes per channel) into a raw RGB8 byte slice.
///
/// The size of `dst` must be: `src.len() * 3`.
pub fn rgba32_to_rgb8_in_place(src: &[Vec4], dst: &mut [u8]) {
    src.iter()
        .zip(cast_slice_mut::<u8, [u8; RGB]>(dst))
        .for_each(|(src, dst)| {
            dst[0] = (src.x * 255.) as u8;
            dst[1] = (src.y * 255.) as u8;
            dst[2] = (src.z * 255.) as u8;
        })
}

/// Convert a bitmap of RGBA32 pixels (4 bytes per channel) into a raw RGBA byte slice.
///
/// The size of `dst` must be: `src.len() * 4`.
pub fn rgba32_to_rgba8_in_place(src: &[Vec4], dst: &mut [u8]) {
    src.iter()
        .zip(cast_slice_mut::<u8, [u8; RGBA]>(dst))
        .for_each(|(src, dst)| {
            dst[0] = (src.x * 255.) as u8;
            dst[1] = (src.y * 255.) as u8;
            dst[2] = (src.z * 255.) as u8;
            dst[3] = (src.w * 255.) as u8;
        })
}

/// Convert a bitmap of RGB8 pixels (1 byte per channel) into a slice of [`Vec4`].
pub fn rgb8_to_rgba32(src: &[u8]) -> Vec<Vec4> {
    let mut dst = vec![Vec4::default(); src.len() / RGB];
    rgb8_to_rgba32_in_place(src, &mut dst);
    dst
}

/// Convert a bitmap of RGBA8 pixels (1 byte per channel) into a slice of [`Vec4`].
pub fn rgba8_to_rgba32(src: &[u8]) -> Vec<Vec4> {
    let mut dst = vec![Vec4::default(); src.len() / RGBA];
    rgba8_to_rgba32_in_place(src, &mut dst);
    dst
}

/// Convert a bitmap of RGB32 pixels (4 bytes per channel) into a raw RGB8 byte slice.
pub fn rgba32_to_rgb8(src: &[Vec4]) -> Vec<u8> {
    let mut dst = vec![0; src.len() * RGB];
    rgba32_to_rgb8_in_place(src, &mut dst);
    dst
}

/// Convert a bitmap of RGBA32 pixels (4 bytes per channel) into a raw RGBA byte slice.
pub fn rgba32_to_rgba8(src: &[Vec4]) -> Vec<u8> {
    let mut dst = vec![0; src.len() * RGBA];
    rgba32_to_rgba8_in_place(src, &mut dst);
    dst
}

/// Overlay `src` onto `dst` using an `alpha` value.
///
/// - `src` is an RGB8 (1 byte per channel) image.
/// - `src_size` is the size of the region of `src` that will be blitted.
/// - `dst` is an RGBA32 (4 bytes per channel) image.
/// - `dst_position` is the top-left position of the region that `src` will blit onto.
/// - `dst_size` is the [`Size`]'s of the destination image.
/// - `alpha` is the alpha channel. 0 is totally transparent.
///
/// Returns `src` as an RGBA32 bitmap (see: [`rgb8_to_rgba32`]).
pub fn overlay_rgb8(
    src: &[u8],
    src_size: &Size,
    dst: &mut [Vec4],
    dst_position: &PositionU,
    dst_size: &Size,
    alpha: u8,
) -> Vec<Vec4> {
    if src_size.w > 0 && src_size.h > 0 && alpha > 0 {
        let src = rgb8_to_rgba32(src);
        (0..src_size.h).for_each(|src_y| {
            let src_index = get_index(0, src_y, src_size.w);
            let dst_index = get_index(dst_position.x, dst_position.y + src_y, dst_size.w);
            src[src_index..src_index + src_size.w]
                .iter()
                .zip(dst[dst_index..dst_index + src_size.w].iter_mut())
                .for_each(|(src, dst)| {
                    // Replace the values.
                    if alpha == 255 {
                        dst.x = src.x;
                        dst.y = src.y;
                        dst.z = src.z;
                        dst.w = 1.;
                    } else {
                        overlay_pixel(src, dst);
                    }
                });
        });
        src
    } else {
        Vec::default()
    }
}

/// Overlay `src` onto `dst`.
///
/// - `src` is an RGBA8 (1 byte per channel) image.
/// - `src_size` is the size of the region of `src` that will be blitted.
/// - `dst` is an RGBA32 (4 bytes per channel) image.
/// - `dst_position` is the top-left position of the region that `src` will blit onto.
/// - `dst_size` is the [`Size`]'s of the destination image.
///
/// Returns `src` as an RGBA32 bitmap (see: [`rgba8_to_rgba32`]).
pub fn overlay_rgba8(
    src: &[u8],
    src_size: &Size,
    dst: &mut [Vec4],
    dst_position: &PositionU,
    dst_size: &Size,
) -> Vec<Vec4> {
    if src_size.w > 0 && src_size.h > 0 {
        let src = rgba8_to_rgba32(src);
        overlay_rgba32(&src, src_size, dst, dst_position, dst_size);
        src
    } else {
        Vec::default()
    }
}

/// Overlay `src` onto `dst`.
///
/// - `src` is an RGBA32 (4 bytes per channel) image.
/// - `src_size` is the size of the region of `src` that will be blitted.
/// - `dst` is an RGBA32 (4 bytes per channel) image.
/// - `dst_position` is the top-left position of the region that `src` will blit onto.
/// - `dst_size` is the [`Size`]'s of the destination image.
pub fn overlay_rgba32(
    src: &[Vec4],
    src_size: &Size,
    dst: &mut [Vec4],
    dst_position: &PositionU,
    dst_size: &Size,
) {
    if src_size.w > 0 && src_size.h > 0 {
        (0..src_size.h).for_each(|src_y| {
            let src_index = get_index(0, src_y, src_size.w);
            let dst_index = get_index(dst_position.x, dst_position.y + src_y, dst_size.w);
            src[src_index..src_index + src_size.w]
                .iter()
                .zip(dst[dst_index..dst_index + src_size.w].iter_mut())
                .for_each(|(src, dst)| {
                    overlay_pixel(src, dst);
                });
        });
    }
}

fn overlay_pixel(src: &Vec4, dst: &mut Vec4) {
    let src = *src;
    // Alpha midpoint.
    let a = dst.midpoint(src).w;
    // Lerp to `src`.
    *dst = dst.lerp(src, a);
}

const fn get_index(x: usize, y: usize, w: usize) -> usize {
    x + y * w
}
