use crate::error::Error;
use crate::rect::{RectI, RectU};
use crate::{PositionI, PositionU, Size};
use bytemuck::{Pod, Zeroable, cast_slice, cast_slice_mut};
use glam::Vec4;
use std::marker::PhantomData;

/// Grayscale.
pub type L8Surface<'s> = Surface<'s, Vec<u8>, u8>;
/// Grayscale + alpha.
pub type La8Surface<'s> = Surface<'s, Vec<[u8; 2]>, [u8; 2]>;
/// Red, green, blue.
pub type Rgb8Surface<'s> = Surface<'s, Vec<[u8; 3]>, [u8; 3]>;
/// Red, green, blue, alpha.
pub type Rgba8Surface<'s> = Surface<'s, Vec<[u8; 4]>, [u8; 4]>;
/// 32-bit grayscale.
pub type L32Surface<'s> = Surface<'s, Vec<f32>, f32>;
/// 32-bit grayscale + alpha.
pub type La32Surface<'s> = Surface<'s, Vec<[f32; 2]>, [f32; 2]>;
/// Red, green, blue, alpha as Vec4s.
/// This uses glam for that sweet sweet SIMD, so you can't get the underlying bytes buffer.
pub type Rgba32Surface<'s> = Surface<'s, Vec<Vec4>, Vec4>;

/// A Surface is a pixel buffer, a size, and some underlying data describing how to blit it to a given destination Surface.
///
/// ## Type aliases
///
/// There are type aliases for surface of common buffer and pixel types. Consider using them!
///
/// ```
/// use blittle::{Rgba8Surface, Size};
///
/// let _ = Rgba8Surface::new(Size::new(512, 512));
/// ```
///
/// Sometimes, you will need to use a buffer that isn't a vec or is an uncommon pixel type. In those cases, see below:
///
/// ## Generics
///
/// A Surface has a lifetime and two generics: `Surface<'s, S, P>`.
///
/// - The lifetime is only relevant if the buffer is a reference, e.g. `&mut [[u8; 3]]`
/// - `S` is the type of underlying pixel buffer. This could be, for example, a vec, or a mutable slice.
/// - `P` is the type of pixel. `S` must be a buffer with elements of type `P`.
///
/// So, `Rgba8Surface` is a type alias for: `Surface<'s, Vec<[u8; 4]>, [u8; 4]>`
///
/// ```
/// use blittle::{Size, Surface};
///
/// let _ = Surface::<'_, Vec<[u8; 4]>, [u8; 4]>::new(Size::new(512, 512));
/// ```
pub struct Surface<
    's,
    S: AsRef<[P]> + AsMut<[P]>,
    P: Copy + Clone + Sized + Default + Zeroable + Pod,
> {
    pub(crate) size: Size,
    pub(crate) buffer: S,
    pub(crate) destination_rect: Option<RectU>,
    pub(crate) blit_area: Option<RectU>,
    pub(crate) _p: PhantomData<&'s P>,
}

impl<P: Copy + Clone + Sized + Default + Zeroable + Pod> Surface<'_, Vec<P>, P> {
    /// Get a new surface.
    ///
    /// The position defaults to `(0, 0)`.
    /// The underlying pixel buffer is set to the pixel's default value (i.e. `[0, 0, 0]`), length `size.width * size.height`.
    pub fn new(size: Size) -> Self {
        Self {
            size,
            buffer: vec![P::default(); size.width * size.height],
            destination_rect: None,
            blit_area: None,
            _p: PhantomData,
        }
    }

    /// Get a new surface with color `color`.
    ///
    /// The position defaults to `(0, 0)`.
    /// The underlying pixel buffer is set to `vec![color; size.width * size.height]`.
    pub fn new_filled(size: Size, color: P) -> Self {
        Self {
            size,
            buffer: vec![color; size.width * size.height],
            destination_rect: None,
            blit_area: None,
            _p: PhantomData,
        }
    }
}

impl<'s, P: Copy + Clone + Sized + Default + Zeroable + Pod> Surface<'s, &'s mut [P], P> {
    /// Get a new surface from a mutable slice.
    ///
    /// The position defaults to `(0, 0)`.
    /// The underlying pixel buffer is set to the pixel's default value (i.e. `[0, 0, 0]`), length `size.width * size.height`.
    ///
    /// Returns an error if `size.width * size.height != buffer.len()`
    pub fn new(size: Size, buffer: &'s mut [P]) -> Result<Self, Error> {
        let len = buffer.len();
        if size.width * size.height == len {
            Ok(Self {
                size,
                buffer,
                destination_rect: None,
                blit_area: None,
                _p: PhantomData,
            })
        } else {
            Err(Error::InvalidSize { len, size })
        }
    }
}

impl<S: AsRef<[P]> + AsMut<[P]>, P: Copy + Clone + Sized + Default + Zeroable + Pod>
    Surface<'_, S, P>
{
    /// Blit onto `other`.
    ///
    /// There are three constraints for blitting to `other`.
    ///
    /// 1. `other` must have the same pixel type as `self`.
    ///    For example, you can't blit an `Rgba8Surface` onto an `L8Surface`.
    ///    However, `other` can have a different *buffer* type.
    ///    For example, you can blit `Surface<'_, Vec<u8>, u8>` onto `Surface<'s, &'s mut[u8], u8>`
    ///
    /// 2. You must call [Self::set_position] before blitting.
    ///    This is because [Self::set_position] not only sets the position,
    ///    but also defines the region in which pixels in `src` actually overlap with `dst`.
    ///    If you have called [Self::set_position] for one destination,
    ///    and you are blitting to a *different* destination,
    ///    you must call [Self::set_position] again.
    ///
    /// 3. Having called [Self::set_position], some pixels of `src` must overlap with `dst`.
    ///
    /// For example, this returns an error:
    ///
    /// ```
    /// use blittle::*;
    ///  use glam::{PositionI, USizeVec2};
    ///
    /// let mut src = Rgb8Surface::new(USizeVec2::new(512, 512));
    /// let mut dst = Rgb8Surface::new(USizeVec2::new(1920, 1080));
    /// // This works. Some, or all, of src is within dst.
    /// src.set_position(PositionI::new(100, 100), &dst).unwrap();
    /// src.blit(&mut dst).unwrap();
    ///
    /// // Set a new destination.
    /// let mut dst = Rgb8Surface::new(USizeVec2::new(64, 64));
    /// // The position of `src` is obsolete, and is out of bounds.
    /// assert!(src.blit(&mut dst).is_err());
    /// ```
    pub fn blit<B: AsRef<[P]> + AsMut<[P]>>(
        &self,
        other: &mut Surface<'_, B, P>,
    ) -> Result<(), Error> {
        let (destination_rect, blit_area) = self.get_blit_params(other.size)?;
        // Iterate per-row.
        (0..blit_area.size.height).for_each(|src_y| {
            // Get the start index in the source slice.
            let src_index = self.get_index(
                blit_area.position.x,         // Blit area offset or zero
                src_y + blit_area.position.y, // y offset + blit area offset
            );
            let dst_index = other.get_index(
                destination_rect.position.x,         // Destination position (x)
                src_y + destination_rect.position.y, // y offset + destination position (y)
            );
            // Copy the slice, using the blit area's width.
            other.buffer.as_mut()[dst_index..dst_index + blit_area.size.width].copy_from_slice(
                &self.buffer.as_ref()[src_index..src_index + blit_area.size.width],
            );
        });
        Ok(())
    }

    /// Fill the surface with `color`.
    pub fn fill(&mut self, color: P) {
        self.buffer.as_mut().fill(color);
    }

    /// Set the top-left position of the surface, relative to the surface it will blit to `destination`.
    ///
    /// `destination` must be of the same pixel type and can be of a different buffer type.
    /// For example, if `self` is of type `Surface<'_, Vec<[u8; 4]>, [u8; 4]>>`:
    ///
    /// - `self` can set its position relative to a `Surface<'s, &'s mut [u8], [u8; 4]>>`
    /// - `self` *can't* set its position relative to a `Surface<'_, Vec<[u8; 3]>, [u8; 3]>>`
    ///
    /// This will also set the region of pixels of `self.buffer` that otherlap with `destination.buffer`.
    ///
    /// Returns an error if the clipped blitting area is not within `destination`.
    pub const fn set_position<B: AsRef<[P]> + AsMut<[P]>>(
        &mut self,
        position: PositionI,
        destination: &Surface<'_, B, P>,
    ) -> Result<RectU, Error> {
        let rect = RectI {
            position,
            size: self.size,
        };
        let destination_rect = RectI::from_size(destination.size);
        match rect.clip(destination_rect) {
            Some(rect) => {
                self.destination_rect = Some(rect);
                Ok(rect)
            }
            None => Err(Error::InvalidDestinationRect(rect, destination_rect)),
        }
    }

    /// Returns the top-left position of `self` relative to a destination surface,
    /// or None if [Self::set_position] hasn't been called.
    pub const fn get_position(&self) -> Option<PositionU> {
        match self.destination_rect {
            Some(rect) => Some(rect.position),
            None => None,
        }
    }

    /// Set an `area` within the pixel buffer to blit.
    ///
    /// You must call [Self::set_position] before calling this.
    ///
    /// If `area` is None, then the entirety of this surface will blit (this is the default behavior).
    ///
    /// If `area` is out of bounds of `self`'s underlying buffer, this will return an error.
    /// Otherwise, this returns a [RectU] describing the clipped area
    /// (i.e. which pixels within the area overlap with `dst`).
    ///
    /// ```
    /// use blittle::*;
    ///
    /// let mut src = Rgb8Surface::new(Size::new(512, 512));
    /// let dst = Rgb8Surface::new(Size::new(1920, 1080));
    /// // Set the position.
    /// src.set_position(PositionI::new(0, 0), &dst).unwrap();
    /// // Blit only the pixels in this region.
    /// src.set_area(Some(RectI { position: PositionI::new(1, 3), size: Size::new(60, 80)})).unwrap();
    /// // Reset the area. Now, all pixels will blit.
    /// src.set_area(None).unwrap();
    /// // Invalid area.
    /// assert!(src.set_area(Some(RectI { position: PositionI::new(-2000, -2000), size: Size::new(60, 80)})).is_err());
    /// ```
    pub const fn set_area(&mut self, area: Option<RectI>) -> Result<Option<RectU>, Error> {
        match self.destination_rect {
            Some(destination_rect) => match area {
                Some(area) => {
                    let rect = RectI::from_size(destination_rect.size);
                    match area.clip(rect) {
                        Some(area) => {
                            self.blit_area = Some(area);
                            Ok(self.blit_area)
                        }
                        None => Err(Error::InvalidArea(area)),
                    }
                }
                None => Ok(None),
            },
            None => Err(Error::AreaBeforePosition),
        }
    }

    /// Iterate through the pixel buffer per-row, top to bottom.
    pub fn rows(&self) -> impl Iterator<Item = &[P]> {
        self.buffer.as_ref().chunks_exact(self.size.width)
    }

    /// Iterate through the pixel buffer per-row, top to bottom.
    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [P]> {
        self.buffer.as_mut().chunks_exact_mut(self.size.width)
    }

    /// Returns the color of the pixel at `position`.
    ///
    /// Returns an error if `position` is out of bounds.
    ///
    /// ```
    /// use blittle::*;
    ///
    /// let src = Rgb8Surface::new(Size::new(64, 64));
    /// // Get the pixel at this position.
    /// let _ = src.get_pixel_checked(PositionU::new(3, 15));
    /// // This position is out of bounds.
    /// assert!(src.get_pixel_checked(PositionU::new(102, 15)).is_err());
    /// ```
    pub fn get_pixel_checked(&self, position: PositionU) -> Result<P, Error> {
        if position.x < self.size.width && position.y < self.size.height {
            Ok(self.get_pixel_unchecked(position))
        } else {
            Err(Error::PixelPosition {
                position,
                size: self.size,
            })
        }
    }

    /// Returns the color of the pixel at `position`.
    ///
    /// ```
    /// use blittle::*;
    ///
    /// let src = Rgb8Surface::new(Size::new(64, 64));
    /// let _ = src.get_pixel_unchecked(PositionU::new(3, 15));
    /// ```
    pub fn get_pixel_unchecked(&self, position: PositionU) -> P {
        let index = self.get_index(position.x, position.y);
        self.buffer.as_ref()[index]
    }

    /// Set the color of the pixel at `position`.
    ///
    /// Returns an error if `position` is out of bounds.
    ///
    /// ```
    /// use blittle::*;
    ///
    /// let mut src = Rgb8Surface::new(Size::new(64, 64));
    /// // Set the pixel at this position.
    /// src.set_pixel_checked(PositionU::new(3, 15), [50, 0, 220]).unwrap();
    /// // This pixel is out of bounds.
    /// assert!(src.set_pixel_checked(PositionU::new(120, 15), [50, 0, 220]).is_err());
    /// ```
    pub fn set_pixel_checked(&mut self, position: PositionU, color: P) -> Result<(), Error> {
        if position.x < self.size.width && position.y < self.size.height {
            self.set_pixel_unchecked(position, color);
            Ok(())
        } else {
            Err(Error::PixelPosition {
                position,
                size: self.size,
            })
        }
    }

    /// Set the color of the pixel at `position`.
    ///
    /// ```
    /// use blittle::*;
    ///
    /// let mut src = Rgb8Surface::new(Size::new(64, 64));
    /// src.set_pixel_unchecked(PositionU::new(3, 15), [50, 0, 220]);
    /// ```
    pub fn set_pixel_unchecked(&mut self, position: PositionU, color: P) {
        let index = self.get_index(position.x, position.y);
        self.buffer.as_mut()[index] = color;
    }

    /// Convert (x, y) coordinates into an index value within the underlying pixel buffer.
    pub const fn get_index(&self, x: usize, y: usize) -> usize {
        x + y * self.size.width
    }

    /// Unset all internal data related to blitting to the destination surface.
    ///
    /// This function can be a useful safeguard when blitting to multiple surfaces.
    pub const fn reset(&mut self) {
        self.blit_area = None;
        self.destination_rect = None;
    }

    /// Returns the size of the surface.
    pub const fn get_size(&self) -> Size {
        self.size
    }

    /// Returns the underlying pixel buffer.
    pub fn buffer(&self) -> &[P] {
        self.buffer.as_ref()
    }

    /// Returns the underlying mutable pixel buffer.
    pub fn buffer_mut(&mut self) -> &mut [P] {
        self.buffer.as_mut()
    }

    /// Returns the underlying mutable buffer as bytes.
    pub fn bytes(&self) -> &[u8] {
        cast_slice::<P, u8>(self.buffer())
    }

    /// Returns the underlying mutable buffer as bytes.
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        cast_slice_mut::<P, u8>(self.buffer_mut())
    }

    pub(crate) fn get_blit_params(&self, destination_size: Size) -> Result<(RectU, RectU), Error> {
        match self.destination_rect {
            Some(destination_rect) => {
                if destination_rect.overlaps(&RectU::from_size(destination_size)) {
                    // Blit either a chunk of the source buffer, or all of it.
                    let blit_area = match self.blit_area {
                        Some(rect) => rect,
                        None => RectU {
                            position: PositionU::ZERO,
                            size: destination_rect.size,
                        },
                    };
                    Ok((destination_rect, blit_area))
                } else {
                    Err(Error::NoOverlap)
                }
            }
            None => Err(Error::NoDestinationRect),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "png")]
    use crate::png::Png;
    #[cfg(feature = "png")]
    use std::env::current_dir;
    #[cfg(feature = "png")]
    use std::io::Cursor;

    const SRC_W: usize = 32;
    const SRC_H: usize = 17;
    const DST_W: usize = 64;
    const DST_H: usize = 64;

    #[cfg(feature = "png")]
    #[test]
    fn test_blit() {
        let position = PositionI { x: 2, y: 12 };
        let src_size = Size {
            width: SRC_W,
            height: SRC_H,
        };
        let mut dst = Surface::new_filled(
            Size {
                width: DST_W,
                height: DST_H,
            },
            [0u8, 0, 0],
        );

        let mut src = Surface::new_filled(src_size, [255u8, 255, 255]);
        src.set_position(position, &dst).unwrap();

        let rect = src.destination_rect.unwrap();
        assert_eq!(rect.position, PositionU { x: 2, y: 12 });
        assert_eq!(rect.size, src_size);

        src.blit(&mut dst).unwrap();

        Rgb8Surface::write_png(
            &dst,
            current_dir().unwrap().join("test_output").join("blit.png"),
        )
        .unwrap();
    }

    #[cfg(feature = "png")]
    #[test]
    fn test_clip() {
        blit_clipped("clip_positive.png", DST_W as isize - 12, 16);
        blit_clipped("clip_negative.png", -8, -8);
    }

    #[test]
    fn test_area() {
        let position = PositionI { x: 2, y: 12 };
        let src_size = Size::new(SRC_W, SRC_H);
        let dst = Surface::new_filled(Size::new(DST_W, DST_H), [0u8, 0, 0]);

        let mut src = Surface::new_filled(src_size, [255u8, 255, 255]);
        src.set_position(position, &dst).unwrap();
        let size = Size::new(5, 5);
        let area = src
            .set_area(Some(RectI {
                position: PositionI::ZERO,
                size,
            }))
            .unwrap()
            .unwrap();
        assert_eq!(area.position, PositionU::ZERO);
        assert_eq!(area.size, size);

        // Clipped size.
        let size = Size::new(70, 80);
        let area = src
            .set_area(Some(RectI {
                position: PositionI::ZERO,
                size,
            }))
            .unwrap()
            .unwrap();
        assert_eq!(area.position, PositionU::ZERO);
        assert_eq!(area.size, src.size);

        // Clipped position.
        let position = PositionI::new(6, 8);
        let area = src
            .set_area(Some(RectI { position, size }))
            .unwrap()
            .unwrap();
        assert_eq!(
            area.position,
            PositionU {
                x: position.x.cast_unsigned(),
                y: position.y.cast_unsigned(),
            }
        );
        assert_eq!(
            area.size,
            Size {
                width: SRC_W - position.x.cast_unsigned(),
                height: SRC_H - position.y.cast_unsigned(),
            }
        );

        // Negative position.
        let position = PositionI::new(-5, -5);
        let area = src
            .set_area(Some(RectI { position, size }))
            .unwrap()
            .unwrap();
        assert_eq!(area.position, PositionU::ZERO);
        assert_eq!(area.size, src.size);

        // Out of bounds.
        let position = PositionI::new(-50, -5);
        assert!(src.set_area(Some(RectI { position, size })).is_err());
    }

    #[cfg(feature = "png")]
    fn blit_clipped(name: &str, x: isize, y: isize) {
        let src_size = Size::new(SRC_W, SRC_H);
        let mut dst = Surface::new_filled(Size::new(DST_W, DST_H), [0u8, 0, 0]);

        let mut src = Surface::new_filled(src_size, [0u8, 255, 255]);
        src.set_position(PositionI { x, y }, &dst).unwrap();

        src.blit(&mut dst).unwrap();

        Rgb8Surface::write_png(&dst, current_dir().unwrap().join("test_output").join(name))
            .unwrap();
    }

    #[cfg(feature = "png")]
    #[test]
    fn test_src_area() {
        const D: usize = 128;
        const SIZE: Size = Size::new(D, D);

        let mut dst = Rgb8Surface::new_filled(SIZE, [255, 255, 255]);
        let mut src =
            Rgb8Surface::read_png(Cursor::new(include_bytes!("../test_images/text.png"))).unwrap();
        src.set_position(PositionI::new(12, 13), &dst).unwrap();
        src.set_area(Some(RectI {
            position: PositionI::new(20, 3),
            size: Size::new(50, 70),
        }))
        .unwrap();
        src.blit(&mut dst).unwrap();
        Rgb8Surface::write_png(
            &dst,
            current_dir()
                .unwrap()
                .join("test_output")
                .join("clipped_text.png"),
        )
        .unwrap();
    }
}
