use rand::RngExt;
use rand::prelude::ThreadRng;
use crate::blend::{EPSILON_0, EPSILON_255};

macro_rules! arithmetic {
    ($f:ident, $op:tt) => {
        pub const fn $f(top: &Pixel, bottom: &mut Pixel) {
            bottom[0] $op top[0];
            bottom[1] $op top[1];
            bottom[2] $op top[2];
        }
    };
}

macro_rules! arithmetic_clamp {
    ($f:ident, $op:tt) => {
        pub const fn $f(top: &Pixel, bottom: &mut Pixel) {
            bottom[0] = (bottom[0] $op top[0]).clamp(0., 1.);
            bottom[1] = (bottom[1] $op top[1]).clamp(0., 1.);
            bottom[2] = (bottom[2] $op top[2]).clamp(0., 1.);
        }
    };
}

macro_rules! light_dark {
    ($f:ident, $e:ident) => {
        pub const fn $f(top: &Pixel, bottom: &mut Pixel) {
        bottom[0] = bottom[0].$e(top[0]);
        bottom[1] = bottom[1].$e(top[1]);
        bottom[2] = bottom[2].$e(top[2]);
    }
    };
}

type Pixel = [f32; 4];

/// Standard color blend modes.
/// 
/// Source for most of the math: <https://en.wikipedia.org/wiki/Blend_modes>
pub enum BlendMode {
    Normal,
    #[cfg(feature = "dissolve")]
    Dissolve(ThreadRng),
    Multiply,
    Screen,
    Overlay,
    HardLight,
    SoftLight,
    Dodge,
    Burn,
    VividLight,
    Divide,
    Add,
    Subtract,
    Difference,
    DarkenOnly,
    LightenOnly,
}

impl BlendMode {
    /// Blend `top` onto `bottom` with an `alpha` channel multiplier.
    pub fn blend(&mut self, top: &Pixel, bottom: &mut Pixel, alpha: f32) {
        // Copy the pixel.
        let a = top[3];
        if a > EPSILON_255 && alpha > EPSILON_255 {
            *bottom = *top;
        } else if a >= EPSILON_0 && alpha >= EPSILON_0 {
            match self {
                Self::Normal => Self::normal(top, bottom),
                #[cfg(feature = "dissolve")]
                Self::Dissolve(rng) => Self::dissolve(top, bottom, rng),
                Self::Multiply => Self::multiply(top, bottom),
                Self::Screen => Self::screen(top, bottom),
                Self::Overlay => Self::overlay(top, bottom),
                Self::SoftLight => Self::soft_light(top, bottom),
                Self::HardLight => Self::hard_light(top, bottom),
                Self::Dodge => Self::dodge(top, bottom),
                Self::Burn => Self::burn(top, bottom),
                Self::VividLight => Self::vivid_light(top, bottom),
                Self::Divide => Self::divide(top, bottom),
                Self::Add => Self::add(top, bottom),
                Self::Subtract => Self::subtract(top, bottom),
                Self::Difference => Self::difference(top, bottom),
                Self::DarkenOnly => Self::darken_only(top, bottom),
                Self::LightenOnly => Self::lighten_only(top, bottom),
            }
            // Composited alpha.
            bottom[3] = a + bottom[3] * alpha * (1. - a);
        }
    }
    pub const fn normal(top: &Pixel, bottom: &mut Pixel) {
        const fn composite(top: &Pixel, bottom: &mut Pixel, a: f32, i: usize) {
            bottom[i] = top[i] + bottom[i] * (1. - a);
        }
        // Source: https://en.wikipedia.org/wiki/Alpha_compositing
        let a = top[3];
        composite(top, bottom, a, 0);
        composite(top, bottom, a, 1);
        composite(top, bottom, a, 2);
    }

    arithmetic!(multiply, *=);

    #[cfg(feature = "dissolve")]
    pub fn dissolve(top: &Pixel, bottom: &mut Pixel, rng: &mut ThreadRng) {
        if rng.random_bool(0.5) {
            *bottom = *top;
        }
    }

    pub const fn screen(top: &Pixel, bottom: &mut Pixel) {
        const fn blend(top: &Pixel, bottom: &mut Pixel, i: usize) {
            bottom[i] = 1. - (1. - bottom[i]) * (1. - top[i]);
        }

        blend(top, bottom, 0);
        blend(top, bottom, 1);
        blend(top, bottom, 2);
    }

    pub const fn overlay(top: &Pixel, bottom: &mut Pixel) {
        Self::overlay_inner(top, bottom, Self::luminance(bottom));
    }

    pub const fn hard_light(top: &Pixel, bottom: &mut Pixel) {
        Self::overlay_inner(top, bottom, Self::luminance(top));
    }

    pub fn soft_light(top: &Pixel, bottom: &mut Pixel) {
        fn greater_than_half(top: &Pixel, bottom: &mut Pixel, i: usize) {
            let a = bottom[i];
            let b = top[i];
            bottom[i] = 2. * a * b + (a * a) * (1. - 2. * b);
        }

        fn half(top: &Pixel, bottom: &mut Pixel, i: usize) {
            let a = bottom[i];
            let b = top[i];
            bottom[i] = (1. - 2. * b) * (a * a) + 2. * a * b
        }

        fn less_than_half(top: &Pixel, bottom: &mut Pixel, i: usize) {
            let a = bottom[i];
            let b = top[i];
            bottom[i] = 2. * a * (1. - b) + a.sqrt() * (2. * b - 1.)
        }

        let lum = Self::luminance(top);
        let f = if lum < 0.5 {
            greater_than_half
        } else if lum == 0.5 {
            half
        } else {
            less_than_half
        };
        f(top, bottom, 0);
        f(top, bottom, 1);
        f(top, bottom, 2);
    }

    pub const fn dodge(top: &Pixel, bottom: &mut Pixel) {
        bottom[0] = Self::get_dodge(top, bottom, 0);
        bottom[1] = Self::get_dodge(top, bottom, 1);
        bottom[2] = Self::get_dodge(top, bottom, 2);
    }

    pub const fn burn(top: &Pixel, bottom: &mut Pixel) {
        bottom[0] = Self::get_dodge(bottom, top, 0);
        bottom[1] = Self::get_dodge(bottom, top, 1);
        bottom[2] = Self::get_dodge(bottom, top, 2);
    }

    pub const fn vivid_light(top: &Pixel, bottom: &mut Pixel) {
        let lum = Self::luminance(top);
        if lum > 0.5 {
            Self::dodge(top, bottom);
        } else {
            Self::burn(top, bottom);
        }
    }

    arithmetic!(divide, /=);

    arithmetic_clamp!(add, +);

    arithmetic_clamp!(subtract, -);

    pub const fn difference(top: &Pixel, bottom: &mut Pixel) {
        const fn blend(top: &Pixel, bottom: &mut Pixel, i: usize) {
            bottom[i] = (bottom[i] - top[i]).abs().clamp(0., 1.);
        }

        blend(top, bottom, 0);
        blend(top, bottom, 1);
        blend(top, bottom, 2);
    }

    light_dark!(darken_only, min);

    light_dark!(lighten_only, max);

    const fn get_dodge(top: &Pixel, bottom: &Pixel, i: usize) -> f32 {
        (bottom[i] / (1. - top[i])).clamp(0., 1.)
    }

    /// Source: <https://github.com/emgyrz/colorsys.rs/blob/4a458d55110a802bb01c9f7123ea0535ab87749f/src/converters/rgb_to_hsl.rs#L6>
    const fn luminance(pixel: &Pixel) -> f32 {
        let max = pixel[0].max(pixel[1]).max(pixel[2]);
        let min = pixel[0].min(pixel[1]).min(pixel[2]);
        (max + min) * 0.5
    }

    const fn multiply_two(top: &Pixel, bottom: &mut Pixel) {
        const fn ttb(top: &Pixel, bottom: &mut Pixel, i: usize) {
            bottom[i] = (2. * top[i] * bottom[i]).clamp(0., 1.)
        }

        ttb(top, bottom, 0);
        ttb(top, bottom, 1);
        ttb(top, bottom, 2);
    }

    const fn overlay_inner(top: &Pixel, bottom: &mut Pixel, lum: f32) {
        const fn over(top: &Pixel, bottom: &mut Pixel, i: usize) {
            bottom[i] = 1. - 2. * (1. - bottom[i]) * (1. - top[i]).clamp(0., 1.);
        }

        if lum < 0.5 {
            Self::multiply_two(top, bottom);
        } else {
            over(top, bottom, 0);
            over(top, bottom, 1);
            over(top, bottom, 2);
        }
    }
}
