use super::rgb::Rgb;
use super::{BlendFunction, EPSILON_255, Pixel};
use paste::paste;

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

macro_rules! pub_blend_fn {
    ($f:ident) => {
        paste! {
            pub fn [<blend_ $f>](top: Pixel, bottom: &mut Pixel, alpha: f32) {
                Self::blend_pixel(Self::$f, top, bottom, alpha);
            }
        }
    };
}

/// Color blend modes.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BlendMode {
    /// Interpolate between the pixels.
    Normal,
    /// Multiply the pixels.
    Multiply,
    /// Multiply the inverse of the pixels, then return the inverse of the result.
    Screen,
    /// Multiply or screen depending on which pixel is lighter.
    Overlay,
    /// The opposite of overlay.
    HardLight,
    /// It's like overlay, sort of.
    /// <https://en.wikipedia.org/wiki/Blend_modes#Soft_Light>
    SoftLight,
    /// Divide the bottom pixel by the inverted top pixel.
    Dodge,
    /// Divide the bottom pixel by the inverted top pixel.
    Burn,
    /// Dodge or blend depending on which pixel is lighter.
    VividLight,
    /// Divide the bottom pixel by the top pixel.
    Divide,
    /// Add the two pixels.
    Add,
    /// Subtract the top pixel from the bottom pixel.
    Subtract,
    /// The absolute value of `Subtract`.
    Difference,
    /// Each color channel is set to the darker of the corresponding channels of the two pixels.
    DarkenOnly,
    /// Each color channel is set to the lighter of the corresponding channels of the two pixels.
    LightenOnly,
}

impl BlendMode {
    pub_blend_fn!(normal);
    pub_blend_fn!(multiply);
    pub_blend_fn!(screen);
    pub_blend_fn!(overlay);
    pub_blend_fn!(hard_light);
    pub_blend_fn!(soft_light);
    pub_blend_fn!(dodge);
    pub_blend_fn!(burn);
    pub_blend_fn!(vivid_light);
    pub_blend_fn!(divide);
    pub_blend_fn!(add);
    pub_blend_fn!(subtract);
    pub_blend_fn!(difference);
    pub_blend_fn!(lighten_only);
    pub_blend_fn!(darken_only);

    pub(super) const fn get_blend_function(&self) -> BlendFunction {
        match self {
            Self::Normal => Self::normal,
            Self::Multiply => Self::multiply,
            Self::Screen => Self::screen,
            Self::Overlay => Self::overlay,
            Self::HardLight => Self::hard_light,
            Self::SoftLight => Self::soft_light,
            Self::Dodge => Self::dodge,
            Self::Burn => Self::burn,
            Self::VividLight => Self::vivid_light,
            Self::Divide => Self::divide,
            Self::Add => Self::add,
            Self::Subtract => Self::subtract,
            Self::Difference => Self::difference,
            Self::LightenOnly => Self::lighten_only,
            Self::DarkenOnly => Self::darken_only,
        }
    }

    pub(super) fn blend_pixel(f: BlendFunction, top: Pixel, bottom: &mut Pixel, alpha: f32) {
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
        const fn greater_than_half(top: &Pixel, bottom: &Pixel, i: usize) -> f32 {
            let a = bottom[i];
            let b = top[i];
            2. * a * b + (a * a) * (1. - 2. * b)
        }

        const fn half(top: &Pixel, bottom: &Pixel, i: usize) -> f32 {
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
