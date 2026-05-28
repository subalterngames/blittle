use crate::{Size, Surface};
use bytemuck::{Pod, Zeroable, cast_slice};
use jpeg_encoder::{ColorType, Encoder};
use std::io::{BufRead, Seek};
use std::marker::PhantomData;
use std::path::Path;
use thiserror::Error;
use zune_jpeg::zune_core::colorspace::ColorSpace;

#[derive(Debug, Error)]
pub enum JpgError {
    #[error("Failed to create jpg file for {0} Reason: {1}")]
    WriteFile(std::path::PathBuf, jpeg_encoder::EncodingError),
    #[error("Failed to encode jpg data for {0} Reason: {1}")]
    Encode(std::path::PathBuf, jpeg_encoder::EncodingError),
    #[error("Failed to decode jpg header data: {0}")]
    DecodeHeaders(zune_jpeg::errors::DecodeErrors),
    #[error("Failed to decode jpg pixel data: {0}")]
    DecodePixels(zune_jpeg::errors::DecodeErrors),
    #[error("Failed to get jpg color space")]
    Colorspace,
    #[error("Failed to get jpg dimensions")]
    Dimensions,
    #[error("Invalid jpg color space: {:?} gf", 0)]
    InvalidColorspace(ColorType),
    #[error("Expected color space {:?} but got {:?}", expected, actual)]
    WrongColorSpace {
        expected: ColorSpace,
        actual: ColorSpace,
    },
}

macro_rules! impl_jpg {
    ($p:ty, $color_type:ident) => {
        impl<S: AsRef<[$p]> + AsMut<[$p]>> Jpg<S, $p> for Surface<'_, S, $p> {
            fn get_jpg_color_type() -> ColorType {
                ColorType::$color_type
            }
        }
    };
}

/// Read and write surfaces to/from .jpg files.
///
/// ```
/// use std::fs::File;
/// use std::io::BufReader;
/// use blittle::*;
/// use blittle::jpg::Jpg;
///
/// let surface = Rgb8Surface::read_jpg(BufReader::new(File::open("test_images/plasma.jpg").unwrap())).unwrap();
/// Rgb8Surface::write_jpg(&surface, "test_output/plasma.jpg").unwrap();
/// ```
pub trait Jpg<S: AsRef<[P]> + AsMut<[P]>, P: Copy + Clone + Sized + Default + Zeroable + Pod> {
    /// Returns the expected .jpg color type.
    fn get_jpg_color_type() -> ColorType;

    /// Write to a .jpg file.
    ///
    /// Returns an error if a file at `path` can't be created
    /// or if the buffer in `surface` is somehow invalid.
    fn write_jpg<Pa: AsRef<Path>>(surface: &Surface<'_, S, P>, path: Pa) -> Result<(), JpgError> {
        let encoder = Encoder::new_file(path.as_ref(), 100)
            .map_err(|e| JpgError::WriteFile(path.as_ref().to_path_buf(), e))?;
        encoder
            .encode(
                surface.bytes(),
                surface.size.width as u16,
                surface.size.height as u16,
                Self::get_jpg_color_type(),
            )
            .map_err(|e| JpgError::Encode(path.as_ref().to_path_buf(), e))
    }

    /// Read from a .jpg file.
    ///
    /// Returns if `jpg` does not contain valid .jpg data,
    /// or if its color type doesn't match [Self::get_jpg_color_type].
    fn read_jpg<'s, B: BufRead + Seek>(jpg: B) -> Result<Surface<'s, Vec<P>, P>, JpgError> {
        let mut decoder = zune_jpeg::JpegDecoder::new(jpg);
        decoder.decode_headers().map_err(JpgError::DecodeHeaders)?;

        let (width, height) = decoder.dimensions().ok_or(JpgError::Dimensions)?;

        let pixels = decoder.decode().map_err(JpgError::DecodePixels)?;
        let colorspace = decoder.output_colorspace().ok_or(JpgError::Colorspace)?;
        let expected_colorspace = match Self::get_jpg_color_type() {
            ColorType::Luma => Ok(ColorSpace::Luma),
            ColorType::Rgb => Ok(ColorSpace::RGB),
            ColorType::Rgba => Ok(ColorSpace::RGBA),
            other => Err(JpgError::InvalidColorspace(other)),
        }?;
        if colorspace == expected_colorspace {
            Ok(Surface {
                size: Size::new(width, height),
                buffer: cast_slice::<u8, P>(&pixels).to_vec(),
                destination_rect: None,
                blit_area: None,
                _p: PhantomData,
            })
        } else {
            Err(JpgError::WrongColorSpace {
                expected: expected_colorspace,
                actual: colorspace,
            })
        }
    }
}

impl_jpg!(u8, Luma);
impl_jpg!([u8; 3], Rgb);
impl_jpg!([u8; 4], Rgba);
