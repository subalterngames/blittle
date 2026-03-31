use crate::PixelType;
use crate::rect::Rect;
use glam::USizeVec2;
use num_traits::Zero;

pub struct Surface<const CHANNELS: usize, T: Copy + Clone + Sized + Zero> {
    rect: Rect,
    buffer: Vec<[T; CHANNELS]>,
    destination_rect: Option<Rect>,
    blit_area: Option<Rect>,
}

impl<const CHANNELS: usize, T: Copy + Clone + Sized + Zero> Surface<CHANNELS, T> {
    const BYTES_PER_CHANNEL: usize = size_of::<T>();

    pub fn new(size: USizeVec2) -> Self {
        Self {
            rect: Rect::from_size(size),
            buffer: vec![[T::zero(); CHANNELS]; size.x * size.y],
            destination_rect: None,
            blit_area: None,
        }
    }

    pub fn new_filled(size: USizeVec2, color: [T; CHANNELS]) -> Self {
        Self {
            rect: Rect::from_size(size),
            buffer: vec![color; size.x * size.y],
            destination_rect: None,
            blit_area: None,
        }
    }

    pub fn filled(mut self, color: [T; CHANNELS]) -> Self {
        self.fill(color);
        self
    }

    pub fn fill(&mut self, color: [T; CHANNELS]) {
        self.buffer.fill(color);
    }

    pub const fn position(mut self, position: USizeVec2) -> Self {
        self.set_position(position);
        self
    }

    pub const fn set_position(&mut self, position: USizeVec2) {
        self.rect.position = position;
    }

    pub const fn get_rect(&self) -> Rect {
        self.rect
    }
    
    pub const fn destination(mut self, destination: &Self) -> Self {
        let _ = self.set_destination(destination);
        self
    }

    pub const fn set_destination(&mut self, destination: &Self) -> Option<Rect> {
        // Out of bounds or bad size.
        if self.rect.position.x >= destination.rect.size.x
            || self.rect.position.y >= destination.rect.size.y
            || self.rect.size.x == 0
            || self.rect.size.y == 0
        {
            None
        }
        else {
            let mut destination_rect = self.rect;
            if destination_rect.position.x + destination_rect.size.x >= destination.rect.size.x {
                destination_rect.size.x = destination.rect.size.x;
            }
            if destination_rect.position.x + destination_rect.size.x >= destination.rect.size.y {
                destination_rect.size.y = destination.rect.size.y;
            }
            self.destination_rect = Some(destination_rect);
            self.destination_rect
        }
    }
    
    pub const fn set_area(&mut self, area: Rect) -> Option<Rect> {
        if area.size.x == 0 || area.size.y == 0 {
            None
        }
        else {
             match &mut self.destination_rect {
                Some(destination_rect) => {
                    if area.overlaps(&self.rect.zeroed_position()) {
                        let mut area = area;
                        // Clip the size.
                        area.size.x = area.size.x.m
                        Some(area)
                    }
                    else {
                        // No overlap.
                        None
                    }
                }
                None => None
            }
        }
    }
}
