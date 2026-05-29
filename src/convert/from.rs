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
                    .map(|pixel| pixel.$converter())
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

impl_from_surface!(u8, [u8; 2], to_la8);
impl_from_surface!(u8, f32, to_l32);
impl_from_surface!(u8, [f32; 2], to_la32);
impl_from_surface!(u8, [u8; 3], to_rgb8);
impl_from_surface!(u8, [u8; 4], to_rgba8);
impl_from_surface!(u8, [f32; 4], to_rgba32);

impl_from_surface!([u8; 2], u8, to_l8);
impl_from_surface!([u8; 2], f32, to_l32);
impl_from_surface!([u8; 2], [f32; 2], to_la32);
impl_from_surface!([u8; 2], [u8; 3], to_rgb8);
impl_from_surface!([u8; 2], [u8; 4], to_rgba8);
impl_from_surface!([u8; 2], [f32; 4], to_rgba32);

impl_from_surface!(f32, u8, to_l8);
impl_from_surface!(f32, [u8; 2], to_la8);
impl_from_surface!(f32, [f32; 2], to_la32);
impl_from_surface!(f32, [u8; 3], to_rgb8);
impl_from_surface!(f32, [u8; 4], to_rgba8);
impl_from_surface!(f32, [f32; 4], to_rgba32);

impl_from_surface!([f32; 2], u8, to_l8);
impl_from_surface!([f32; 2], [u8; 2], to_la8);
impl_from_surface!([f32; 2], f32, to_l32);
impl_from_surface!([f32; 2], [u8; 3], to_rgb8);
impl_from_surface!([f32; 2], [u8; 4], to_rgba8);
impl_from_surface!([f32; 2], [f32; 4], to_rgba32);

impl_from_surface!([u8; 3], u8, to_l8);
impl_from_surface!([u8; 3], [u8; 2], to_la8);
impl_from_surface!([u8; 3], f32, to_l32);
impl_from_surface!([u8; 3], [f32; 2], to_la32);
impl_from_surface!([u8; 3], [u8; 4], to_rgba8);
impl_from_surface!([u8; 3], [f32; 4], to_rgba32);

impl_from_surface!([u8; 4], u8, to_l8);
impl_from_surface!([u8; 4], [u8; 2], to_la8);
impl_from_surface!([u8; 4], f32, to_l32);
impl_from_surface!([u8; 4], [f32; 2], to_la32);
impl_from_surface!([u8; 4], [u8; 3], to_rgb8);
impl_from_surface!([u8; 4], [f32; 4], to_rgba32);

impl_from_surface!([f32; 4], u8, to_l8);
impl_from_surface!([f32; 4], [u8; 2], to_la8);
impl_from_surface!([f32; 4], f32, to_l32);
impl_from_surface!([f32; 4], [f32; 2], to_la32);
impl_from_surface!([f32; 4], [u8; 3], to_rgb8);
impl_from_surface!([f32; 4], [u8; 4], to_rgba8);

#[cfg(feature = "softbuffer")]
mod impl_softbuffer {
    use super::*;
    use crate::sb::Zrgb;

    impl_from_surface!(u8, Zrgb, to_zrgb);
    impl_from_surface!([u8; 2], Zrgb, to_zrgb);
    impl_from_surface!(f32, Zrgb, to_zrgb);
    impl_from_surface!([f32; 2], Zrgb, to_zrgb);
    impl_from_surface!([u8; 3], Zrgb, to_zrgb);
    impl_from_surface!([u8; 4], Zrgb, to_zrgb);
    impl_from_surface!([f32; 4], Zrgb, to_zrgb);

    impl_from_surface!(Zrgb, u8, to_l8);
    impl_from_surface!(Zrgb, [u8; 2], to_la8);
    impl_from_surface!(Zrgb, f32, to_l32);
    impl_from_surface!(Zrgb, [f32; 2], to_la32);
    impl_from_surface!(Zrgb, [u8; 3], to_rgb8);
    impl_from_surface!(Zrgb, [u8; 4], to_rgba8);
}

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
