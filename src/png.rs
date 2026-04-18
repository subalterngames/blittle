use crate::error::Error;
use crate::{
    L8Surface, L32Surface, La8Surface, La32Surface, Rgb8Surface, Rgba8Surface, Rgba32Surface,
};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

macro_rules! impl_png {
    ($surface:ty, $color_type:ident, $bit_depth:ident, $bytes:ident) => {
        impl $surface {
            pub fn write_png<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
                let file = File::create(path.as_ref())
                    .map_err(|e| Error::PngFile(path.as_ref().to_path_buf(), e))?;
                let w = BufWriter::new(file);
                let mut encoder = png::Encoder::new(w, self.size.x as u32, self.size.y as u32);
                encoder.set_color(png::ColorType::$color_type);
                encoder.set_depth(png::BitDepth::$bit_depth);
                let mut writer = encoder.write_header().map_err(Error::PngHeader)?;
                writer
                    .write_image_data(&self.$bytes())
                    .map_err(Error::PngPixels)
            }
        }
    };
}

macro_rules! impl_png_bytes {
    ($surface:ty, $color_type:ident, $bit_depth:ident) => {
        impl_png!($surface, $color_type, $bit_depth, bytes);
    };
}

macro_rules! impl_png_convert {
    ($from:ty, $to:tt) => {
        impl $from {
            pub fn write_png<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
                $to::from(self).write_png(path)
            }
        }
    };
}

impl_png_bytes!(L8Surface, Grayscale, Eight);
impl_png_bytes!(La8Surface, GrayscaleAlpha, Eight);
impl_png_bytes!(Rgb8Surface, Rgb, Eight);
impl_png_bytes!(Rgba8Surface, Rgba, Eight);

impl_png_convert!(L32Surface, L8Surface);
impl_png_convert!(La32Surface, La8Surface);
impl_png_convert!(Rgba32Surface, Rgba8Surface);
