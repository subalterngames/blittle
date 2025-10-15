/// Rectangular bounds defined by a width and height.
#[derive(Copy, Clone, Default)]
pub struct Size {
    pub w: usize,
    pub h: usize,
}

impl Size {
    pub const fn new(buffer: &[u8], w: usize, stride: usize) -> Self {
        let h = (buffer.len() / stride) / w;
        Self { w, h }
    }
}
