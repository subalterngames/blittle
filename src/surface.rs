use crate::error::Error;
use crate::rect::{RectI, RectU};
use bytemuck::{cast_slice, cast_slice_mut};
use glam::{I64Vec2, USizeVec2, Vec4};

/// Grayscale.
pub type L8Surface = Surface<u8>;
/// Grayscale + alpha.
pub type La8Surface = Surface<[u8; 2]>;
/// Red, green, blue.
pub type Rgb8Surface = Surface<[u8; 3]>;
/// Red, green, blue, alpha. Or, if using [softbuffer](https://docs.rs/softbuffer/latest/softbuffer/): zero, red, green, blue.
pub type Rgba8Surface = Surface<[u8; 4]>;
/// Zero, red, green, blue. Meant to be used with [softbuffer](https://docs.rs/softbuffer/latest/softbuffer/).
pub type Zrgb8Surface = Surface<u32>;
/// 32-bit grayscale.
pub type L32Surface = Surface<f32>;
/// 32-bit grayscale + alpha.
pub type La32Surface = Surface<[f32; 2]>;
/// Red, green, blue, alpha as Vec4s.
/// This uses glam for that sweet sweet SIMD, so you can't get the underlying bytes buffer.
pub type Rgba32Surface = Surface<Vec4>;

macro_rules! impl_surface {
    ($self:ident) => {
        /// Fill the surface with `color`.
        pub fn fill(&mut $self, color: P) {
            $self.buffer.fill(color);
        }

        /// Set the top-left position of the surface, relative to the surface it will blit to `destination`.
        ///
        /// This also sets the clipped blitting area within `destination`.
        ///
        /// Returns an error if the clipped blitting area is not within `destination`.
        pub fn position(mut $self, position: I64Vec2, destination: &Self) -> Result<Self, Error> {
            $self.set_position(position, destination).map(|_| $self)
        }

        /// Set the top-left position of the surface, relative to the surface it will blit to `destination`.
        ///
        /// This also sets the clipped blitting area within `destination`, which is the return value.
        ///
        /// Returns an error if the clipped blitting area is not within `destination`.
        pub const fn set_position(
            &mut $self,
            position: I64Vec2,
            destination: &Self,
        ) -> Result<RectU, Error> {
            let rect = RectI {
                position,
                size: $self.size
            };
            let destination_rect = RectI::from_size(destination.size);
            match rect.clip(destination_rect) {
                Some(rect) => {
                    $self.destination_rect = Some(rect);
                    Ok(rect)
                }
                None => Err(Error::InvalidDestinationRect(rect, destination_rect)),
            }
        }

        /// Returns the size of the surface.
        pub const fn get_size(&$self) -> USizeVec2 {
            $self.size
        }

        /// Set an `area` within the pixel buffer to blit.
        ///
        /// If `area` is None, then the entirety of this surface will blit (this is the default behavior).
        pub fn area(mut $self, area: Option<RectI>) -> Result<Self, Error> {
            $self.set_area(area).map(|_| $self)
        }

        /// Set an `area` within the pixel buffer to blit.
        ///
        /// If `area` is None, then the entirety of this surface will blit (this is the default behavior).
        pub const fn set_area(&mut $self, area: Option<RectI>) -> Result<Option<RectU>, Error> {
            match $self.destination_rect {
                Some(destination_rect) => {
                    match area {
                        Some(area) => {
                            let rect = RectI::from_size(destination_rect.size);
                            match area.clip(rect) {
                                Some(area) => {
                                    $self.blit_area = Some(area);
                                    Ok($self.blit_area)
                                }
                                None => Err(Error::InvalidArea(area)),
                            }
                        }
                        None => Ok(None),
                    }
                }
                None => Err(Error::AreaBeforePosition)
            }
        }

        /// The underlying pixel buffer.
        pub fn buffer(&$self) -> &[P] {
            &$self.buffer
        }

        pub fn buffer_mut(&mut $self) -> &mut [P] {
            &mut $self.buffer
        }

        /// Iterate through the pixel buffer per-row, top to bottom.
        pub fn rows(&$self) -> impl Iterator<Item = &[P]> {
            $self.buffer.chunks_exact($self.size.x)
        }

        /// Iterate through the pixel buffer per-row, top to bottom.
        pub fn rows_mut(&mut $self) -> impl Iterator<Item = &mut [P]> {
            $self.buffer.chunks_exact_mut($self.size.x)
        }

        /// Returns the color of the pixel at (x, y).
        ///
        /// Returns an error if (x, y) is out of bounds.
        pub fn get_pixel_checked(&$self, x: usize, y: usize) -> Result<P, Error> {
            if x < $self.size.x && y < $self.size.y {
                Ok($self.get_pixel_unchecked(x, y))
            } else {
                Err(Error::PixelPosition {
                    x,
                    y,
                    size: $self.size,
                })
            }
        }

        /// Returns the color of the pixel at (x, y).
        pub fn get_pixel_unchecked(&$self, x: usize, y: usize) -> P {
            let index = $self.get_index(x, y);
            $self.buffer[index]
        }

        /// Set the color of the pixel at (x, y).
        ///
        /// Returns an error if (x, y) is out of bounds.
        pub fn set_pixel_checked(&mut $self, x: usize, y: usize, color: P) -> Result<(), Error> {
            if x < $self.size.x && y < $self.size.y {
                $self.set_pixel_unchecked(x, y, color);
                Ok(())
            } else {
                Err(Error::PixelPosition {
                    x,
                    y,
                    size: $self.size,
                })
            }
        }

        /// Set the color of the pixel at (x, y).
        pub fn set_pixel_unchecked(&mut $self, x: usize, y: usize, color: P) {
            let index = $self.get_index(x, y);
            $self.buffer[index] = color;
        }

        /// Convert (x, y) coordinates into an index value within the underlying pixel buffer.
        pub const fn get_index(&$self, x: usize, y: usize) -> usize {
            x + y * $self.size.x
        }

        /// Unset all internal data related to blitting to the destination surface.
        ///
        /// This function can be a useful safeguard when blitting to multiple surfaces.
        pub const fn reset(&mut $self) {
            $self.blit_area = None;
            $self.destination_rect = None;
        }
    };
}

// Expose bytes for a kind of surface.
macro_rules! impl_bytes_inner {
    ($s:tt, $p:ty $(,$lt:lifetime)?) => {
        impl $s<$($lt,)? $p> {
            /// The underlying buffer as bytes.
            pub fn bytes(&self) -> &[u8] {
                cast_slice::<$p, u8>(&self.buffer)
            }

            /// The underlying mutable buffer as bytes.
            pub fn bytes_mut(&mut self) -> &mut [u8] {
                cast_slice_mut::<$p, u8>(&mut self.buffer)
            }
        }
    };
}

// Expose bytes for Surface and SurfaceRef.
macro_rules! impl_bytes {
    ($p:ty) => {
        impl_bytes_inner!(Surface, $p);

        impl_bytes_inner!(SurfaceRef, $p, '_);
    };
}

pub struct Surface<P: Copy + Clone + Sized + Default> {
    pub(crate) size: USizeVec2,
    pub(crate) buffer: Vec<P>,
    pub(crate) destination_rect: Option<RectU>,
    pub(crate) blit_area: Option<RectU>,
}

impl<P: Copy + Clone + Sized + Default> Surface<P> {
    /// Get a new surface.
    ///
    /// The position defaults to `(0, 0)`.
    /// The underlying pixel buffer is set to the pixel's default value (i.e. `[0, 0, 0]`), length `size.x * size.y`.
    /// The destination rect and blit area both default to None.
    pub fn new(size: USizeVec2) -> Self {
        Self {
            size,
            buffer: vec![P::default(); size.x * size.y],
            destination_rect: None,
            blit_area: None,
        }
    }

    /// Get a new surface with color `color`.
    ///
    /// The position defaults to `(0, 0)`.
    /// The underlying pixel buffer is set to `vec![color; size.x * size.y]`.
    /// The destination rect and blit area both default to None.
    pub fn new_filled(size: USizeVec2, color: P) -> Self {
        Self {
            size,
            buffer: vec![color; size.x * size.y],
            destination_rect: None,
            blit_area: None,
        }
    }

    /// Blit onto `other`.
    ///
    /// Be sure to call [Self::position] or [Self::set_position]
    /// before blitting to a *new* `other` surface.
    pub fn blit(&self, other: &mut Self) -> Result<(), Error> {
        // Try to get the destination rect.
        let destination_rect = self.destination_rect.ok_or(Error::NoDestinationRect)?;

        // Blit either a chunk of the source buffer, or all of it.
        let blit_area = match self.blit_area {
            Some(rect) => rect,
            None => RectU {
                position: USizeVec2::ZERO,
                size: destination_rect.size,
            },
        };

        // Iterate per-row.
        (0..blit_area.size.y).for_each(|src_y| {
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
            other.buffer[dst_index..dst_index + blit_area.size.x]
                .copy_from_slice(&self.buffer[src_index..src_index + blit_area.size.x]);
        });
        Ok(())
    }

    impl_surface!(self);
}

impl L8Surface {
    /// The underlying buffer as bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.buffer
    }

    /// The underlying mutable buffer as bytes.
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }
}

impl_bytes!([u8; 2]);
impl_bytes!([u8; 3]);
impl_bytes!([u8; 4]);
impl_bytes!(f32);
impl_bytes!([f32; 2]);
impl_bytes!(u32);

/// A surface with a mutable reference to a pixel buffer.
///
/// This is used to view ZRGB surfaces as RGBA surfaces and vice-versa.
pub struct SurfaceRef<'s, P: Copy + Clone + Sized + Default> {
    pub(crate) size: USizeVec2,
    pub(crate) buffer: &'s mut [P],
    pub(crate) destination_rect: Option<RectU>,
    pub(crate) blit_area: Option<RectU>,
}

impl<'s, P: Copy + Clone + Sized + Default> SurfaceRef<'s, P> {
    impl_surface!(self);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "png")]
    use png::ColorType;
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
        let position = I64Vec2 { x: 2, y: 12 };
        let src_size = USizeVec2 { x: SRC_W, y: SRC_H };
        let mut dst = Surface::new_filled(USizeVec2 { x: DST_W, y: DST_H }, [0u8, 0, 0]);

        let src = Surface::new_filled(src_size, [255u8, 255, 255])
            .position(position, &dst)
            .unwrap();

        let rect = src.destination_rect.unwrap();
        assert_eq!(rect.position, USizeVec2 { x: 2, y: 12 });
        assert_eq!(rect.size, src_size);

        src.blit(&mut dst).unwrap();

        dst.write_png(current_dir().unwrap().join("test_output/blit.png"))
            .unwrap();
    }

    #[cfg(feature = "png")]
    #[test]
    fn test_clip() {
        blit_clipped("clip_positive.png", DST_W as i64 - 12, 16);
        blit_clipped("clip_negative.png", -8, -8);
    }

    #[test]
    fn test_area() {
        let position = I64Vec2 { x: 2, y: 12 };
        let src_size = USizeVec2 { x: SRC_W, y: SRC_H };
        let dst = Surface::new_filled(USizeVec2 { x: DST_W, y: DST_H }, [0u8, 0, 0]);

        let mut src = Surface::new_filled(src_size, [255u8, 255, 255])
            .position(position, &dst)
            .unwrap();
        let size = USizeVec2::new(5, 5);
        let area = src
            .set_area(Some(RectI {
                position: I64Vec2::ZERO,
                size,
            }))
            .unwrap()
            .unwrap();
        assert_eq!(area.position, USizeVec2::ZERO);
        assert_eq!(area.size, size);

        // Clipped size.
        let size = USizeVec2::new(70, 80);
        let area = src
            .set_area(Some(RectI {
                position: I64Vec2::ZERO,
                size,
            }))
            .unwrap()
            .unwrap();
        assert_eq!(area.position, USizeVec2::ZERO);
        assert_eq!(area.size, src.size);

        // Clipped position.
        let position = I64Vec2::new(6, 8);
        let area = src
            .set_area(Some(RectI { position, size }))
            .unwrap()
            .unwrap();
        assert_eq!(
            area.position,
            USizeVec2 {
                x: position.x.cast_unsigned() as usize,
                y: position.y.cast_unsigned() as usize,
            }
        );
        assert_eq!(
            area.size,
            USizeVec2 {
                x: SRC_W - position.x.cast_unsigned() as usize,
                y: SRC_H - position.y.cast_unsigned() as usize,
            }
        );

        // Negative position.
        let position = I64Vec2::new(-5, -5);
        let area = src
            .set_area(Some(RectI { position, size }))
            .unwrap()
            .unwrap();
        assert_eq!(area.position, USizeVec2::ZERO);
        assert_eq!(area.size, src.size);

        // Out of bounds.
        let position = I64Vec2::new(-50, -5);
        assert!(src.set_area(Some(RectI { position, size })).is_err());
    }

    #[cfg(feature = "png")]
    fn blit_clipped(name: &str, x: i64, y: i64) {
        let src_size = USizeVec2 { x: SRC_W, y: SRC_H };
        let mut dst = Surface::new_filled(USizeVec2 { x: DST_W, y: DST_H }, [0u8, 0, 0]);

        let src = Surface::new_filled(src_size, [0u8, 255, 255])
            .position(I64Vec2 { x, y }, &dst)
            .unwrap();

        src.blit(&mut dst).unwrap();

        dst.write_png(current_dir().unwrap().join("test_output").join(name))
            .unwrap();
    }

    #[cfg(feature = "png")]
    #[test]
    fn test_src_area() {
        const D: usize = 128;
        const SIZE: USizeVec2 = USizeVec2::new(D, D);

        let mut dst = Rgb8Surface::new_filled(SIZE, [255, 255, 255]);
        let src = read_png(include_bytes!("../test_images/text.png"))
            .position(I64Vec2::new(12, 13), &dst)
            .unwrap()
            .area(Some(RectI {
                position: I64Vec2::new(20, 3),
                size: USizeVec2::new(50, 70),
            }))
            .unwrap();
        src.blit(&mut dst).unwrap();
        dst.write_png(current_dir().unwrap().join("test_output/clipped_text.png"))
            .unwrap();
    }

    #[cfg(feature = "png")]
    fn read_png(bytes: &[u8]) -> Rgb8Surface {
        let decoder = png::Decoder::new(Cursor::new(bytes));
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size().unwrap()];
        let info = reader.next_frame(&mut buf).unwrap();
        assert_eq!(info.color_type, ColorType::Rgb);
        Rgb8Surface {
            size: USizeVec2::new(info.width as usize, info.height as usize),
            buffer: cast_slice::<u8, [u8; 3]>(&buf[..info.buffer_size()]).to_vec(),
            destination_rect: None,
            blit_area: None,
        }
    }
}
