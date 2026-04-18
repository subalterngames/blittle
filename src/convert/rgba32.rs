use crate::convert::PixelConverter;
use crate::Surface;
use glam::{Vec4, Vec4Swizzles};
use std::ops::Deref;

impl<S: AsRef<[Vec4]> + AsMut<[Vec4]>> PixelConverter<Vec4> for Surface<'_, S, Vec4> {
    fn pixel_to_l8(pixel: &Vec4) -> u8 {
        Self::pixel_to_l32(pixel) as u8
    }

    fn pixel_to_la8(pixel: &Vec4) -> [u8; 2] {
        let pixel = pixel * 256.;
        let p = pixel.deref();
        [((p.x + p.y + p.z) / 3.) as u8, p.w as u8]
    }

    fn pixel_to_l32(pixel: &Vec4) -> f32 {
        (pixel * 256.).xyz().element_sum() / 3.
    }

    fn pixel_to_la32(pixel: &Vec4) -> [f32; 2] {
        let pixel = pixel * 256.;
        let p = pixel.deref();
        [(p.x + p.y + p.z) / 3., p.w]
    }

    fn pixel_to_rgb8(pixel: &Vec4) -> [u8; 3] {
        let pixel = pixel * 256.;
        let pixel = pixel.deref();
        [pixel.x as u8, pixel.y as u8, pixel.z as u8]
    }

    fn pixel_to_rgba8(pixel: &Vec4) -> [u8; 4] {
        let pixel = pixel * 256.;
        let pixel = pixel.deref();
        [pixel.x as u8, pixel.y as u8, pixel.z as u8, pixel.w as u8]
    }

    fn pixel_to_rgba32(pixel: &Vec4) -> Vec4 {
        *pixel
    }
}
