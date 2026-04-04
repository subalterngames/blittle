use crate::PixelType;
use crate::error::Error;
use crate::rect::{RectI, RectU};
use glam::{I64Vec2, USizeVec2};
use num_traits::Zero;

pub struct Surface<P: Copy + Clone + Sized + Zero> {
    rect: RectI,
    buffer: Vec<P>,
    destination_rect: Option<RectU>,
    blit_area: Option<RectU>,
}

impl<P: Copy + Clone + Sized + Zero> Surface<P> {
    pub fn new(size: USizeVec2) -> Self {
        Self {
            rect: RectI::from_size(size),
            buffer: vec![P::zero(); size.x * size.y],
            destination_rect: None,
            blit_area: None,
        }
    }

    pub fn new_filled(size: USizeVec2, color: P) -> Self {
        Self {
            rect: RectI::from_size(size),
            buffer: vec![color; size.x * size.y],
            destination_rect: None,
            blit_area: None,
        }
    }

    pub fn filled(mut self, color: P) -> Self {
        self.fill(color);
        self
    }

    pub fn fill(&mut self, color: P) {
        self.buffer.fill(color);
    }

    pub const fn position(mut self, position: I64Vec2) -> Self {
        self.set_position(position);
        self
    }

    pub const fn set_position(&mut self, position: I64Vec2) {
        self.rect.position = position;
    }

    pub const fn get_rect(&self) -> RectI {
        self.rect
    }

    pub const fn destination(mut self, destination: &Self) -> Self {
        let _ = self.set_destination(destination);
        self
    }

    pub const fn set_destination(&mut self, destination: &Self) -> Option<RectU> {
        self.destination_rect = self.rect.clip(destination.rect);
        self.destination_rect
    }

    pub const fn set_area(&mut self, area: RectI) -> Result<RectU, Error> {
        match area.clip(self.rect) {
            Some(area) => {
                self.blit_area = Some(area);
                Ok(area)
            }
            None => Err(Error::InvalidArea(area)),
        }
    }

    pub const fn get_index(x: usize, y: usize, w: usize) -> usize {
        x + y * w
    }

    pub fn blit(&self, other: &mut Self) -> Result<(), Error> {
        // Try to get the destination rect.
        let destination_rect = self.destination_rect.ok_or(Error::NoDestinationRect)?;

        // Blit either a chunk of the source buffer, or all of it.
        let blit_area = match self.blit_area {
            Some(rect) => rect,
            None => RectU {
                position: USizeVec2::ZERO,
                size: self.rect.size,
            },
        };

        // Iterate per-row.
        (0..blit_area.size.y).for_each(|src_y| {
            // Get the start index in the source slice.
            let src_index = Self::get_index(
                blit_area.position.x,         // Blit area offset or zer
                src_y + blit_area.position.y, // y offset + blit area offset
                self.rect.size.x,             // The actual width of the source buffer
            );
            let dst_index = Self::get_index(
                destination_rect.position.x,         // Destination position (x)
                src_y + destination_rect.position.y, // y offset + destination position (y)
                other.rect.size.x,                   // The actual width of the destination buffer
            );
            // Copy the slice, using the blit area's width.
            other.buffer[dst_index..dst_index + blit_area.size.x]
                .copy_from_slice(&self.buffer[src_index..src_index + blit_area.size.x]);
        });
        Ok(())
    }
}
