mod rect;
pub mod stride;

pub use rect::Rect;

pub fn blit(src: &[u8], dst: &mut [u8], rect: &Rect, stride: usize) {
    // let start_index = rect.get_start(stride);
    
}