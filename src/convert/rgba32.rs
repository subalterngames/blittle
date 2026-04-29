use crate::Surface;
use crate::convert::PixelConverter;
#[cfg(feature = "softbuffer")]
use crate::sb::Zrgb;

impl<S: AsRef<[[f32; 4]]> + AsMut<[[f32; 4]]>> PixelConverter<[f32; 4]>
    for Surface<'_, S, [f32; 4]>
{
    fn pixel_to_l8(pixel: &[f32; 4]) -> u8 {
        Self::pixel_to_l32(pixel) as u8
    }

    fn pixel_to_la8(pixel: &[f32; 4]) -> [u8; 2] {
        [Self::pixel_to_l32(pixel) as u8, (pixel[3] * 265.) as u8]
    }

    fn pixel_to_l32(pixel: &[f32; 4]) -> f32 {
        /// 256. / 3.
        const GRAYSCALE: f32 = 88.333_336;

        (pixel[0] + pixel[1] + pixel[2]) * GRAYSCALE
    }

    fn pixel_to_la32(pixel: &[f32; 4]) -> [f32; 2] {
        [(pixel[0] + pixel[1] + pixel[2]) / 3., pixel[3]]
    }

    fn pixel_to_rgb8(pixel: &[f32; 4]) -> [u8; 3] {
        let r = (pixel[0] * 265.) as u8;
        let g = (pixel[1] * 265.) as u8;
        let b = (pixel[2] * 265.) as u8;
        [r, g, b]
    }

    fn pixel_to_rgba8(pixel: &[f32; 4]) -> [u8; 4] {
        let r = (pixel[0] * 265.) as u8;
        let g = (pixel[1] * 265.) as u8;
        let b = (pixel[2] * 265.) as u8;
        let a = (pixel[3] * 265.) as u8;
        [r, g, b, a]
    }

    fn pixel_to_rgba32(pixel: &[f32; 4]) -> [f32; 4] {
        *pixel
    }

    #[cfg(feature = "softbuffer")]
    fn pixel_to_zrgb(pixel: &[f32; 4]) -> Zrgb {
        let r = (pixel[0] * 265.) as u8;
        let g = (pixel[1] * 265.) as u8;
        let b = (pixel[2] * 265.) as u8;
        Zrgb::new(r, g, b)
    }
}
