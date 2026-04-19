use crate::Surface;
use crate::sb::{SoftbufferSurface, Zrgb};
use crate::{L8Surface, L32Surface, Rgb8Surface};
use bytemuck::{Pod, Zeroable};
use glam::Vec4;
use std::marker::PhantomData;
use std::ops::Deref;

macro_rules! impl_from_zrgb {
    ($p:ty, $f:expr) => {
        impl<S: AsRef<[$p]> + AsMut<[$p]> + FromIterator<$p>> FromZrgb<S, $p>
            for Surface<'_, S, $p>
        {
            fn zrgb_to_pixel(pixel: &Zrgb) -> $p {
                $f(pixel)
            }
        }
    };
}

/// Convert from a [SoftbufferSurface] to a surface.
pub trait FromZrgb<
    S: AsRef<[P]> + AsMut<[P]> + FromIterator<P>,
    P: Copy + Clone + Sized + Default + Zeroable + Pod,
>
{
    fn zrgb_to_pixel(pixel: &Zrgb) -> P;

    fn from_zrgb<'s>(surface: &SoftbufferSurface) -> Surface<'s, S, P> {
        let buffer = surface
            .buffer
            .iter()
            .map(|pixel| Self::zrgb_to_pixel(pixel))
            .collect();
        Surface {
            size: surface.size,
            buffer,
            destination_rect: surface.destination_rect,
            blit_area: surface.blit_area,
            _p: PhantomData,
        }
    }
}

impl_from_zrgb!(u8, zrgb_to_u8);
impl_from_zrgb!([u8; 2], |p| [zrgb_to_u8(p), 255]);
impl_from_zrgb!(f32, zrgb_to_f32);
impl_from_zrgb!([f32; 2], |p| [zrgb_to_f32(p), 1.]);
impl_from_zrgb!([u8; 3], |p: &Zrgb| {
    let p = p.deref();
    [p[1], p[2], p[3]]
});
impl_from_zrgb!([u8; 4], |p: &Zrgb| {
    let p = p.deref();
    [p[1], p[2], p[3], 255]
});
impl_from_zrgb!(Vec4, |p: &Zrgb| {
    let p = p.deref();
    Vec4::new(
        L8Surface::u8_to_f32(p[1]),
        L8Surface::u8_to_f32(p[2]),
        L8Surface::u8_to_f32(p[3]),
        1.,
    )
});

#[inline]
fn zrgb_to_f32(pixel: &Zrgb) -> f32 {
    let p = pixel.deref();
    Rgb8Surface::grayscale(p[1], p[2], p[3])
}

#[inline]
fn zrgb_to_u8(pixel: &Zrgb) -> u8 {
    L32Surface::f32_to_u8(zrgb_to_f32(pixel))
}
