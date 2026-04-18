use crate::{
    L8Surface, L32Surface, La8Surface, La32Surface, Rgb8Surface, Rgba8Surface, Rgba32Surface,
};
use glam::Vec4;
use std::marker::PhantomData;

mod l32;
mod l8;
mod la32;
mod la8;
mod rgb8;
mod rgba32;
mod rgba8;

/// Convert from one surface's pixel type to another surface's pixel type.
pub trait PixelConverter<P: Copy + Clone + Sized + Default> {
    fn pixel_to_l8(pixel: &P) -> u8;

    fn pixel_to_la8(pixel: &P) -> [u8; 2];

    fn pixel_to_l32(pixel: &P) -> f32;

    fn pixel_to_la32(pixel: &P) -> [f32; 2];

    fn pixel_to_rgb8(pixel: &P) -> [u8; 3];

    fn pixel_to_rgba8(pixel: &P) -> [u8; 4];

    fn pixel_to_rgba32(pixel: &P) -> Vec4;
}

macro_rules! impl_from_surface {
    ($from:ident, $to:ident, $converter:ident) => {
        impl From<&$from<'_>> for $to<'_> {
            fn from(value: &$from) -> Self {
                let buffer = value
                    .buffer
                    .iter()
                    .map(|pixel| $from::$converter(pixel))
                    .collect();
                Self {
                    size: value.size,
                    buffer,
                    destination_rect: value.destination_rect,
                    blit_area: value.blit_area,
                    _p: PhantomData::default(),
                }
            }
        }
    };
}

impl_from_surface!(L8Surface, La8Surface, pixel_to_la8);
impl_from_surface!(L8Surface, L32Surface, pixel_to_l32);
impl_from_surface!(L8Surface, La32Surface, pixel_to_la32);
impl_from_surface!(L8Surface, Rgb8Surface, pixel_to_rgb8);
impl_from_surface!(L8Surface, Rgba8Surface, pixel_to_rgba8);
impl_from_surface!(L8Surface, Rgba32Surface, pixel_to_rgba32);

impl_from_surface!(La8Surface, L8Surface, pixel_to_l8);
impl_from_surface!(La8Surface, L32Surface, pixel_to_l32);
impl_from_surface!(La8Surface, La32Surface, pixel_to_la32);
impl_from_surface!(La8Surface, Rgb8Surface, pixel_to_rgb8);
impl_from_surface!(La8Surface, Rgba8Surface, pixel_to_rgba8);
impl_from_surface!(La8Surface, Rgba32Surface, pixel_to_rgba32);

impl_from_surface!(L32Surface, L8Surface, pixel_to_l8);
impl_from_surface!(L32Surface, La8Surface, pixel_to_la8);
impl_from_surface!(L32Surface, La32Surface, pixel_to_la32);
impl_from_surface!(L32Surface, Rgb8Surface, pixel_to_rgb8);
impl_from_surface!(L32Surface, Rgba8Surface, pixel_to_rgba8);
impl_from_surface!(L32Surface, Rgba32Surface, pixel_to_rgba32);

impl_from_surface!(La32Surface, L8Surface, pixel_to_l8);
impl_from_surface!(La32Surface, La8Surface, pixel_to_la8);
impl_from_surface!(La32Surface, L32Surface, pixel_to_l32);
impl_from_surface!(La32Surface, Rgb8Surface, pixel_to_rgb8);
impl_from_surface!(La32Surface, Rgba8Surface, pixel_to_rgba8);
impl_from_surface!(La32Surface, Rgba32Surface, pixel_to_rgba32);

impl_from_surface!(Rgb8Surface, L8Surface, pixel_to_l8);
impl_from_surface!(Rgb8Surface, La8Surface, pixel_to_la8);
impl_from_surface!(Rgb8Surface, L32Surface, pixel_to_l32);
impl_from_surface!(Rgb8Surface, La32Surface, pixel_to_la32);
impl_from_surface!(Rgb8Surface, Rgba8Surface, pixel_to_rgba8);
impl_from_surface!(Rgb8Surface, Rgba32Surface, pixel_to_rgba32);

impl_from_surface!(Rgba8Surface, L8Surface, pixel_to_l8);
impl_from_surface!(Rgba8Surface, La8Surface, pixel_to_la8);
impl_from_surface!(Rgba8Surface, L32Surface, pixel_to_l32);
impl_from_surface!(Rgba8Surface, La32Surface, pixel_to_la32);
impl_from_surface!(Rgba8Surface, Rgb8Surface, pixel_to_rgb8);
impl_from_surface!(Rgba8Surface, Rgba32Surface, pixel_to_rgba32);

impl_from_surface!(Rgba32Surface, L8Surface, pixel_to_l8);
impl_from_surface!(Rgba32Surface, La8Surface, pixel_to_la8);
impl_from_surface!(Rgba32Surface, L32Surface, pixel_to_l32);
impl_from_surface!(Rgba32Surface, La32Surface, pixel_to_la32);
impl_from_surface!(Rgba32Surface, Rgb8Surface, pixel_to_rgb8);
impl_from_surface!(Rgba32Surface, Rgba8Surface, pixel_to_rgba8);
