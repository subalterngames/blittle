use crate::blend::{EPSILON_0, EPSILON_255};
use rand::RngExt;
use rand::prelude::ThreadRng;
use super::blend::*;

macro_rules! arithmetic {
    ($f:ident, $op:tt) => {
        pub const fn $f(top: &Pixel, bottom: &mut Pixel) {
            bottom[0] $op top[0];
            bottom[1] $op top[1];
            bottom[2] $op top[2];
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
    Dissolve,
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
    pub fn get_blender(&self) -> Box<dyn Blend> {
        Box::new(match self {
            Self::Normal => Normal::default(),
            #[cfg(feature = "dissolve")]
            Self::Dissolve => dissolve::Dissolve::default(),
            Self::Multiply => Multiply::default(),
            Self::Screen => Screen::default(),
            Self::Overlay => Overlay::default(),
            Self::HardLight => HardLight::default(),
            Self::SoftLight => SoftLight::default(),
            Self::Dodge => Dodge::default(),
            Self::Burn => Burn::default(),
            Self::VividLight => VividLight::default(),
            Self::Divide => Divide::default(),
            Self::Add => Add::default(),
            Self::Subtract => Subtract::default(),
            Self::Difference => Difference::default(),
            Self::DarkenOnly => DarkenOnly::default(),
            Self::LightenOnly => LightenOnly::default(),
        })
    }
}
