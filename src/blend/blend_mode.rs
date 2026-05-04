/// Color blend modes.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BlendMode {
    /// Interpolate between the pixels.
    Normal,
    /// Multiply the pixels.
    Multiply,
    /// Multiply the inverse of the pixels, then return the inverse of the result.
    Screen,
    /// Multiply or screen depending on which pixel is lighter.
    Overlay,
    /// The opposite of overlay.
    HardLight,
    /// It's like overlay, sort of.
    /// <https://en.wikipedia.org/wiki/Blend_modes#Soft_Light>
    SoftLight,
    /// Divide the bottom pixel by the inverted top pixel.
    Dodge,
    /// Divide the bottom pixel by the inverted top pixel.
    Burn,
    /// Dodge or blend depending on which pixel is lighter.
    VividLight,
    /// Divide the bottom pixel by the top pixel.
    Divide,
    /// Add the two pixels.
    Add,
    /// Subtract the top pixel from the bottom pixel.
    Subtract,
    /// The absolute value of `Subtract`.
    Difference,
    /// Each color channel is set to the darker of the corresponding channels of the two pixels.
    DarkenOnly,
    /// Each color channel is set to the lighter of the corresponding channels of the two pixels.
    LightenOnly,
}
