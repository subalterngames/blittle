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
    pub(crate) const fn get_line_indices(&self, stride: usize) -> (usize, usize) {
        let start = (self.x + self.y * self.w) * stride;
        let end = start + self.w * stride;
        (start, end)
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
    use crate::stride::*;
    
    #[test]
    fn test_rect_start() {
        
        let mut r = Rect {
            x: 0,
            y: 0,
            w: 64,
            h: 32
        };
        let (start, end) = r.get_line_indices(GRAYSCALE);
        assert_eq!(start, 0);
        assert_eq!(end, r.w);
        let (start, end) = r.get_line_indices(RGB);
        assert_eq!(start, 0);
        assert_eq!(end, r.w * 3);

        r.x = 1;
        let (start, end) = r.get_line_indices(GRAYSCALE);
        assert_eq!(start, 1);
        assert_eq!(end, r.w + 1);
        let (start, end) = r.get_line_indices(RGB);
        assert_eq!(start, 3);
        assert_eq!(end, (r.w + 1) * 3);
        

        r.x = 3;
        r.y = 2;
        let (start, end) = r.get_line_indices(GRAYSCALE);
        assert_eq!(start, 131);
        assert_eq!(end, 131 + r.w);
        let (start, end) = r.get_line_indices(RGB);
        assert_eq!(start, 393);
        assert_eq!(end, 393 + r.w * 3);
    }
}