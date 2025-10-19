use blit::{Blit, BlitBuffer, BlitOptions, geom::Size};
use blittle::{PositionU, blit, stride::RGBA};
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
    let dst_position = PositionU { x: 2, y: 12 };
    let dst_size = blittle::Size { w: DST_W, h: DST_H };
    let src_size = blittle::Size { w: SRC_W, h: SRC_H };
    c.bench_function("blittle", |b| {
        b.iter(|| blit(&src, &src_size, &mut dst, &dst_position, &dst_size, RGBA))
    });

    // Multi-thread.
    

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
    let mut dst = Surface::new(DST_W as u32, DST_H as u32, PixelFormatEnum::RGBA32).unwrap();
    let mut src = Surface::new(SRC_W as u32, SRC_H as u32, PixelFormatEnum::RGBA32).unwrap();
    let src_rect = src.rect();
    src.fill_rect(src_rect, Color::BLUE).unwrap();
    c.bench_function("SDL2", |b| b.iter(|| src.blit(src_rect, &mut dst, None)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
