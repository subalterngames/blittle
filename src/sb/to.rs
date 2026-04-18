use crate::sb::{Zrgb, ZrgbSurfaceRef};
use crate::surface_trait::SurfaceTrait;
use crate::{
    Error, L8Surface, L32Surface, La8Surface, La32Surface, Rgb8Surface, Rgba8Surface, Rgba32Surface,
};
use bytemuck::{Pod, Zeroable};
use glam::Vec4;
use std::ops::Deref;

/// Blit a surface to a [ZrgbSurfaceRef] surface.
pub trait ToZrgb<P: Copy + Clone + Sized + Default + Zeroable + Pod>: SurfaceTrait<P> {
    /// Convert a pixel to a [Zrgb] pixel.
    fn pixel_to_zrgb(pixel: P) -> Zrgb;

    /// Blit this surface to `destination`.
    fn blit_to_zrgb_ref(&self, destination: &mut ZrgbSurfaceRef) -> Result<(), Error> {
        let (destination_rect, blit_area) = self.get_blit_params()?;
        let dst_offset =
            destination.get_index(destination_rect.position.x, destination_rect.position.y);
        let len = blit_area.size.x * blit_area.size.y;
        let src_offset = blit_area.position.x + blit_area.position.y * self.get_size().x;
        for i in 0..len {
            let src_index = src_offset + i;
            let p = self.buffer()[src_index];
            destination.buffer[dst_offset + i] = Self::pixel_to_zrgb(p);
        }
        Ok(())
    }
}

impl ToZrgb<u8> for L8Surface<'_> {
    fn pixel_to_zrgb(pixel: u8) -> Zrgb {
        Zrgb::new(pixel, pixel, pixel)
    }
}

impl ToZrgb<[u8; 2]> for La8Surface<'_> {
    fn pixel_to_zrgb(pixel: [u8; 2]) -> Zrgb {
        let p = pixel[0];
        Zrgb::new(p, p, p)
    }
}

impl ToZrgb<f32> for L32Surface<'_> {
    fn pixel_to_zrgb(pixel: f32) -> Zrgb {
        let p = L32Surface::f32_to_u8(pixel);
        Zrgb::new(p, p, p)
    }
}

impl ToZrgb<[f32; 2]> for La32Surface<'_> {
    fn pixel_to_zrgb(pixel: [f32; 2]) -> Zrgb {
        let p = L32Surface::f32_to_u8(pixel[0]);
        Zrgb::new(p, p, p)
    }
}

impl ToZrgb<[u8; 3]> for Rgb8Surface<'_> {
    fn pixel_to_zrgb(pixel: [u8; 3]) -> Zrgb {
        Zrgb::new(pixel[0], pixel[1], pixel[2])
    }
}

impl ToZrgb<[u8; 4]> for Rgba8Surface<'_> {
    fn pixel_to_zrgb(pixel: [u8; 4]) -> Zrgb {
        Zrgb::new(pixel[0], pixel[1], pixel[2])
    }
}

impl ToZrgb<Vec4> for Rgba32Surface<'_> {
    fn pixel_to_zrgb(pixel: Vec4) -> Zrgb {
        let p = pixel * 256.;
        let p = p.deref();
        Zrgb::new(p.x as u8, p.y as u8, p.z as u8)
    }
}
