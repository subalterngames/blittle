# Changelog

## 0.3.0

- Added `overlay` feature which adds many new functions for overlaying pixels using an alpha transparency channel.
- Replaced the `clip()` function with a `ClippedRect` struct.
- Changed the parameters of `blit()` so that it now accepts a `ClippedRect`.
- Updated benchmarks.
- Added `From<PositionU>` for `PositionI` and `From<PositionI>` for `PositionU`
- Added a `fill()` function.