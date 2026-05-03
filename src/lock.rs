use crate::{RectU, Surface};

pub(crate) const fn get_indices<
    S: AsRef<[P]> + AsMut<[P]>,
    B: AsRef<[P]> + AsMut<[P]>,
    P: Copy + Clone + Sized + Default,
>(
    x: usize,
    y: usize,
    destination_rect: &RectU,
    blit_area: &RectU,
    src: &Surface<'_, S, P>,
    other: &Surface<'_, B, P>,
) -> (usize, usize) {
    // Get the start index in the source slice.
    let src_index = src.get_index(
        blit_area.position.x + x, // Blit area offset or zero
        y + blit_area.position.y, // y offset + blit area offset
    );
    let dst_index = other.get_index(
        destination_rect.position.x + x, // Destination position (x)
        y + destination_rect.position.y, // y offset + destination position (y)
    );
    (src_index, dst_index)
}

#[cfg(feature = "std")]
pub(crate) const fn get_dst_index<
    S: AsRef<[P]> + AsMut<[P]>,
    B: AsRef<[P]> + AsMut<[P]>,
    P: Copy + Clone + Sized + Default,
>(
    i: usize,
    destination_rect: &RectU,
    dst_len: usize,
    src: &Surface<'_, S, P>,
    other: &Surface<'_, B, P>,
) -> Option<usize> {
    let src_x = i % src.size.width;
    let src_y = i / src.size.width;
    let dst_index = other.get_index(
        destination_rect.position.x + src_x, // Destination position (x)
        src_y + destination_rect.position.y, // y offset + destination position (y)
    );
    if dst_index < dst_len {
        Some(dst_index)
    } else {
        None
    }
}
