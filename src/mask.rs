use crate::error::Error;
use crate::{RectU, Surface};
use glam::USizeVec2;

pub struct MaskedSurface<P: Copy + Clone + Sized + Default> {
    pub surface: Surface<P>,
    mask_color: P,
    masked_indices: Option<Vec<usize>>,
}

impl<P: Copy + Clone + Sized + Default + Eq + PartialEq> MaskedSurface<P> {
    pub const fn new(surface: Surface<P>, mask_color: P) -> Self {
        Self {
            surface,
            mask_color,
            masked_indices: None,
        }
    }

    pub const fn set_mask_color(&mut self, mask_color: P) -> Result<(), Error> {
        if self.is_locked() {
            Err(Error::Locked)
        } else {
            self.mask_color = mask_color;
            Ok(())
        }
    }

    pub fn lock(&mut self) {
        // Set new lock indices.
        if self.masked_indices.is_none() {
            self.masked_indices = Some(match self.surface.blit_area {
                // If there is a blit area, only check those pixels.
                Some(blit_area) => {
                    let mut indices = vec![];
                    // Iterate through the blit area.
                    for y in blit_area.position.y..blit_area.position.y + blit_area.size.y {
                        for x in blit_area.position.x..blit_area.position.x + blit_area.size.x {
                            let index = self.surface.get_index(x, y);
                            if self.surface.buffer[index] == self.mask_color {
                                indices.push(index);
                            }
                        }
                    }
                    indices
                }
                // Iterate through all pixels.
                None => self
                    .surface
                    .buffer
                    .iter()
                    .enumerate()
                    .filter_map(|(i, pixel)| {
                        if *pixel != self.mask_color {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect(),
            })
        }
    }

    pub const fn is_locked(&self) -> bool {
        self.masked_indices.is_some()
    }

    pub fn unlock(&mut self) {
        self.masked_indices = None;
    }

    /// Blit onto `other`, using a mask.
    ///
    /// Be sure to call [Surface::position] or [Surface::set_position]
    /// before blitting to a *new* `other` surface.
    pub fn blit(&self, other: &mut Surface<P>) -> Result<(), Error> {
        // Try to get the destination rect.
        let destination_rect = self
            .surface
            .destination_rect
            .ok_or(Error::NoDestinationRect)?;
        // Blit either a chunk of the source buffer, or all of it.
        let blit_area = match self.surface.blit_area {
            Some(rect) => rect,
            None => RectU {
                position: USizeVec2::ZERO,
                size: destination_rect.size,
            },
        };
        let dst_offset = other.get_index(destination_rect.position.x, destination_rect.position.y);
        match self.masked_indices.as_ref() {
            Some(mask_indices) => {
                mask_indices.iter().for_each(|i| {
                    let i = *i;
                    other.buffer[dst_offset + i] = self.surface.buffer[i];
                });
            }
            None => {
                // Iterate per-pixel.
                let len = blit_area.size.x * blit_area.size.y;
                let src_offset = self
                    .surface
                    .get_index(blit_area.position.x, blit_area.position.y);
                for i in 0..len {
                    let src_index = src_offset + i;
                    if self.surface.buffer[src_index] != self.mask_color {
                        other.buffer[dst_offset + i] = self.surface.buffer[src_index];
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(feature = "png")]
#[cfg(test)]
mod tests {
    use super::*;
    use glam::I64Vec2;
    use std::env::current_dir;

    const SRC_W: usize = 32;
    const SRC_H: usize = 17;
    const DST_W: usize = 64;
    const DST_H: usize = 64;

    #[test]
    fn test_blit_mask() {
        let position = I64Vec2 { x: 2, y: 12 };
        let src_size = USizeVec2 { x: SRC_W, y: SRC_H };
        let mut dst = Surface::new_filled(USizeVec2 { x: DST_W, y: DST_H }, [0u8, 0, 0]);

        let src_color = [255u8, 255, 255];
        let mask_color = [255, 0, 255];
        let mut src = Surface::new_filled(src_size, src_color)
            .position(position, &dst)
            .unwrap();
        for pixel in src.buffer.chunks_exact_mut(3) {
            pixel[0] = mask_color;
        }
        let mut src = MaskedSurface::new(src, mask_color);

        src.blit(&mut dst).unwrap();

        dst.write_png(current_dir().unwrap().join("test_output/mask_unlocked.png"))
            .unwrap();

        // Lock.
        src.lock();
        src.blit(&mut dst).unwrap();

        dst.write_png(current_dir().unwrap().join("test_output/mask_locked.png"))
            .unwrap();
    }
}
