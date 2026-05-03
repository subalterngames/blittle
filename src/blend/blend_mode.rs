/// Standard color blend modes.
///
/// Source for most of the math: <https://en.wikipedia.org/wiki/Blend_modes>
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    HardLight,
    SoftLight,
    Dodge,
    Burn,
    VividLight,
    Divide,
    Add,
    Subtract,
    Difference,
    DarkenOnly,
    LightenOnly,
}
