use crate::error::Error;
use crate::{L8Surface, La8Surface, Rgb8Surface, Rgba8Surface};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

macro_rules! impl_png {
    ($surface:ty, $color_type:ident, $bit_depth:ident) => {
        impl $surface {
            pub fn write_png<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
                let file = File::create(path.as_ref())
                    .map_err(|e| Error::PngFile(path.as_ref().to_path_buf(), e))?;
                let w = BufWriter::new(file);
                let mut encoder =
                    png::Encoder::new(w, self.rect.size.x as u32, self.rect.size.y as u32);
                encoder.set_color(png::ColorType::$color_type);
                encoder.set_depth(png::BitDepth::$bit_depth);
                let mut writer = encoder.write_header().map_err(Error::PngHeader)?;
                writer
                    .write_image_data(self.bytes())
                    .map_err(Error::PngPixels)
            }
        }
    };
}

impl_png!(L8Surface, Grayscale, Eight);
impl_png!(La8Surface, GrayscaleAlpha, Eight);
impl_png!(Rgb8Surface, Rgb, Eight);
impl_png!(Rgba8Surface, Rgba, Eight);
