# Blittle

**`blittle` is a fast little blitter.**

`blittle` blits *surfaces* onto each other. 
A surface is a pixel buffer with some additional data such as its pixel type and dimensions.

```
use blittle::*;

// The size of the source image.
let size = Size::new(512, 512);
// Create a new surface. 
// This surface has three channels (r, g, b) and 1 byte per channel.
let mut src = Rgb8Surface::new(size);
// Fill the surface.
src.fill([255, 0, 255]);
// Create a destination surface.
let mut dst = Rgb8Surface::new_filled(Size::new(1920, 1080), [100, 100, 50]);
// Set the position of src relative to dst.
src.set_position(PositionI::new(-50, 100), &dst).unwrap();
// Blit src onto dst.
src.blit(&mut dst).unwrap();
```

The above example is *very* fast because there is no mask or blending involved.

A mask is a certain color. Pixels in the source image that have the mask color aren't blitted to the destination image. If there was a mask, then `src.blit(dst)` would have to evaluate every pixel. But, because there isn't a mask, `blittle` can copy *each row* of `src` onto `dst` rather than each pixel.

If you *do* want to use a mask, you can use a `MaskedSurface`

## Converting surfaces

The `PixelConverter` trait can be used to convert one surface into another type of surface:

```
use blittle::{PixelConverter, Rgb8Surface, Rgba8Surface, Size};

let rgb = Rgb8Surface::new(Size::new(512, 512));
let rgba = Rgba8Surface::from(&rgb);
```

## Feature Flags

- `bytes` allows access to the underlying buffer of a `Surface` as bytes
- `png` adds the `Png` trait, which can be used to read and write .png files. See: `blittle::png`
- `serde` allows some structs such as `RectU` and `RectI` to be serializable
- `softbuffer` adds a new type of surface that can be created as a reference to a `softbuffer::Buffer`. See: `blittle::sb`
- `std` (default) allows std. 

## `no_std`

If `blittle` is `no_std`, you'll lose some functionality:

- No type aliases for `Surface` (they are all backed by vecs)
- No `MaskedSurface`
- No `PixelConverter`
- Can't add anything from the `png` or `softbuffer` features

## Benchmarks

Run `cargo bench --all-features` and find out.