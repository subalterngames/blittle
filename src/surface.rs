use crate::error::Error;
use crate::rect::{RectI, RectU};
use bytemuck::{cast_slice, cast_slice_mut};
use glam::{I64Vec2, USizeVec2, Vec4};

pub type L8Surface = Surface<u8>;
pub type La8Surface = Surface<[u8; 2]>;
pub type Rgb8Surface = Surface<[u8; 3]>;
pub type Rgba8Surfaace = Surface<[u8; 4]>;
pub type L32Surface = Surface<f32>;
pub type Rgba32Surface = Surface<Vec4>;

pub struct Surface<P: Copy + Clone + Sized + Default> {
    rect: RectI,
    buffer: Vec<P>,
    destination_rect: Option<RectU>,
    blit_area: Option<RectU>,
}

impl<P: Copy + Clone + Sized + Default> Surface<P> {
    pub fn new(size: USizeVec2) -> Self {
        Self {
            rect: RectI::from_size(size),
            buffer: vec![P::default(); size.x * size.y],
            destination_rect: None,
            blit_area: None,
        }
    }

    pub fn new_filled(size: USizeVec2, color: P) -> Self {
        Self {
            rect: RectI::from_size(size),
            buffer: vec![color; size.x * size.y],
            destination_rect: None,
            blit_area: None,
        }
    }

    pub fn filled(mut self, color: P) -> Self {
        self.fill(color);
        self
    }

    pub fn fill(&mut self, color: P) {
        self.buffer.fill(color);
    }

    pub const fn position(mut self, position: I64Vec2) -> Self {
        self.set_position(position);
        self
    }

    pub const fn set_position(&mut self, position: I64Vec2) {
        self.rect.position = position;
    }

    pub const fn get_rect(&self) -> RectI {
        self.rect
    }

    pub const fn set_destination(&mut self, destination: &Self) -> Result<RectU, Error> {
        match self.rect.clip(destination.rect) {
            Some(rect) => {
                self.destination_rect = Some(rect);
                Ok(rect)
            }
            None => Err(Error::InvalidDestinationRect(self.rect, destination.rect)),
        }
    }

    pub const fn set_area(&mut self, area: RectI) -> Result<RectU, Error> {
        match area.clip(self.rect) {
            Some(area) => {
                self.blit_area = Some(area);
                Ok(area)
            }
            None => Err(Error::InvalidArea(area)),
        }
    }

    pub const fn get_index(x: usize, y: usize, w: usize) -> usize {
        x + y * w
    }

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
            let src_index = Self::get_index(
                blit_area.position.x,         // Blit area offset or zer
                src_y + blit_area.position.y, // y offset + blit area offset
                self.rect.size.x,             // The actual width of the source buffer
            );
            let dst_index = Self::get_index(
                destination_rect.position.x,         // Destination position (x)
                src_y + destination_rect.position.y, // y offset + destination position (y)
                other.rect.size.x,                   // The actual width of the destination buffer
            );
            // Copy the slice, using the blit area's width.
            other.buffer[dst_index..dst_index + blit_area.size.x]
                .copy_from_slice(&self.buffer[src_index..src_index + blit_area.size.x]);
        });
        Ok(())
    }
}

pub trait SurfaceBytes {
    fn bytes(&self) -> &[u8];

    fn bytes_mut(&mut self) -> &mut [u8];
}

impl SurfaceBytes for L8Surface {
    fn bytes(&self) -> &[u8] {
        &self.buffer
    }

    fn bytes_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }
}

impl SurfaceBytes for L32Surface {
    fn bytes(&self) -> &[u8] {
        cast_slice::<f32, u8>(&self.buffer)
    }

    fn bytes_mut(&mut self) -> &mut [u8] {
        cast_slice_mut::<f32, u8>(&mut self.buffer)
    }
}

macro_rules! impl_surface_bytes {
    ($c:literal) => {
        impl SurfaceBytes for Surface<[u8; $c]> {
            fn bytes(&self) -> &[u8] {
                cast_slice::<[u8; $c], u8>(&self.buffer)
            }

            fn bytes_mut(&mut self) -> &mut [u8] {
                cast_slice_mut::<[u8; $c], u8>(&mut self.buffer)
            }
        }
    };
}

impl_surface_bytes!(2);
impl_surface_bytes!(3);
impl_surface_bytes!(4);

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
