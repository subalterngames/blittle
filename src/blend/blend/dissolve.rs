use super::{Blend, Pixel};
use rand::prelude::ThreadRng;
use rand::{RngExt, rng};

pub struct Dissolve(ThreadRng);

impl Default for Dissolve {
    fn default() -> Self {
        Self(rng())
    }
}

impl Blend for Dissolve {
    fn blend_mode(&mut self, top: &Pixel, bottom: &mut Pixel) {
        if self.0.random_bool(0.5) {
            *bottom = *top;
        }
    }
}
