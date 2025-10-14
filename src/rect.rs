pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

impl Rect {
    pub const fn get_start(&self, stride: usize) -> usize {
        (self.x + self.y * self.w) * stride
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