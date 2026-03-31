pub trait Pixel<const NUM_CHANNELS: usize, T: Sized> {
    fn from_array(array: [T; NUM_CHANNELS]) -> Self;
}
