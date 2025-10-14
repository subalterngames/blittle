# Blittle

**`blittle` is a fast little blitter.**

Most blit functions assume that you might want to apply a mask. A mask is typically a certain color. Pixels in the source image that have the mask color aren't blitted to the destination image.

**`blittle` is fast because it doesn't apply a mask.** Since `blittle` doesn't have to check each pixel's color, it can copy every row of the source image onto the destination image, rather than each pixel.

`blittle` is also fast because:

- You can optionally enable `rayon` for per-row copying
- You can precalculate the region of `dst` that `src` will be blitted to, and then reuse it (for example, during an animation)