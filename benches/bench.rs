use blittle::stride::RGB;
use blittle::{blit_to_buffer, blit_to_slices, get_dst_slices};
use bytemuck::{cast_slice, cast_slice_mut};
use criterion::{Criterion, criterion_group, criterion_main};

pub fn criterion_benchmark(c: &mut Criterion) {
    const SRC_W: usize = 32;
    const SRC_H: usize = 17;
    const DST_W: usize = 64;
    const DST_H: usize = 64;
    let src = [[255u8, 0, 0]; SRC_W * SRC_H];
    let mut dst = [[0u8, 0, 255]; DST_W * DST_H];

    let dst_x = 2;
    let dst_y = 12;

    let src = cast_slice::<[u8; RGB], u8>(&src);
    let mut dst = cast_slice_mut::<[u8; RGB], u8>(&mut dst);
    c.bench_function("blit_to_buffer", |b| {
        b.iter(|| blit_to_buffer(&src, &mut dst, dst_x, dst_y, DST_W, SRC_W, RGB))
    });

    let mut dst_slices = get_dst_slices(&mut dst, dst_x, dst_y, DST_W, SRC_W, SRC_H, RGB);

    c.bench_function("blit_to_buffer", |b| {
        b.iter(|| blit_to_slices(&src, &mut dst_slices, SRC_W, RGB))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
