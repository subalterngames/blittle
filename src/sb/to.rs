use crate::sb::{Zrgb, ZrgbSurfaceRef};
use crate::{Error, L32Surface, Surface};
use bytemuck::{Pod, Zeroable};
use glam::Vec4;
use std::ops::Deref;

macro_rules! impl_to_zrgb {
    ($p:ty, $f:expr) => {
        impl<S: AsRef<[$p]> + AsMut<[$p]>> ToZrgb<S, $p> for Surface<'_, S, $p> {
            fn pixel_to_zrgb(pixel: $p) -> Zrgb {
                $f(pixel)
            }
        }
    };
}

/// Blit a surface to a [ZrgbSurfaceRef] surface.
pub trait ToZrgb<S: AsRef<[P]> + AsMut<[P]>, P: Copy + Clone + Sized + Default + Zeroable + Pod> {
    /// Convert a pixel to a [Zrgb] pixel.
    fn pixel_to_zrgb(pixel: P) -> Zrgb;

    /// Blit this surface to `destination`.
    fn blit_to_zrgb_ref(
        surface: Surface<'_, S, P>,
        destination: &mut ZrgbSurfaceRef,
    ) -> Result<(), Error> {
        let (destination_rect, blit_area) = surface.get_blit_params()?;
        let dst_offset =
            destination.get_index(destination_rect.position.x, destination_rect.position.y);
        let len = blit_area.size.x * blit_area.size.y;
        let src_offset = blit_area.position.x + blit_area.position.y * surface.get_size().x;
        for i in 0..len {
            let src_index = src_offset + i;
            let p = surface.buffer()[src_index];
            destination.buffer[dst_offset + i] = Self::pixel_to_zrgb(p);
        }
        Ok(())
    }
}

impl_to_zrgb!(u8, |p| Zrgb::new(p, p, p));
impl_to_zrgb!([u8; 2], |p: [u8; 2]| {
    let p = p[0];
    Zrgb::new(p, p, p)
});
impl_to_zrgb!(f32, |p| {
    let p = L32Surface::f32_to_u8(p);
    Zrgb::new(p, p, p)
});
impl_to_zrgb!([f32; 2], |p: [f32; 2]| {
    let p = L32Surface::f32_to_u8(p[0]);
    Zrgb::new(p, p, p)
});
impl_to_zrgb!([u8; 3], |p: [u8; 3]| Zrgb::new(p[0], p[1], p[2]));
impl_to_zrgb!([u8; 4], |p: [u8; 4]| Zrgb::new(p[0], p[1], p[2]));
impl_to_zrgb!(Vec4, |p: Vec4| {
    let p = p * 256.;
    let p = p.deref();
    Zrgb::new(p.x as u8, p.y as u8, p.z as u8)
});
