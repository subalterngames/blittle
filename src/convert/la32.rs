use crate::Surface;
use crate::convert::{PixelConverter, f32_to_u8};

impl<S: AsRef<[[f32; 2]]> + AsMut<[[f32; 2]]>> PixelConverter<[f32; 2]>
    for Surface<'_, S, [f32; 2]>
{
    fn pixel_to_l8(pixel: &[f32; 2]) -> u8 {
        f32_to_u8(pixel[0])
    }

    fn pixel_to_la8(pixel: &[f32; 2]) -> [u8; 2] {
        [f32_to_u8(pixel[0]), f32_to_u8(pixel[1])]
    }

    fn pixel_to_l32(pixel: &[f32; 2]) -> f32 {
        pixel[0]
    }

    fn pixel_to_la32(pixel: &[f32; 2]) -> [f32; 2] {
        *pixel
    }

    fn pixel_to_rgb8(pixel: &[f32; 2]) -> [u8; 3] {
        [f32_to_u8(pixel[0]); 3]
    }

    fn pixel_to_rgba8(pixel: &[f32; 2]) -> [u8; 4] {
        let p = f32_to_u8(pixel[0]);
        [p, p, p, f32_to_u8(pixel[1])]
    }

    fn pixel_to_rgba32(pixel: &[f32; 2]) -> [f32; 4] {
        let p = pixel[0];
        [p, p, p, pixel[1]]
    }
}
