use std::fmt::{Display, Formatter};

/// A destination rect.
/// `(x, y)` is the top-left position of the `Rect`.
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

impl Rect {
    /// Converts `(x, y)` to a start index.
    /// `stride` is the number of bytes per pixel.
    /// For example, an RGB24 pixel (3 channels, 1 byte per channel) has a stride of 3.
    /// See `crate::strides` for some stride constants.
    pub const fn get_start(&self, stride: usize) -> usize {
        (self.x + self.y * self.w) * stride
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.w, self.h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rect_start() {
        const RGB: usize = 3;
        const L: usize = 1;
        
        let mut r = Rect {
            x: 0,
            y: 0,
            w: 64,
            h: 32
        };
        assert_eq!(r.get_start(RGB), 0);
        assert_eq!(r.get_start(L), 0);
        r.x = 1;
        assert_eq!(r.get_start(RGB), 3);
        assert_eq!(r.get_start(L), 1);
        r.x = 3;
        r.y = 2;
        assert_eq!(r.get_start(RGB), 393);
        assert_eq!(r.get_start(L), 131);
    }
}