use blit::{Blit, BlitBuffer, BlitOptions, geom::Size};
use blittle::{blit_to_buffer, blit_to_slices, get_dst_slices, stride::RGBA};
use bytemuck::{cast_slice, cast_slice_mut};
use criterion::{Criterion, criterion_group, criterion_main};
use sdl2::{
    pixels::{Color, PixelFormatEnum},
    surface::Surface,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    const SRC_W: usize = 32;
    const SRC_H: usize = 17;
    const DST_W: usize = 64;
    const DST_H: usize = 64;
    let src_map = [[255u8, 0, 0, 0]; SRC_W * SRC_H];
    let mut dst_map = [[0u8, 0, 255, 0]; DST_W * DST_H];

    let dst_x = 2;
    let dst_y = 12;

    let src = cast_slice::<[u8; RGBA], u8>(&src_map);
    let mut dst = cast_slice_mut::<[u8; RGBA], u8>(&mut dst_map);
    c.bench_function("blittle_buffer", |b| {
        b.iter(|| blit_to_buffer(&src, &mut dst, dst_x, dst_y, DST_W, SRC_W, RGBA))
    });

    let mut dst_slices = get_dst_slices(&mut dst, dst_x, dst_y, DST_W, SRC_W, SRC_H, RGBA);
    c.bench_function("blittle_slices", |b| {
        b.iter(|| blit_to_slices(&src, &mut dst_slices, SRC_W, RGBA))
    });

    // `blit` crate.
    let mut dst_buffer = [0u32; DST_W * DST_H];
    let src_u32 = cast_slice::<[u8; 4], u32>(&src_map);
    let blit_buffer = BlitBuffer::from_buffer(src_u32, SRC_W, 255);
    let position = BlitOptions::new_position(dst_x, dst_y);
    let size = Size {
        width: DST_W as u32,
        height: DST_H as u32,
    };
    c.bench_function("blit", |b| {
        b.iter(|| blit_buffer.blit(&mut dst_buffer, size, &position))
    });

    // SDL3
    let mut dst = Surface::new(DST_W as u32, DST_H as u32, PixelFormatEnum::RGBA32).unwrap();
    let mut src = Surface::new(SRC_W as u32, SRC_H as u32, PixelFormatEnum::RGBA32).unwrap();
    let src_rect = src.rect();
    src.fill_rect(src_rect, Color::BLUE).unwrap();
    c.bench_function("sdl2", |b| b.iter(|| src.blit(src_rect, &mut dst, None)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
