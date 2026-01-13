use std::fmt::{Display, Formatter};

/// Rectangular bounds defined by a width and height.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.w, self.h)
    }
}
