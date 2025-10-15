/// Rectangular bounds defined by a width and height.
#[derive(Copy, Clone)]
pub struct Size {
    pub w: usize,
    pub h: usize,
}

impl Size {
    pub fn new(buffer: &[u8], w: usize, stride: usize) -> Self {
        let h = (buffer.len() / stride) / w;
        Self { w, h }
    }

    /// Clip the width and height to be within the bounds of `other`.
    pub fn clip(&mut self, other: &Self) {
        self.w = self.w.min(other.w);
        self.h = self.h.min(other.h);
    }
}
