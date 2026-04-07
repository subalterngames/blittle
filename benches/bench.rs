use blit::{Blit, BlitBuffer, BlitOptions, geom::Size};
use blittle::*;
use criterion::{Criterion, criterion_group, criterion_main};
use glam::{I64Vec2, USizeVec2};
use sdl2::{
    pixels::{Color, PixelFormatEnum},
    surface::Surface,
};

const SRC_W: usize = 512;
const SRC_H: usize = 512;

macro_rules! get_rgba32 {
    ($c:ident, $surface:tt, $name:literal) => {
        let src = $surface::new(USizeVec2::new(SRC_W, SRC_H));
        $c.bench_function(concat!($name, ": get_rgba32"), |b| {
            b.iter(|| Rgba32Surface::from(&src))
        });
    };
}

pub fn criterion_benchmark(c: &mut Criterion) {
    const DST_W: usize = 1920;
    const DST_H: usize = 1080;
    const SRC_LEN: usize = SRC_W * SRC_H;
    const SRC_COLOR: [u8; 4] = [0; 4];

    let position = I64Vec2::new(2, 12);
    let mut dst = Rgba8Surface::new_filled(USizeVec2::new(DST_W, DST_H), [255; 4]);
    let src = Rgba8Surface::new_filled(USizeVec2::new(SRC_W, SRC_H), SRC_COLOR)
        .position(position, &dst)
        .unwrap();
    c.bench_function("blittle", |b| b.iter(|| src.blit(&mut dst)));

    // `blit` crate.
    let mut dst_u32 = vec![0u32; DST_W * DST_H];
    let src_u32 = vec![255u32; SRC_LEN];
    let blit_buffer = BlitBuffer::from_buffer(&src_u32, SRC_W, 0);
    let position = BlitOptions::new_position(position.x, position.y);
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

    let mut src = MaskedSurface::new(src, SRC_COLOR);
    c.bench_function("blittle mask (unlocked)", |b| b.iter(|| src.blit(&mut dst)));
    src.lock();
    c.bench_function("blittle mask (locked)", |b| b.iter(|| src.blit(&mut dst)));

    get_rgba32!(c, Rgb8Surface, "rgb8");
    let src = Rgb8Surface::new(USizeVec2::new(SRC_W, SRC_H));
    let mut dst = Rgba32Surface::new(USizeVec2::new(SRC_W, SRC_H));
    c.bench_function("rgb8: set_rgba32", |b| {
        b.iter(|| src.set_rgba32(&mut dst, 1.))
    });

    get_rgba32!(c, Rgba8Surface, "rgba8");
    let src = Rgba8Surface::new(USizeVec2::new(SRC_W, SRC_H));
    let mut dst = Rgba32Surface::new(USizeVec2::new(SRC_W, SRC_H));
    c.bench_function("rgba8: set_rgba32", |b| b.iter(|| src.set_rgba32(&mut dst)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
