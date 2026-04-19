use crate::Surface;
use crate::sb::{SoftbufferSurface, Zrgb};
use crate::{L8Surface, L32Surface, Rgb8Surface};
use bytemuck::{Pod, Zeroable};
use glam::Vec4;
use std::marker::PhantomData;
use std::ops::Deref;

macro_rules! impl_from_zrgb {
    ($p:ty, $f:expr) => {
        impl FromZrgb<$p> for Surface<'_, Vec<$p>, $p> {
            fn zrgb_to_pixel(pixel: &Zrgb) -> $p {
                $f(pixel)
            }
        }
    };
}

/// Convert from a [SoftbufferSurface] to a surface.
pub trait FromZrgb<P: Copy + Clone + Sized + Default + Zeroable + Pod> {
    /// Convert a [Zrgb] pixel into my pixel type.
    fn zrgb_to_pixel(pixel: &Zrgb) -> P;

    /// Convert a [SoftbufferSurface] to a surface of this type.
    fn from_zrgb<'s>(surface: &SoftbufferSurface) -> Surface<'s, Vec<P>, P> {
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
