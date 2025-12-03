use blit::{Blit, BlitBuffer, BlitOptions, geom::Size};
use blittle::{stride::*, *};
use criterion::{Criterion, criterion_group, criterion_main};
use sdl2::{
    pixels::{Color, PixelFormatEnum},
    surface::Surface,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    const SRC_W: usize = 512;
    const SRC_H: usize = 512;
    const DST_W: usize = 1920;
    const DST_H: usize = 1080;
    let src = vec![255u8; SRC_W * SRC_H * RGBA];
    let mut dst = vec![0u8; DST_W * DST_H * RGBA];

    // Single thread.
    let dst_position = PositionI { x: 2, y: 12 };
    let dst_size = blittle::Size { w: DST_W, h: DST_H };
    let src_size = blittle::Size { w: SRC_W, h: SRC_H };
    let rect = ClippedRect::new(dst_position, dst_size, src_size).unwrap();
    c.bench_function("blittle", |b| {
        b.iter(|| blit(&src, &mut dst, &rect, RGBA))
    });

    // Multi-thread.
    #[cfg(feature = "rayon")]
    {
        let num_threads = 16.max(rayon::max_num_threads());
        c.bench_function("blittle multi-threaded", |b| {
            b.iter(|| {
                blit_multi_threaded(
                    &src,
                    &mut dst,
                    &rect,
                    RGBA,
                    num_threads,
                )
            })
        });
    }

    // `blit` crate.
    let mut dst_u32 = vec![0u32; DST_W * DST_H];
    let src_u32 = vec![255u32; SRC_W * SRC_H];
    let blit_buffer = BlitBuffer::from_buffer(&src_u32, SRC_W, 0);
    let position = BlitOptions::new_position(dst_position.x, dst_position.y);
    let size = Size {
        width: DST_W as u32,
        height: DST_H as u32,
    };
    c.bench_function("blit (crate)", |b| {
        b.iter(|| blit_buffer.blit(&mut dst_u32, size, &position))
    });

    // SDL2
    let mut dst_surface =
        Surface::new(DST_W as u32, DST_H as u32, PixelFormatEnum::RGBA32).unwrap();
    let mut src_surface =
        Surface::new(SRC_W as u32, SRC_H as u32, PixelFormatEnum::RGBA32).unwrap();
    let src_rect = src_surface.rect();
    src_surface.fill_rect(src_rect, Color::BLUE).unwrap();
    c.bench_function("SDL2", |b| {
        b.iter(|| src_surface.blit(src_rect, &mut dst_surface, None))
    });

    // Overlay.
    #[cfg(feature = "overlay")]
    {
        let src = vec![[100, 200, 80]; SRC_W * SRC_H * RGB];
        let src = bytemuck::cast_slice::<[u8; 3], u8>(&src);
        let mut dst = vec![overlay::Vec4::default(); SRC_W * SRC_H];
        c.bench_function("rgba8 to rgba32", |b| {
            b.iter(|| overlay::rgb8_to_rgba32_in_place(&src, &mut dst))
        });

        let dst = vec![[19, 100, 234, 190]; DST_W * DST_H * RGBA];
        let src = overlay::rgba8_to_rgba32(&src);
        let mut dst = overlay::rgba8_to_rgba32(bytemuck::cast_slice::<[u8; 4], u8>(&dst));

        c.bench_function("overlay", |b| {
            b.iter(|| overlay::overlay_rgba32(&src, &mut dst, &rect));
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
