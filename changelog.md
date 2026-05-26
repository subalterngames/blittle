# Changelog

## 0.7.4

- Added `jpg` feature for jpg reading/writing

## 0.7.3

- Fixed: `Surface::set_area` doesn't work when it should due to bad clipping logic
- Removed: `Surface::new_from_slice` 
- Added: `Surface::new_ref_from_bitmap` Similar to `new_from_slice` but accepts raw bytes instead of raw pixels
- Added: `Surface::new_from_bitmap` for vec-backed surfaces

## 0.7.2

- Renamed a function to eliminate ambiguity

## 0.7.1

- Exposed all blend functions

## 0.7.0

- Added color blending via `BlendableSurface` and `BlendMode`.
- Fixed a bug that caused `MaskedSurface` to basically always blit incorrectly. 

## 0.6.2

`std` is a default feature.

## 0.6.1

Added `no_std` support.

## 0.6.0

Removed `glam`. It just isn't as useful as what I'd hoped for.

- `RectI::position` is now of type `PositionI`
- `RectU::position` is now of type `PositionU`
- `Surface::size` is now of type `Size`

Also: Removed `bytemuck` as a required dependency and added a `bytes` feature.

## 0.5.1

Fixed the implausibly-fast converter benchmarks.

## 0.5.0

Big, sweeping changes.

- `blittle` now uses *surfaces* that can blit to each other. This eliminates a lot of bugs involved in using raw byte buffers.
A surface contains a pixel buffer of a given pixel type, a size, and some other underlying cached data.
- `blittle` uses `I64Vec2` instead of `PositionI` and `USizeVec2` instead of `PositionU`.
- The `overlay` functions are now gone. Now, there are comprehensive conversions to and from pixel types, but no option to overlay pixels.
- Added a `MaskedSurface` that includes a blit mask color.
- Added `Png` trait to read/write .png files.
- Added a `softbuffer` feature that allows `blittle` to work with the `softbuffer` crate.
- Updated benchmarks and tests.

## 0.4.4

- Improved color conversion performance by 40-60%
    - All u8 -> f32 conversions, e.g. `rgba8_to_rgba32`, use a look-up table of pre-calculated float values
    - All f32 -> u8 conversions, e.g. `rgba32_to_rgba8`, exclude unnecessary `ceil()` calls, and deref Vec4s exactly once per pixel
- Improved accuracy of benchmarks
- Bumped dependency versions

## 0.4.3

- Fixed `ClippedRect::set_src_rect()` to handle clipping properly.

## 0.4.2

- Derive `Eq` and `PartialEq` for `Size`.

## 0.4.1

- Fixed: Clipped images start to blit at the top-left corner.

## 0.4.0

- Removed `bytemuck` feature from `glam`
- Removed `fill()` function. It's slow and easy to implement ad-hoc.
- Replaced `blittle::stride::*` with `blittle::PixelType`
- Added: `ClippedRect::set_src_rect()` to set an area within the source bitmap to blit

## 0.3.3

- Added `bytemuck` feature to `glam`

## 0.3.2

- Oops, `ClippedRect::overlaps(other)` was wrong.

## 0.3.1

- Improved `fill(buffer, color)` performance.
- Added: `ClippedRect::overlaps(other)`.

## 0.3.0

- Added `overlay` feature which adds many new functions for overlaying pixels using an alpha transparency channel.
- Replaced the `clip()` function with a `ClippedRect` struct.
- Changed the parameters of `blit()` so that it now accepts a `ClippedRect`.
- Fixed a major bug in which pixel indices were calculated using the clipped size rather than the true size (`ClippedRect` stores both).
- Updated benchmarks.
- Added `From<PositionU>` for `PositionI` and `From<PositionI>` for `PositionU`
- Added a `fill()` function.

## 0.2.3

- Implemented `Debug` and `Display` for `PositionI`, `PositionU`, and `Size`.

## 0.2.2

- Added optional `serde` feature to derive de(serialize) traits for `PositionI`, `PositionU`, and `Size`.

## 0.2.1

- Fixed a bad cast from usize to isize

## 0.2.0

- Added optional `rayon` feature that adds a new `blit_multi_threaded` function.
- Updated benchmarks.