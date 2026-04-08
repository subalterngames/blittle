use blittle::*;
use criterion::{Criterion, criterion_group, criterion_main};
use glam::{I64Vec2, USizeVec2};

const SRC_W: usize = 512;
const SRC_H: usize = 512;
const DST_W: usize = 1920;
const DST_H: usize = 1080;
const SRC_COLOR: [u8; 4] = [0; 4];

pub fn criterion_benchmark(c: &mut Criterion) {
    let position = I64Vec2::new(2, 12);
    let mut dst = Rgba8Surface::new_filled(USizeVec2::new(DST_W, DST_H), [255; 4]);
    let src = Rgba8Surface::new_filled(USizeVec2::new(SRC_W, SRC_H), SRC_COLOR)
        .position(position, &dst)
        .unwrap();

    let mut src = MaskedSurface::new(src, [255, 0, 0, 0]);
    let mut group = c.benchmark_group("mask");
    group.bench_function("unlocked", |b| b.iter(|| src.blit(&mut dst)));
    src.lock();
    group.bench_function("locked", |b| b.iter(|| src.blit(&mut dst)));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
