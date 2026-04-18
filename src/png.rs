use crate::error::Error;
use crate::surface_trait::SurfaceTrait;
use crate::{L8Surface, La8Surface, Rgb8Surface, Rgba8Surface};
use bytemuck::{Pod, Zeroable};
use png::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub trait PngWriter<P: Copy + Clone + Sized + Default + Zeroable + Pod>: SurfaceTrait<P> {
    fn get_png_color_type() -> ColorType;

    fn write_png<Pa: AsRef<Path>>(&self, path: Pa) -> Result<(), Error> {
        let file = File::create(path.as_ref())
            .map_err(|e| Error::PngFile(path.as_ref().to_path_buf(), e))?;
        let w = BufWriter::new(file);
        let size = self.get_size();
        let mut encoder = Encoder::new(w, size.x as u32, size.y as u32);
        encoder.set_color(Self::get_png_color_type());
        encoder.set_depth(BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(Error::PngHeader)?;
        writer
            .write_image_data(self.bytes())
            .map_err(Error::PngPixels)
    }
}

impl PngWriter<u8> for L8Surface<'_> {
    fn get_png_color_type() -> ColorType {
        ColorType::Grayscale
    }
}

impl PngWriter<[u8; 2]> for La8Surface<'_> {
    fn get_png_color_type() -> ColorType {
        ColorType::GrayscaleAlpha
    }
}

impl PngWriter<[u8; 3]> for Rgb8Surface<'_> {
    fn get_png_color_type() -> ColorType {
        ColorType::Rgb
    }
}

impl PngWriter<[u8; 4]> for Rgba8Surface<'_> {
    fn get_png_color_type() -> ColorType {
        ColorType::Rgba
    }
}
