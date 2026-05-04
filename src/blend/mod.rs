mod blend_mode;

#[cfg(feature = "std")]
use crate::lock::get_dst_index;
use crate::lock::get_indices;
use crate::{Error, RectU, Surface};
pub use blend_mode::BlendMode;

/// A hacky optimization.
/// We assume that we're converting to and from pixels with 8-bit channels.
/// So, this value is 254. / 255.
/// This will (hopefully!) help with floating point precision.
const EPSILON_255: f32 = 0.9960784;
/// Likewise, this is 1. / 255.
const EPSILON_0: f32 = 0.0039216;

type Pixel = [f32; 4];
type BlendFunction = fn(top: &[f32; 4], bottom: &[f32; 4]) -> Rgb;

macro_rules! blend_per_pixel {
    ($f:ident, $c:expr, $clamp:literal) => {
        const fn $f(top: &Pixel, bottom: &Pixel) -> Rgb {
            Rgb {
                r: $c(top, bottom, 0),
                g: $c(top, bottom, 1),
                b: $c(top, bottom, 2),
                clamp: $clamp,
            }
        }
    };
}

macro_rules! arithmetic {
    ($f:ident, $op:tt, $clamp:literal) => {
        const fn $f(top: &Pixel, bottom: &Pixel) -> Rgb {
            Rgb {
                r: bottom[0] $op top[0],
                g: bottom[1] $op top[1],
                b: bottom[2] $op top[2],
                clamp: $clamp
            }
        }
    };
}

macro_rules! light_dark {
    ($f:ident, $e:ident) => {
        const fn $f(top: &Pixel, bottom: &Pixel) -> Rgb {
            Rgb {
                r: bottom[0].$e(top[0]),
                g: bottom[1].$e(top[1]),
                b: bottom[2].$e(top[2]),
                clamp: false,
            }
        }
    };
}

/// A lightweight way to keep the returned RGB values grouped together.
pub struct Rgb {
    r: f32,
    g: f32,
    b: f32,
    pub(super) clamp: bool,
}

/// A blendable surface backed by a vec.
///
/// Source for most of the math: <https://en.wikipedia.org/wiki/Blend_modes>
#[cfg(feature = "std")]
pub type BlendableSurfaceVec<'s> = BlendableSurface<'s, Vec<Pixel>>;

/// A surface that allows you to blend pixels rather than copying them onto each other.
///
/// The underlying [Surface] must use `[f32; 4]` pixels, such as [crate::surface::Rgba32Surface]
///
/// ```
/// use blittle::{BlendMode, BlendableSurface, PositionI, Rgba32Surface, Size};
///
/// // Define a [f32; 4] surface.
/// let mut bottom = Rgba32Surface::new_filled(Size { width: 256, height: 256 }, [0.1, 0.3, 0.5, 0.9]);
///
/// // We will blend this surface on top of `bottom`.
/// let mut top = Rgba32Surface::new_filled(Size { width: 64, height: 120 }, [0.3, 0.95, 0.1, 1.]);
/// // Set the top surface's position relative to the bottom surface.
/// top.set_position(PositionI::new(1, 5), &bottom);
/// // Feed the top layer into a BlendableSurface.
/// let top = BlendableSurface::new(top);
///
/// // Blend.
/// let blend_mode = BlendMode::Overlay;
/// let alpha = 0.5;
/// top.blend(blend_mode, alpha, &mut bottom).unwrap();
/// ```
///
/// A BlendableSurface can be locked or unlocked.
/// A locked BlendableSurface usually blends faster but can't mutate its underlying surface pixel buffer.
/// When initially locked, a BlendableSurface caches the position of every non-transparent pixel.
/// In the above example, locking `top` would be make blending slightly *slower*
/// because the surface is filled with a uniform color.
/// Locking is useful when some pixels are transparent and some are not.
pub struct BlendableSurface<'s, S: AsRef<[Pixel]> + AsMut<[Pixel]>> {
    surface: Surface<'s, S, Pixel>,
    #[cfg(feature = "std")]
    indices: Option<Vec<usize>>,
}

impl<'s, S: AsRef<[Pixel]> + AsMut<[Pixel]>> BlendableSurface<'s, S> {
    pub fn new(surface: Surface<'s, S, Pixel>) -> Self {
        Self {
            surface,
            #[cfg(feature = "std")]
            indices: None,
        }
    }

    /// Returns a reference to the underlying surface.
    pub const fn surface(&self) -> &Surface<'s, S, Pixel> {
        &self.surface
    }

    /// Returns a mutable reference of the surface.
    /// Returns an error if the masked surface is locked.
    pub const fn surface_mut(&mut self) -> Result<&mut Surface<'s, S, Pixel>, Error> {
        #[cfg(feature = "std")]
        if self.is_locked() {
            Err(Error::Locked)
        } else {
            Ok(&mut self.surface)
        }
        #[cfg(not(feature = "std"))]
        Ok(&mut self.surface)
    }

    /// Blend into `other` using the `blend_mode` and an `alpha` transparency value (0-1).
    ///
    /// The `alpha` value is applied to each pixel.
    /// A value of 0 means that `other`'s pixels will be the original (non-blended) pixels.
    /// A value of 1 means that `other`'s pixels will be the blended pixels.
    /// A value of 0.5 means that `other`'s pixels will be halfway between the original and blended pixels.
    pub fn blend<B: AsRef<[Pixel]> + AsMut<[Pixel]>>(
        &self,
        blend_mode: BlendMode,
        alpha: f32,
        other: &mut Surface<'s, B, Pixel>,
    ) -> Result<(), Error> {
        let (destination_rect, blit_area) = self.surface.get_blit_params(other.size)?;

        let f = Self::get_blend_function(blend_mode);
        #[cfg(feature = "std")]
        match self.indices.as_ref() {
            Some(mask) => {
                self.blend_locked(f, alpha, destination_rect, mask, other);
            }
            None => {
                self.blend_unlocked(f, alpha, destination_rect, blit_area, other);
            }
        }
        #[cfg(not(feature = "std"))]
        self.blend_unlocked(f, alpha, destination_rect, blit_area, other);
        Ok(())
    }

    /// Lock the surface, optimizing blit speed while preventing pixel manipulation.
    #[cfg(feature = "std")]
    pub fn lock(&mut self) {
        if self.is_locked() {
            return;
        }

        self.indices = Some(
            self.surface
                .buffer()
                .iter()
                .enumerate()
                .filter_map(|(i, p)| if p[3] >= EPSILON_0 { Some(i) } else { None })
                .collect(),
        );
    }

    /// Returns true if the surface is locked.
    #[cfg(feature = "std")]
    pub const fn is_locked(&self) -> bool {
        self.indices.is_some()
    }

    /// Unlock the surface.
    /// Blit speed will be unoptimized while pixel manipulation will be permitted.
    #[cfg(feature = "std")]
    pub fn unlock(&mut self) {
        self.indices = None;
    }

    #[cfg(feature = "std")]
    fn blend_locked<B: AsRef<[Pixel]> + AsMut<[Pixel]>>(
        &self,
        f: BlendFunction,
        alpha: f32,
        destination_rect: RectU,
        mask: &[usize],
        other: &mut Surface<'_, B, Pixel>,
    ) {
        let dst_len = other.buffer().len();
        mask.iter().for_each(|i| {
            let i = *i;
            if let Some(dst_index) =
                get_dst_index(i, &destination_rect, dst_len, &self.surface, other)
            {
                Self::blend_pixel(
                    f,
                    alpha,
                    self.surface.buffer.as_ref()[i],
                    &mut other.buffer_mut()[dst_index],
                );
            }
        });
    }

    fn blend_unlocked<B: AsRef<[Pixel]> + AsMut<[Pixel]>>(
        &self,
        f: BlendFunction,
        alpha: f32,
        destination_rect: RectU,
        blit_area: RectU,
        other: &mut Surface<'_, B, Pixel>,
    ) {
        if alpha < EPSILON_0 {
            return;
        }
        (0..blit_area.size.height).for_each(|y| {
            (0..blit_area.size.width).for_each(|x| {
                let (src_index, dst_index) =
                    get_indices(x, y, &destination_rect, &blit_area, &self.surface, other);
                let p = self.surface.buffer.as_ref()[src_index];
                if p[3] >= EPSILON_0 {
                    Self::blend_pixel(
                        f,
                        alpha,
                        self.surface.buffer.as_ref()[src_index],
                        &mut other.buffer_mut()[dst_index],
                    );
                }
            })
        });
    }

    const fn get_blend_function(blend_mode: BlendMode) -> BlendFunction {
        match &blend_mode {
            BlendMode::Normal => Self::normal,
            BlendMode::Multiply => Self::multiply,
            BlendMode::Screen => Self::screen,
            BlendMode::Overlay => Self::overlay,
            BlendMode::HardLight => Self::hard_light,
            BlendMode::SoftLight => Self::soft_light,
            BlendMode::Dodge => Self::dodge,
            BlendMode::Burn => Self::burn,
            BlendMode::VividLight => Self::vivid_light,
            BlendMode::Divide => Self::divide,
            BlendMode::Add => Self::add,
            BlendMode::Subtract => Self::subtract,
            BlendMode::Difference => Self::difference,
            BlendMode::LightenOnly => Self::lighten_only,
            BlendMode::DarkenOnly => Self::darken_only,
        }
    }

    fn blend_pixel(f: BlendFunction, alpha: f32, top: [f32; 4], bottom: &mut [f32; 4]) {
        const fn lerp(a: f32, b: f32, t: f32) -> f32 {
            a + t * (b - a)
        }

        let a = top[3];
        // Blend.
        let rgb = f(&top, bottom);
        // Blend with composited alpha.
        let ca = (a + bottom[3] * (1. - a)) * alpha;
        if ca >= EPSILON_255 {
            bottom[0] = rgb.r;
            bottom[1] = rgb.g;
            bottom[2] = rgb.b;
        } else {
            bottom[0] = lerp(bottom[0], rgb.r, ca);
            bottom[1] = lerp(bottom[1], rgb.g, ca);
            bottom[2] = lerp(bottom[2], rgb.b, ca);
        }
        if rgb.clamp {
            bottom[0] = bottom[0].clamp(0., 1.);
            bottom[1] = bottom[1].clamp(0., 1.);
            bottom[2] = bottom[2].clamp(0., 1.);
        }
    }

    const fn normal(top: &Pixel, bottom: &Pixel) -> Rgb {
        const fn composite(top: &Pixel, bottom: &Pixel, a: f32, i: usize) -> f32 {
            top[i] + bottom[i] * (1. - a)
        }
        // Source: https://en.wikipedia.org/wiki/Alpha_compositing
        let a = top[3];
        Rgb {
            r: composite(top, bottom, a, 0),
            g: composite(top, bottom, a, 1),
            b: composite(top, bottom, a, 2),
            clamp: false,
        }
    }

    arithmetic!(multiply, *, false);

    const fn screen_channel(top: &Pixel, bottom: &Pixel, i: usize) -> f32 {
        1. - (1. - bottom[i]) * (1. - top[i])
    }
    blend_per_pixel!(screen, Self::screen_channel, false);

    const fn overlay(top: &Pixel, bottom: &Pixel) -> Rgb {
        Self::overlay_inner(top, bottom, Self::luminance(bottom))
    }

    const fn hard_light(top: &Pixel, bottom: &Pixel) -> Rgb {
        Self::overlay_inner(top, bottom, Self::luminance(top))
    }

    fn soft_light(top: &Pixel, bottom: &Pixel) -> Rgb {
        fn greater_than_half(top: &Pixel, bottom: &Pixel, i: usize) -> f32 {
            let a = bottom[i];
            let b = top[i];
            2. * a * b + (a * a) * (1. - 2. * b)
        }

        fn half(top: &Pixel, bottom: &Pixel, i: usize) -> f32 {
            let a = bottom[i];
            let b = top[i];
            (1. - 2. * b) * (a * a) + 2. * a * b
        }

        fn less_than_half(top: &Pixel, bottom: &Pixel, i: usize) -> f32 {
            let a = bottom[i];
            let b = top[i];
            2. * a * (1. - b) + a.sqrt() * (2. * b - 1.)
        }

        let lum = Self::luminance(top);
        let f = if lum < 0.5 {
            greater_than_half
        } else if lum == 0.5 {
            half
        } else {
            less_than_half
        };
        Rgb {
            r: f(top, bottom, 0),
            g: f(top, bottom, 1),
            b: f(top, bottom, 2),
            clamp: false,
        }
    }

    const fn dodge_channel(top: &Pixel, bottom: &Pixel, i: usize) -> f32 {
        bottom[i] / (1. - top[i])
    }
    blend_per_pixel!(dodge, Self::dodge_channel, true);

    const fn burn(top: &Pixel, bottom: &Pixel) -> Rgb {
        Self::dodge(bottom, top)
    }

    const fn vivid_light(top: &Pixel, bottom: &Pixel) -> Rgb {
        let lum = Self::luminance(top);
        if lum > 0.5 {
            Self::dodge(top, bottom)
        } else {
            Self::burn(top, bottom)
        }
    }

    arithmetic!(divide, /, false);

    arithmetic!(add, +, true);

    arithmetic!(subtract, -, true);

    const fn difference_channel(top: &Pixel, bottom: &Pixel, i: usize) -> f32 {
        (bottom[i] - top[i]).abs()
    }
    blend_per_pixel!(difference, Self::difference_channel, true);

    light_dark!(darken_only, min);

    light_dark!(lighten_only, max);

    /// Source: <https://github.com/emgyrz/colorsys.rs/blob/4a458d55110a802bb01c9f7123ea0535ab87749f/src/converters/rgb_to_hsl.rs#L6>
    const fn luminance(pixel: &Pixel) -> f32 {
        let max = pixel[0].max(pixel[1]).max(pixel[2]);
        let min = pixel[0].min(pixel[1]).min(pixel[2]);
        (max + min) * 0.5
    }

    const fn overlay_darker_channel(top: &Pixel, bottom: &Pixel, i: usize) -> f32 {
        2. * top[i] * bottom[i]
    }
    blend_per_pixel!(overlay_darker, Self::overlay_darker_channel, true);

    const fn overlay_lighter_channel(top: &Pixel, bottom: &Pixel, i: usize) -> f32 {
        1. - 2. * (1. - bottom[i]) * (1. - top[i])
    }
    blend_per_pixel!(overlay_lighter, Self::overlay_lighter_channel, true);

    const fn overlay_inner(top: &Pixel, bottom: &Pixel, luminance: f32) -> Rgb {
        if luminance < 0.5 {
            Self::overlay_darker(top, bottom)
        } else {
            Self::overlay_lighter(top, bottom)
        }
    }
}

#[cfg(feature = "png")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::png::Png;
    use crate::{PositionI, Rgb8Surface, Rgba8Surface, Rgba32Surface};
    use std::io::Cursor;

    #[test]
    fn test_overlay() {
        let bottom =
            Rgba8Surface::read_png(Cursor::new(include_bytes!("../../test_images/plasma.png")))
                .unwrap();
        let top = Rgb8Surface::read_png(Cursor::new(include_bytes!("../../test_images/text.png")))
            .unwrap();

        let mut top_blendable = BlendableSurface::new(Rgba32Surface::from(&top));
        let mut bottom = Rgba32Surface::from(&bottom);

        top_blendable
            .surface_mut()
            .unwrap()
            .set_position(PositionI::new(100, 50), &bottom)
            .unwrap();
        top_blendable.lock();
        top_blendable
            .blend(BlendMode::Overlay, 0.5, &mut bottom)
            .unwrap();

        Rgba8Surface::write_png(&Rgba8Surface::from(&bottom), "test_output/overlay.png").unwrap();
    }
}
