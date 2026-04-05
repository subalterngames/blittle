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
            $self.rect.position = position;
            match $self.rect.clip(destination.rect) {
                Some(rect) => {
                    $self.destination_rect = Some(rect);
                    Ok(rect)
                }
                None => Err(Error::InvalidDestinationRect($self.rect, destination.rect)),
            }
        }

        /// Returns the top-left position and size of the surface.
        pub const fn get_rect(&$self) -> RectI {
            $self.rect
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
            match area {
                Some(area) => match area.clip($self.rect) {
                    Some(area) => {
                        $self.blit_area = Some(area);
                        Ok($self.blit_area)
                    }
                    None => Err(Error::InvalidArea(area)),
                },
                None => Ok(None),
            }
        }

         /// The underlying pixel buffer.
    pub fn buffer(&$self) -> &[P] {
        &$self.buffer
    }

    /// Iterate through the pixel buffer per-row, top to bottom.
    pub fn rows(&$self) -> impl Iterator<Item = &[P]> {
        $self.buffer.chunks_exact($self.rect.size.x)
    }

    /// Iterate through the pixel buffer per-row, top to bottom.
    pub fn rows_mut(&mut $self) -> impl Iterator<Item = &mut [P]> {
        $self.buffer.chunks_exact_mut($self.rect.size.x)
    }

    /// Returns the color of the pixel at (x, y).
    ///
    /// Returns an error if (x, y) is out of bounds.
    pub fn get_pixel_checked(&$self, x: usize, y: usize) -> Result<P, Error> {
        if x < $self.rect.size.x && y < $self.rect.size.y {
            Ok($self.get_pixel_unchecked(x, y))
        } else {
            Err(Error::PixelPosition {
                x,
                y,
                size: $self.rect.size,
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
        if x < $self.rect.size.x && y < $self.rect.size.y {
            $self.set_pixel_unchecked(x, y, color);
            Ok(())
        } else {
            Err(Error::PixelPosition {
                x,
                y,
                size: $self.rect.size,
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
        x + y * $self.rect.size.x
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
    rect: RectI,
    buffer: Vec<P>,
    destination_rect: Option<RectU>,
    blit_area: Option<RectU>,
}

impl<P: Copy + Clone + Sized + Default> Surface<P> {
    /// Get a new surface.
    ///
    /// The position defaults to `(0, 0).
    /// The underlying pixel buffer is set to the pixel's default value (i.e. `[0, 0, 0]`), length `size.x * size.y`.
    /// The destination rect and blit area both default to None.
    pub fn new(size: USizeVec2) -> Self {
        Self {
            rect: RectI::from_size(size),
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
            rect: RectI::from_size(size),
            buffer: vec![color; size.x * size.y],
            destination_rect: None,
            blit_area: None,
        }
    }

    /// Blit onto `other`.
    ///
    /// Be sure to call [`Self::set_destination`] before blitting to a *new* `other`,
    /// or after setting a new position via [`Self::position`] or [`Self::set_position`].
    pub fn blit(&self, other: &mut Self) -> Result<(), Error> {
        // Try to get the destination rect.
        let destination_rect = self.destination_rect.ok_or(Error::NoDestinationRect)?;

        // Blit either a chunk of the source buffer, or all of it.
        let blit_area = match self.blit_area {
            Some(rect) => rect,
            None => RectU {
                position: USizeVec2::ZERO,
                size: self.rect.size,
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

impl Zrgb8Surface {
    /// View this ZRGB surface as a mutable RGBA surface reference.
    /// This is useful for setting per-pixel color values.
    ///
    /// NOTE: that the returned value does *not* contain RGBA values!
    /// This is because ZRGB is meant to be used with [softbuffer](https://docs.rs/softbuffer/latest/softbuffer/).
    /// The pixel layout is: `[z, r, g, b]` where `z` is always 0.
    ///
    /// NOTE: Setting the position, area, etc. will NOT modify the original surface.
    pub fn as_rgba_ref(&mut self) -> SurfaceRef<'_, [u8; 4]> {
        SurfaceRef {
            rect: self.rect,
            buffer: cast_slice_mut::<u32, [u8; 4]>(&mut self.buffer),
            destination_rect: self.destination_rect,
            blit_area: self.blit_area,
        }
    }
}

impl Rgba8Surface {
    /// View this surface as a mutable ZRGB surface reference.
    /// Useful for integration with [softbuffer](https://docs.rs/softbuffer/latest/softbuffer/).
    ///
    /// NOTE: Setting the position, area, etc. will NOT modify the original surface.
    pub fn as_zrgb_ref(&mut self) -> SurfaceRef<'_, u32> {
        SurfaceRef {
            rect: self.rect,
            buffer: cast_slice_mut::<[u8; 4], u32>(&mut self.buffer),
            destination_rect: self.destination_rect,
            blit_area: self.blit_area,
        }
    }
}

/// A surface with a mutable reference to a pixel buffer.
pub struct SurfaceRef<'s, P: Copy + Clone + Sized + Default> {
    rect: RectI,
    buffer: &'s mut [P],
    destination_rect: Option<RectU>,
    blit_area: Option<RectU>,
}

impl<'s, P: Copy + Clone + Sized + Default> SurfaceRef<'s, P> {
    impl_surface!(self);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::{fs::File, io::BufWriter, path::Path};

    const SRC_W: usize = 32;
    const SRC_H: usize = 17;
    const DST_W: usize = 64;
    const DST_H: usize = 64;

    #[test]
    fn test_blit() {
        let position = I64Vec2 { x: 2, y: 12 };
        let src_size = USizeVec2 { x: SRC_W, y: SRC_H };
        let mut src = Surface::new_filled(src_size, [255u8, 255, 255]).position(position);
        let mut dst = Surface::new_filled(USizeVec2 { x: DST_W, y: DST_H }, [0u8, 255, 255]);

        let rect = src.set_destination(&dst).unwrap();
        assert_eq!(rect.position, USizeVec2 { x: 2, y: 12 });
        assert_eq!(rect.size, src_size);

        src.blit(&mut dst).unwrap();

        write_png(
            "blit.png",
            cast_slice::<[u8; 3], u8>(&dst.buffer),
            DST_W as u32,
            DST_H as u32,
        );
    }

    #[test]
    fn test_clip() {
        blit_clipped("clip_positive.png", 42, 16);
        blit_clipped("clip_negative.png", -8, -8);
    }

    #[test]
    fn test_src_area() {
        let src_size = Size { w: 128, h: 128 };
        let pixel_type = PixelType::Rgb8;

        let src = read_png(include_bytes!("../test_images/text.png"));
        assert_eq!(src.len(), src_size.w * src_size.h * pixel_type.stride());

        let dst_size = Size { w: 128, h: 128 };
        let mut dst = vec![255; dst_size.w * dst_size.h * pixel_type.stride()];
        let mut rect = ClippedRect::new(PositionI { x: 12, y: 13 }, dst_size, src_size).unwrap();
        rect.set_src_rect(PositionU { x: 20, y: 3 }, Size { w: 50, h: 70 });
        blit(&src, &mut dst, &rect, &pixel_type);
        write_png("blit_area.png", &dst, dst_size.w as u32, dst_size.h as u32);
    }

    fn blit_clipped(name: &str, x: isize, y: isize) {
        let src = [255u8; SRC_W * SRC_H * RGB];
        let mut dst = [0u8; DST_W * DST_H * RGB];

        let dst_position = PositionI { x, y };
        let dst_size = Size { w: DST_W, h: DST_H };
        let src_size = Size { w: SRC_W, h: SRC_H };
        let rect = ClippedRect::new(dst_position, dst_size, src_size).unwrap();

        blit(&src, &mut dst, &rect, &PIXEL_TYPE);
        write_png(name, &dst, DST_W as u32, DST_H as u32);
    }

    fn read_png(bytes: &[u8]) -> Vec<u8> {
        let decoder = png::Decoder::new(Cursor::new(bytes));
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size().unwrap()];
        let info = reader.next_frame(&mut buf).unwrap();
        buf[..info.buffer_size()].to_vec()
    }

    fn write_png(path: &str, dst: &[u8], dst_w: u32, dst_h: u32) {
        let path = Path::new(path);
        let file = File::create(path).unwrap();
        let w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, dst_w, dst_h);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(dst).unwrap();
    }
}
