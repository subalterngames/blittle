mod blend_mode;
mod blender;

use crate::Surface;
use crate::blend::blender::Blender;
use crate::lock::LockableSurface;
pub use blend_mode::BlendMode;

type Pixel = [f32; 4];

macro_rules! blend_mode_per_pixel {
    ($f:ident, $c:expr) => {
        const fn $f(top: &Pixel, bottom: &Pixel) -> Rgb {
            Rgb::new($c(top, bottom, 0), $c(top, bottom, 1), $c(top, bottom, 2))
        }
    };
}

macro_rules! arithmetic {
    ($f:ident, $op:tt) => {
        const fn $f(top: &Pixel, bottom: &Pixel) -> Rgb {
            Rgb::new(bottom[0] $op top[0],
            bottom[1] $op top[1],
            bottom[2] $op top[2],)
        }
    };
}

macro_rules! arithmetic_clamp {
    ($f:ident, $op:tt) => {
        const fn $f(top: &Pixel, bottom: &Pixel) -> Rgb {
            Rgb::new((bottom[0] $op top[0]).clamp(0., 1.),
            (bottom[1] $op top[1]).clamp(0., 1.),
            (bottom[2] $op top[2]).clamp(0., 1.),)
        }
    };
}

macro_rules! light_dark {
    ($f:ident, $e:ident) => {
        const fn $f(top: &Pixel, bottom: &Pixel) -> Rgb {
            Rgb::new(
                bottom[0].$e(top[0]),
                bottom[1].$e(top[1]),
                bottom[2].$e(top[2]),
            )
        }
    };
}

/// A lightweight way to keep the returned RGB values grouped together.
pub struct Rgb {
    r: f32,
    g: f32,
    b: f32,
}

impl Rgb {
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}

/// A surface that allows you to blend pixels rather than copying them onto each other.
///
/// A BlendableSurface can be locked or unlocked.
/// If locked, the surface can't be mutated, but blending will be faster.
pub type BlendableSurface<'s, S> = LockableSurface<'s, S, [f32; 4], Blender>;
/// A blendable surface backed by a vec.
#[cfg(feature = "std")]
pub type BlendableSurfaceVec<'s> = BlendableSurface<'s, Vec<Pixel>>;

impl<'s, S: AsRef<[[f32; 4]]> + AsMut<[[f32; 4]]>> BlendableSurface<'s, S> {
    pub fn new(surface: Surface<'s, S, [f32; 4]>) -> Self {
        Self {
            surface,
            blitter: Blender::new(Self::normal, 1.),
            #[cfg(feature = "std")]
            mask: None,
        }
    }

    /// Set the blend mode and alpha transparency for this surface (0-1).
    pub const fn set_blend_mode(&mut self, blend_mode: BlendMode, alpha: f32) {
        let f = match &blend_mode {
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
        };
        self.blitter = Blender::new(f, alpha);
    }

    const fn normal(top: &Pixel, bottom: &Pixel) -> Rgb {
        const fn composite(top: &Pixel, bottom: &Pixel, a: f32, i: usize) -> f32 {
            top[i] + bottom[i] * (1. - a)
        }
        // Source: https://en.wikipedia.org/wiki/Alpha_compositing
        let a = top[3];
        Rgb::new(
            composite(top, bottom, a, 0),
            composite(top, bottom, a, 1),
            composite(top, bottom, a, 2),
        )
    }

    arithmetic!(multiply, *);

    const fn screen_channel(top: &Pixel, bottom: &Pixel, i: usize) -> f32 {
        1. - (1. - bottom[i]) * (1. - top[i])
    }
    blend_mode_per_pixel!(screen, Self::screen_channel);

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
        Rgb::new(f(top, bottom, 0), f(top, bottom, 1), f(top, bottom, 2))
    }

    const fn dodge(top: &Pixel, bottom: &Pixel) -> Rgb {
        Rgb::new(
            Self::get_dodge(top, bottom, 0),
            Self::get_dodge(top, bottom, 1),
            Self::get_dodge(top, bottom, 2),
        )
    }

    const fn burn(top: &Pixel, bottom: &Pixel) -> Rgb {
        Rgb::new(
            Self::get_dodge(bottom, top, 0),
            Self::get_dodge(bottom, top, 1),
            Self::get_dodge(bottom, top, 2),
        )
    }

    const fn vivid_light(top: &Pixel, bottom: &Pixel) -> Rgb {
        let lum = Self::luminance(top);
        if lum > 0.5 {
            Self::dodge(top, bottom)
        } else {
            Self::burn(top, bottom)
        }
    }

    arithmetic!(divide, /);

    arithmetic_clamp!(add, +);

    arithmetic_clamp!(subtract, -);

    const fn difference_channel(top: &Pixel, bottom: &Pixel, i: usize) -> f32 {
        (bottom[i] - top[i]).abs().clamp(0., 1.)
    }
    blend_mode_per_pixel!(difference, Self::difference_channel);

    light_dark!(darken_only, min);

    light_dark!(lighten_only, max);

    /// Source: <https://github.com/emgyrz/colorsys.rs/blob/4a458d55110a802bb01c9f7123ea0535ab87749f/src/converters/rgb_to_hsl.rs#L6>
    const fn luminance(pixel: &Pixel) -> f32 {
        let max = pixel[0].max(pixel[1]).max(pixel[2]);
        let min = pixel[0].min(pixel[1]).min(pixel[2]);
        (max + min) * 0.5
    }

    const fn multiply_two(top: &Pixel, bottom: &Pixel) -> Rgb {
        const fn ttb(top: &Pixel, bottom: &Pixel, i: usize) -> f32 {
            (2. * top[i] * bottom[i]).clamp(0., 1.)
        }

        Rgb::new(
            ttb(top, bottom, 0),
            ttb(top, bottom, 1),
            ttb(top, bottom, 2),
        )
    }

    const fn overlay_inner(top: &Pixel, bottom: &Pixel, lum: f32) -> Rgb {
        const fn over(top: &Pixel, bottom: &Pixel, i: usize) -> f32 {
            1. - 2. * (1. - bottom[i]) * (1. - top[i]).clamp(0., 1.)
        }

        if lum < 0.5 {
            Self::multiply_two(top, bottom)
        } else {
            Rgb::new(
                over(top, bottom, 0),
                over(top, bottom, 1),
                over(top, bottom, 2),
            )
        }
    }

    const fn get_dodge(top: &Pixel, bottom: &Pixel, i: usize) -> f32 {
        (bottom[i] / (1. - top[i])).clamp(0., 1.)
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
        top_blendable.set_blend_mode(BlendMode::Overlay, 0.5);
        top_blendable.lock();
        top_blendable.blit(&mut bottom).unwrap();

        Rgba8Surface::write_png(&Rgba8Surface::from(&bottom), "test_output/overlay.png").unwrap();
    }
}
