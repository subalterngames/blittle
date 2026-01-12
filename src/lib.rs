#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "rayon")]
mod multi_threaded;
#[cfg(feature = "overlay")]
pub mod overlay;
mod pixel_type;
mod position;
mod rect;
mod size;

#[cfg(feature = "rayon")]
pub use multi_threaded::*;

pub use pixel_type::PixelType;
pub use position::*;
pub use rect::ClippedRect;
pub use size::Size;

/// Blit `src` onto `dst`.
///
/// - `src` and `dst` are flat byte slices of images. There are many ways to cast your pixel map to `[u8]`, such as with the `bytemuck` crate.
/// - `rect` is the [`ClippedRect`] defining the blit area.
/// - `pixel_type` is the [`PixelType`] for both `src` and `dst`.
pub fn blit(src: &[u8], dst: &mut [u8], rect: &ClippedRect, pixel_type: &PixelType) {
    let stride = pixel_type.stride();
    let src_w = rect.src_size_clipped.w * stride;
    let src_position = rect.get_src_position();
    (0..rect.src_size_clipped.h).for_each(|src_y| {
        let src_index = get_index(
            src_position.x,
            src_y + src_position.y,
            rect.src_size.w,
            stride,
        );
        let dst_index = get_index(
            rect.dst_position_clipped.x,
            rect.dst_position_clipped.y + src_y,
            rect.dst_size.w,
            stride,
        );
        dst[dst_index..dst_index + src_w].copy_from_slice(&src[src_index..src_index + src_w]);
    });
}

/// Converts a position, width, and stride to an index in a 1D byte slice.
pub const fn get_index(x: usize, y: usize, w: usize, stride: usize) -> usize {
    (x + y * w) * stride
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

    const PIXEL_TYPE: PixelType = PixelType::Rgb8;
    const RGB: usize = PIXEL_TYPE.stride();

    #[test]
    fn test_blit() {
        let src = [255u8; SRC_W * SRC_H * RGB];
        let mut dst = [0u8; DST_W * DST_H * RGB];

        let dst_position = PositionI { x: 2, y: 12 };
        let dst_size = Size { w: DST_W, h: DST_H };
        let src_size = Size { w: SRC_W, h: SRC_H };
        let rect = ClippedRect::new(dst_position, dst_size, src_size).unwrap();

        blit(&src, &mut dst, &rect, &PIXEL_TYPE);

        write_png("blit.png", &dst, DST_W as u32, DST_H as u32);
    }

    #[test]
    fn test_clip() {
        blit_clipped("clip_positive.png", 42, 16);
        blit_clipped("clip_negative.png", -8, -8);
    }

    #[test]
    fn test_clipped_text() {
        let src = read_png(include_bytes!("../test_images/text.png"));
        let d = 512;
        let s = 128;
        let mut dst = vec![0u8; d * d * RGB];

        let dst_position = PositionI {
            x: -s / 4,
            y: s + s / 2,
        };
        let dst_size = Size { w: d, h: d };
        let src_size = Size {
            w: s as usize,
            h: s as usize,
        };
        let rect = ClippedRect::new(dst_position, dst_size, src_size).unwrap();

        blit(&src, &mut dst, &rect, &PIXEL_TYPE);
        write_png("text.png", &dst, d as u32, d as u32);
    }

    #[test]
    fn test_src_area() {
        let src_size = Size { w: 64, h: 64 };
        let pixel_type = PixelType::Rgb8;

        let src = read_png(include_bytes!("../test_images/checkerboard.png"));
        assert_eq!(src.len(), src_size.w * src_size.h * pixel_type.stride());

        let dst_size = Size { w: 128, h: 128 };
        let mut dst = vec![255; dst_size.w * dst_size.h * pixel_type.stride()];
        let mut rect = ClippedRect::new(PositionI { x: 12, y: 13 }, dst_size, src_size).unwrap();
        rect.set_src_rect(PositionU { x: 20, y: 3 }, Size { w: 32, h: 32 });
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
