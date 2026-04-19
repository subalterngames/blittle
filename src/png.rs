use crate::Surface;
use crate::error::Error;
use bytemuck::{Pod, Zeroable, cast_slice};
use glam::USizeVec2;
use png::*;
use std::fs::File;
use std::io::{BufRead, BufWriter, Seek};
use std::marker::PhantomData;
use std::path::Path;

macro_rules! impl_png {
    ($p:ty, $color_type:ident) => {
        impl<S: AsRef<[$p]> + AsMut<[$p]>> Png<S, $p> for Surface<'_, S, $p> {
            fn get_png_color_type() -> ColorType {
                ColorType::$color_type
            }
        }
    };
}

pub trait Png<S: AsRef<[P]> + AsMut<[P]>, P: Copy + Clone + Sized + Default + Zeroable + Pod> {
    fn get_png_color_type() -> ColorType;

    fn write_png<Pa: AsRef<Path>>(surface: &Surface<'_, S, P>, path: Pa) -> Result<(), Error> {
        let file = File::create(path.as_ref())
            .map_err(|e| Error::PngFile(path.as_ref().to_path_buf(), e))?;
        let w = BufWriter::new(file);
        let size = surface.get_size();
        let mut encoder = Encoder::new(w, size.x as u32, size.y as u32);
        encoder.set_color(Self::get_png_color_type());
        encoder.set_depth(BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(Error::PngHeader)?;
        writer
            .write_image_data(surface.bytes())
            .map_err(Error::PngPixels)
    }

    fn read_png<'s, B: BufRead + Seek>(png: B) -> Result<Surface<'s, Vec<P>, P>, Error> {
        let decoder = Decoder::new(png);
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size().unwrap()];
        let info = reader.next_frame(&mut buf).unwrap();
        if info.color_type == Self::get_png_color_type() {
            Ok(Surface {
                size: USizeVec2::new(info.width as usize, info.height as usize),
                buffer: cast_slice::<u8, P>(&buf[..info.buffer_size()]).to_vec(),
                destination_rect: None,
                blit_area: None,
                _p: PhantomData,
            })
        } else {
            Err(Error::PngColorType(
                info.color_type,
                Self::get_png_color_type(),
            ))
        }
    }
}

impl_png!(u8, Grayscale);
impl_png!([u8; 2], GrayscaleAlpha);
impl_png!([u8; 3], Rgb);
impl_png!([u8; 4], Rgba);
