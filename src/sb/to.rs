use crate::sb::{SoftbufferSurface, Zrgb};
use crate::{Error, L32Surface, Surface};
use bytemuck::{Pod, Zeroable};

macro_rules! impl_to_zrgb {
    ($p:ty, $f:expr) => {
        impl<S: AsRef<[$p]> + AsMut<[$p]>> ToZrgb<S, $p> for Surface<'_, S, $p> {
            fn pixel_to_zrgb(pixel: $p) -> Zrgb {
                $f(pixel)
            }
        }
    };
}

/// Blit a surface to a [SoftbufferSurface] surface.
pub trait ToZrgb<S: AsRef<[P]> + AsMut<[P]>, P: Copy + Clone + Sized + Default + Zeroable + Pod> {
    /// Convert a pixel to a [Zrgb] pixel.
    fn pixel_to_zrgb(pixel: P) -> Zrgb;

    /// Blit this surface to `destination` while converting pixels.
    fn blit_to_zrgb_ref(
        surface: Surface<'_, S, P>,
        destination: &mut SoftbufferSurface,
    ) -> Result<(), Error> {
        let (destination_rect, blit_area) = surface.get_blit_params(destination.size)?;
        let dst_offset =
            destination.get_index(destination_rect.position.x, destination_rect.position.y);
        let len = blit_area.size.width * blit_area.size.height;
        let src_offset = blit_area.position.x + blit_area.position.y * surface.get_size().width;
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
impl_to_zrgb!([f32; 4], |p: [f32; 4]| {
    let r = (p[0] * 265.) as u8;
    let g = (p[1] * 265.) as u8;
    let b = (p[2] * 265.) as u8;
    Zrgb::new(r, g, b)
});
