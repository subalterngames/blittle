# Blittle

**`blittle` is a fast little blitter.**

`blittle` blits *surfaces* onto each other. 
A surface is a pixel buffer with some additional data such as its pixel type and dimensions.

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

If you *do* want to use a mask, you can use a `MaskedSurface`

## Converting surfaces

The `PixelConverter` trait can be used to convert one surface into another type of surface:

```
use blittle::{PixelConverter, Rgb8Surface, Rgba8Surface};
use glam::USizeVec2;

let rgb = Rgb8Surface::new(USizeVec2::new(512, 512));
let rgba = Rgba8Surface::from(&rgb);
```

## Feature Flags

- `png` adds the `Png` trait, which can be used to read and write .png files. See: `blittle::png`
- `serde` allows all `glam` structs (e.g. `USizeVec2`), `blittle::RectU`, and `blittle::RectI` to be serializable.
- `softbuffer` adds a new type of surface that can be created as a reference to a `softbuffer::Buffer`. See: `blittle::sb`

## Benchmarks

Run `cargo bench --all-features` and find out.