# Blittle

**`blittle` is a fast little blitter.**

```
use blittle::{*, stride::RGB};

// The dimensions and byte data of the source image.
let src_w = 32;
let src_h = 17;
// A raw image bitmap.re
let src = vec![0u8; src_w * src_h * RGB];
let src_size = Size { w: src_w, h: src_h };

// The dimensions and byte data of the destination image.
let dst_w = 64;
let dst_h = 64;
// Another raw image bitmap.
let mut dst = vec![0u8; dst_w * dst_h * RGB];
// The top-left position of where `src` will appear on `dst`.
let dst_position = Position { x: 2, y: 12 };
let dst_size = Size { w: dst_w, h: dst_h };

// Blit `src` onto `dst`.
blit(&src, &src_size, &mut dst, &dst_position, &dst_size, RGB);
```

## No mask? No mask!

Most blit functions assume that you might want to apply a mask.
A mask is typically a certain color.
Pixels in the source image that have the mask color aren't blitted to the destination image.

**`blittle` is fast because it doesn't apply a mask.**
Since `blittle` doesn't have to check each pixel's color, it can copy per-row, rather than per-pixel.

## Clipping

By default, `blittle` won't check whether your source image exceeds the bounds of the
destination image. This will cause your program to crash with a very opaque memory error.

To trim the source image's blittable region, call [`clip`].

## Benchmarks

Run `cargo bench` and find out.