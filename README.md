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

## Blitting with a mask

A mask is a color. Pixels from the source surface that have the mask color don't get copied onto the destination surface.

You can apply a mask color to a surface by using a `MaskedSurface`.

`MaskedSurface.blit` is usually slower than `Surface.blit` but it can be sped up by calling `MaskedSurface.lock()`.

## Blending

You can blend the pixels of two surfaces together using a `BlendableSurface`.

`BlendableSurface.blend` is usually slower than `Surface.blit` but it can be sped up by calling `BlendableSurface.lock()`.

## Converting surfaces

The `PixelConverter` trait can be used to convert one surface into another type of surface:

```
use blittle::{PixelConverter, Rgb8Surface, Rgba8Surface, Size};

let rgb = Rgb8Surface::new(Size::new(512, 512));
let rgba = Rgba8Surface::from(&rgb);
```

## Feature Flags

- `bytes` allows access to the underlying buffer of a `Surface` as bytes
- `jpg` adds the `Jpg` trait, which can be used to read and write .jpg files. See: `blittle::jpg`
- `png` adds the `Png` trait, which can be used to read and write .png files. See: `blittle::png`
- `serde` allows some structs such as `RectU` and `RectI` to be serializable
- `softbuffer` adds a new type of surface that can be created as a reference to a `softbuffer::Buffer`. See: `blittle::sb`
- `std` (default) allows std. 

## `no_std`

If `blittle` is `no_std`, you'll lose some functionality:

- No `Surface` type aliases such as `Rgba8Surface`
- No implementations for converting one type of surface to another
- Can't add the `png` or `softbuffer` features
- `MaskedSurface` and `BlendableSurface` can't lock/unlock

## Benchmarks

Run `cargo bench --all-features` and find out.