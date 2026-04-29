pub enum LockedIndices {
    /// The index of a pixel.
    Pixel(usize),
    /// The start and end of a row of pixels.
    Row { start: usize, end: usize },
}
