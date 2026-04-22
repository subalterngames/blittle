use crate::error::Error;
use crate::{Size, Surface};
use bytemuck::{Pod, Zeroable, cast_slice};
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

/// Read and write surfaces to/from .png files.
///
/// ```
/// use std::fs::File;
/// use std::io::BufReader;
/// use blittle::*;
/// use blittle::png::Png;
///
/// let surface = Rgba8Surface::read_png(BufReader::new(File::open("test_images/plasma.png").unwrap())).unwrap();
/// Rgba8Surface::write_png(&surface, "test_output/plasma.png").unwrap();
/// ```
pub trait Png<S: AsRef<[P]> + AsMut<[P]>, P: Copy + Clone + Sized + Default + Zeroable + Pod> {
    /// Returns the expected .png color type.
    fn get_png_color_type() -> ColorType;

    /// Write to a .png file.
    ///
    /// Returns an error if a file at `path` can't be created
    /// or if the buffer in `surface` is somehow invalid.
    fn write_png<Pa: AsRef<Path>>(surface: &Surface<'_, S, P>, path: Pa) -> Result<(), Error> {
        let file = File::create(path.as_ref())
            .map_err(|e| Error::PngFile(path.as_ref().to_path_buf(), e))?;
        let w = BufWriter::new(file);
        let size = surface.get_size();
        let mut encoder = Encoder::new(w, size.width as u32, size.height as u32);
        encoder.set_color(Self::get_png_color_type());
        encoder.set_depth(BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(Error::PngHeader)?;
        writer
            .write_image_data(surface.bytes())
            .map_err(Error::PngPixels)
    }

    /// Read from a .png file.
    ///
    /// Returns if `png` does not contain valid .png data,
    /// or if its color type doesn't match [Self::get_png_color_type].
    fn read_png<'s, B: BufRead + Seek>(png: B) -> Result<Surface<'s, Vec<P>, P>, Error> {
        let decoder = Decoder::new(png);
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size().unwrap()];
        let info = reader.next_frame(&mut buf).unwrap();
        if info.color_type == Self::get_png_color_type() {
            Ok(Surface {
                size: Size::new(info.width as usize, info.height as usize),
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
