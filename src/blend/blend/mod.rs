#[cfg(feature = "dissolve")]
pub(super) mod dissolve;

/// A hacky optimization.
/// We assume that we're converting to and from pixels with 8-bit channels.
/// So, this value is 254. / 255.
/// This will (hopefully!) help with floating point precision.
const EPSILON_255: f32 = 0.9960784;
// Likewise, this is 1. / 255.
const EPSILON_0: f32 = 0.0039216;

type Pixel = [f32; 4];

macro_rules! blend_mode {
    ($s:ident, $blend:expr) => {
        #[derive(Default)]
        pub struct $s;

        impl Blend for $s {
            fn blend_mode(&mut self, top: &Pixel, bottom: &mut Pixel) {
                $blend(top, bottom);
            }
        }
    };
}

macro_rules! arithmetic {
    ($s:ident, $f:ident, $op:tt) => {
        const fn $f(top: &Pixel, bottom: &mut Pixel) {
            bottom[0] $op top[0];
            bottom[1] $op top[1];
            bottom[2] $op top[2];
        }

        blend_mode!($s, $f);
    };
}

macro_rules! light_dark {
    ($s:ident, $f:ident, $e:ident) => {
        const fn $f(top: &Pixel, bottom: &mut Pixel) {
            bottom[0] = bottom[0].$e(top[0]);
            bottom[1] = bottom[1].$e(top[1]);
            bottom[2] = bottom[2].$e(top[2]);
        }

        blend_mode!($s, $f);
    };
}

macro_rules! arithmetic_clamp {
    ($s:ident, $f:ident, $op:tt) => {
        const fn $f(top: &Pixel, bottom: &mut Pixel) {
            bottom[0] = (bottom[0] $op top[0]).clamp(0., 1.);
            bottom[1] = (bottom[1] $op top[1]).clamp(0., 1.);
            bottom[2] = (bottom[2] $op top[2]).clamp(0., 1.);
        }

          blend_mode!($s, $f);
    };
}

macro_rules! blend_mode_per_pixel {
    ($s:ident, $f:expr) => {
        #[derive(Default)]
        pub struct $s;

        impl Blend for $s {
            fn blend_mode(&mut self, top: &Pixel, bottom: &mut Pixel) {
                $f(top, bottom, 0);
                $f(top, bottom, 1);
                $f(top, bottom, 2);
            }
        }
    };
}

pub trait Blend {
    fn blend(&mut self, top: &Pixel, bottom: &mut Pixel, alpha: f32) {
        let a = top[3];
        if a > EPSILON_255 && alpha > EPSILON_255 {
            // Blit.
            *bottom = *top;
        } else if a >= EPSILON_0 && alpha >= EPSILON_0 {
            // Blend.
            self.blend_mode(top, bottom);
            // Composited alpha.
            bottom[3] = a + bottom[3] * alpha * (1. - a);
        }
    }

    fn blend_mode(&mut self, top: &Pixel, bottom: &mut Pixel);
}

const fn normal(top: &Pixel, bottom: &mut Pixel) {
    const fn composite(top: &Pixel, bottom: &mut Pixel, a: f32, i: usize) {
        bottom[i] = top[i] + bottom[i] * (1. - a);
    }
    // Source: https://en.wikipedia.org/wiki/Alpha_compositing
    let a = top[3];
    composite(top, bottom, a, 0);
    composite(top, bottom, a, 1);
    composite(top, bottom, a, 2);
}
blend_mode!(Normal, normal);

arithmetic!(Multiply, multiply, *=);

const fn screen(top: &Pixel, bottom: &mut Pixel, i: usize) {
    bottom[i] = 1. - (1. - bottom[i]) * (1. - top[i]);
}
blend_mode_per_pixel!(Screen, screen);

const fn overlay(top: &Pixel, bottom: &mut Pixel) {
    overlay_inner(top, bottom, luminance(bottom));
}
blend_mode!(Overlay, overlay);

const fn hard_light(top: &Pixel, bottom: &mut Pixel) {
    overlay_inner(top, bottom, luminance(top));
}
blend_mode!(HardLight, hard_light);

fn soft_light(top: &Pixel, bottom: &mut Pixel) {
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

    let lum = luminance(top);
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
blend_mode!(SoftLight, soft_light);

const fn dodge(top: &Pixel, bottom: &mut Pixel) {
    bottom[0] = get_dodge(top, bottom, 0);
    bottom[1] = get_dodge(top, bottom, 1);
    bottom[2] = get_dodge(top, bottom, 2);
}
blend_mode!(Dodge, dodge);
const fn burn(top: &Pixel, bottom: &mut Pixel) {
    bottom[0] = get_dodge(bottom, top, 0);
    bottom[1] = get_dodge(bottom, top, 1);
    bottom[2] = get_dodge(bottom, top, 2);
}
blend_mode!(Burn, burn);

const fn vivid_light(top: &Pixel, bottom: &mut Pixel) {
    let lum = luminance(top);
    if lum > 0.5 {
        dodge(top, bottom);
    } else {
        burn(top, bottom);
    }
}
blend_mode!(VividLight, vivid_light);

arithmetic!(Divide, divide, /=);

arithmetic_clamp!(Add, add, +);

arithmetic_clamp!(Subtract, subtract, -);

const fn difference(top: &Pixel, bottom: &mut Pixel, i: usize) {
    bottom[i] = (bottom[i] - top[i]).abs().clamp(0., 1.);
}
blend_mode_per_pixel!(Difference, difference);

light_dark!(DarkenOnly, darken_only, min);

light_dark!(LightenOnly, lighten_only, max);

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
        multiply_two(top, bottom);
    } else {
        over(top, bottom, 0);
        over(top, bottom, 1);
        over(top, bottom, 2);
    }
}

const fn get_dodge(top: &Pixel, bottom: &Pixel, i: usize) -> f32 {
    (bottom[i] / (1. - top[i])).clamp(0., 1.)
}
