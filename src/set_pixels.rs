use crate::{Rgb8Surface, Rgba8Surface, Rgba32Surface};

impl Rgb8Surface {
    /// Copy data into `other`, converting pixel values.
    ///
    /// `alpha` is the alpha value for the entire surface (0-1)
    pub fn set_rgba32(&self, other: &mut Rgba32Surface, alpha: f32) {
        if alpha <= 0. {
            return;
        }
        self.buffer
            .iter()
            .zip(other.buffer.iter_mut())
            .for_each(|(src, dst)| {
                *dst = Self::pixel_and_alpha_to_rgba32(src, alpha);
            });
        other.size = self.size;
        other.destination_rect = self.destination_rect;
        other.blit_area = self.blit_area;
    }
}

macro_rules! floats_to_bytes {
    ($self:ident, $set:ident, $pixel:ident, $dest:tt) => {
        /// Copy data into `other`, converting pixel values.
        pub fn $set(&$self, other: &mut $dest) {
            $self.buffer
                .iter()
                .zip(other.buffer.iter_mut())
                .for_each(|(src, dst)| {
                    *dst = Self::$pixel(src);
                });
            other.size = $self.size;
            other.destination_rect = $self.destination_rect;
            other.blit_area = $self.blit_area;
        }
    };
}

impl Rgba32Surface {
    floats_to_bytes!(self, set_rgb8, pixel_to_rgb8, Rgb8Surface);

    floats_to_bytes!(self, set_rgba8, pixel_to_rgba8, Rgba8Surface);
}
