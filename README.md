# Blittle

**`blittle` is a fast little blitter.**

```
use blittle::*;
use glam::{I64Vec2, USizeVec2};

// The size of the source image.
let size = USizeVec2::new(512, 512);
// Create a new surface. 
// This surface has three channels (r, g, b) and 1 byte per channel.
let mut src = Rgb8Surface::new(size);
// Fill the surface.
src.fill([255, 0, 255]);
// Create a destination surface.
let mut dst = Rgb8Surface::new_filled(USizeVec2::new(1920, 1080), [100, 100, 50]);
// Set the position of src relative to dst.
src.set_position(I64Vec2::new(-50, 100), &dst).unwrap();
// Blit src onto dst.
src.blit(&mut dst).unwrap();
```

The above example is *very* fast because there is no mask or blending involved.

A mask is a certain color. Pixels in the source image that have the mask color aren't blitted to the destination image. If there was a mask, then `src.blit(dst)` would have to evaluate every pixel. But, because there isn't a mask, `blittle` can copy *each row* of `src` onto `dst` rather than each pixel.

## No mask? No mask!

Most blit functions assume that you might want to apply a mask.
A mask is typically a certain color.

**`blittle` is fast because it doesn't apply a mask.**
Since `blittle` doesn't have to check each pixel's color, it can copy per-row, rather than per-pixel.

## Clipping

By default, `blittle` won't check whether your source image exceeds the bounds of the
destination image. This will cause your program to crash with a very opaque memory error.

To trim the source image's blittable region, call [`clip`].

## Feature Flags

### The `overlay` feature

Add the `overlay` feature to include functions for overlaying a source image onto a destination with alpha (transparency) value(s).

The overlaying functions are always slower than `blittle::blit`. `blittle::blit` copies lines of bytes, while overlaying involves per-pixel calculations.

### The `rayon` feature

Add the `rayon` feature to enable multithreaded blitting:

`blit_multi_threaded` breaks the source and destination images into multiple chunks and then blits each chunk in parallel. The function signature is the same as that of [`blit`] except that there's an additional `num_threads` argument.

### The `serde` feature

Add the `serde` feature to make `PositionI`, `PositionU`, and `Size` serializable.

## Benchmarks

Run `cargo bench --all-features` and find out.