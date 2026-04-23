use super::PixelConverter;
use crate::Surface;

macro_rules! impl_from_surface {
    ($from:ty, $to:ty, $converter:ident) => {
        #[cfg(feature = "std")]
        impl<S: AsRef<[$from]> + AsMut<[$from]>> From<&Surface<'_, S, $from>>
            for Surface<'_, Vec<$to>, $to>
        {
            fn from(value: &Surface<'_, S, $from>) -> Self {
                let buffer = value
                    .buffer()
                    .iter()
                    .map(|pixel| Surface::<S, $from>::$converter(pixel))
                    .collect();
                Self {
                    size: value.size,
                    buffer,
                    destination_rect: value.destination_rect,
                    blit_area: value.blit_area,
                    _p: std::marker::PhantomData::default(),
                }
            }
        }
    };
}

impl_from_surface!(u8, [u8; 2], pixel_to_la8);
impl_from_surface!(u8, f32, pixel_to_l32);
impl_from_surface!(u8, [f32; 2], pixel_to_la32);
impl_from_surface!(u8, [u8; 3], pixel_to_rgb8);
impl_from_surface!(u8, [u8; 4], pixel_to_rgba8);
impl_from_surface!(u8, [f32; 4], pixel_to_rgba32);

impl_from_surface!([u8; 2], u8, pixel_to_l8);
impl_from_surface!([u8; 2], f32, pixel_to_l32);
impl_from_surface!([u8; 2], [f32; 2], pixel_to_la32);
impl_from_surface!([u8; 2], [u8; 3], pixel_to_rgb8);
impl_from_surface!([u8; 2], [u8; 4], pixel_to_rgba8);
impl_from_surface!([u8; 2], [f32; 4], pixel_to_rgba32);

impl_from_surface!(f32, u8, pixel_to_l8);
impl_from_surface!(f32, [u8; 2], pixel_to_la8);
impl_from_surface!(f32, [f32; 2], pixel_to_la32);
impl_from_surface!(f32, [u8; 3], pixel_to_rgb8);
impl_from_surface!(f32, [u8; 4], pixel_to_rgba8);
impl_from_surface!(f32, [f32; 4], pixel_to_rgba32);

impl_from_surface!([f32; 2], u8, pixel_to_l8);
impl_from_surface!([f32; 2], [u8; 2], pixel_to_la8);
impl_from_surface!([f32; 2], f32, pixel_to_l32);
impl_from_surface!([f32; 2], [u8; 3], pixel_to_rgb8);
impl_from_surface!([f32; 2], [u8; 4], pixel_to_rgba8);
impl_from_surface!([f32; 2], [f32; 4], pixel_to_rgba32);

impl_from_surface!([u8; 3], u8, pixel_to_l8);
impl_from_surface!([u8; 3], [u8; 2], pixel_to_la8);
impl_from_surface!([u8; 3], f32, pixel_to_l32);
impl_from_surface!([u8; 3], [f32; 2], pixel_to_la32);
impl_from_surface!([u8; 3], [u8; 4], pixel_to_rgba8);
impl_from_surface!([u8; 3], [f32; 4], pixel_to_rgba32);

impl_from_surface!([u8; 4], u8, pixel_to_l8);
impl_from_surface!([u8; 4], [u8; 2], pixel_to_la8);
impl_from_surface!([u8; 4], f32, pixel_to_l32);
impl_from_surface!([u8; 4], [f32; 2], pixel_to_la32);
impl_from_surface!([u8; 4], [u8; 3], pixel_to_rgb8);
impl_from_surface!([u8; 4], [f32; 4], pixel_to_rgba32);

impl_from_surface!([f32; 4], u8, pixel_to_l8);
impl_from_surface!([f32; 4], [u8; 2], pixel_to_la8);
impl_from_surface!([f32; 4], f32, pixel_to_l32);
impl_from_surface!([f32; 4], [f32; 2], pixel_to_la32);
impl_from_surface!([f32; 4], [u8; 3], pixel_to_rgb8);
impl_from_surface!([f32; 4], [u8; 4], pixel_to_rgba8);

#[cfg(feature = "png")]
#[cfg(test)]
mod tests {
    use crate::png::Png;
    use crate::{L8Surface, Rgba8Surface};
    use std::env::current_dir;
    use std::io::Cursor;

    #[test]
    fn test_convert() {
        let src =
            Rgba8Surface::read_png(Cursor::new(include_bytes!("../../test_images/plasma.png")))
                .unwrap();
        let dst = L8Surface::from(&src);
        L8Surface::write_png(
            &dst,
            current_dir()
                .unwrap()
                .join("test_output/plasma_grayscale.png"),
        )
        .unwrap();
    }
}
