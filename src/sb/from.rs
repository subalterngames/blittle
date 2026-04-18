use crate::Surface;
use crate::sb::{Zrgb, ZrgbSurfaceRef};
use crate::{
    L8Surface, L32Surface, La8Surface, La32Surface, Rgb8Surface, Rgba8Surface, Rgba32Surface,
};
use bytemuck::{Pod, Zeroable};
use glam::Vec4;
use std::marker::PhantomData;
use std::ops::Deref;

/// Convert from a [ZrgbSurfaceRef] to a surface.
pub trait FromZrgb<
    S: AsRef<[P]> + AsMut<[P]> + FromIterator<P>,
    P: Copy + Clone + Sized + Default + Zeroable + Pod,
>
{
    fn zrgb_to_pixel(pixel: &Zrgb) -> P;

    fn from_zrgb<'s>(surface: &ZrgbSurfaceRef) -> Surface<'s, S, P> {
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

impl FromZrgb<Vec<u8>, u8> for L8Surface<'_> {
    fn zrgb_to_pixel(pixel: &Zrgb) -> u8 {
        zrgb_to_u8(pixel)
    }
}

impl FromZrgb<Vec<[u8; 2]>, [u8; 2]> for La8Surface<'_> {
    fn zrgb_to_pixel(pixel: &Zrgb) -> [u8; 2] {
        [zrgb_to_u8(pixel), 255]
    }
}

impl FromZrgb<Vec<f32>, f32> for L32Surface<'_> {
    fn zrgb_to_pixel(pixel: &Zrgb) -> f32 {
        zrgb_to_f32(pixel)
    }
}

impl FromZrgb<Vec<[f32; 2]>, [f32; 2]> for La32Surface<'_> {
    fn zrgb_to_pixel(pixel: &Zrgb) -> [f32; 2] {
        [zrgb_to_f32(pixel), 1.]
    }
}

impl FromZrgb<Vec<[u8; 3]>, [u8; 3]> for Rgb8Surface<'_> {
    fn zrgb_to_pixel(pixel: &Zrgb) -> [u8; 3] {
        let p = pixel.deref();
        [p[1], p[2], p[3]]
    }
}

impl FromZrgb<Vec<[u8; 4]>, [u8; 4]> for Rgba8Surface<'_> {
    fn zrgb_to_pixel(pixel: &Zrgb) -> [u8; 4] {
        let p = pixel.deref();
        [0, p[1], p[2], p[3]]
    }
}

impl FromZrgb<Vec<Vec4>, Vec4> for Rgba32Surface<'_> {
    fn zrgb_to_pixel(pixel: &Zrgb) -> Vec4 {
        let p = pixel.deref();
        Vec4::new(
            L8Surface::u8_to_f32(p[1]),
            L8Surface::u8_to_f32(p[2]),
            L8Surface::u8_to_f32(p[3]),
            1.,
        )
    }
}

#[inline]
fn zrgb_to_f32(pixel: &Zrgb) -> f32 {
    let p = pixel.deref();
    Rgb8Surface::grayscale(p[1], p[2], p[3])
}

#[inline]
fn zrgb_to_u8(pixel: &Zrgb) -> u8 {
    L32Surface::f32_to_u8(zrgb_to_f32(pixel))
}
