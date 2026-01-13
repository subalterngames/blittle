# Changelog

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