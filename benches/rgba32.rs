use blittle::*;
use criterion::{Criterion, criterion_group, criterion_main};
use glam::{USizeVec2, Vec4};

const SRC_W: usize = 512;
const SRC_H: usize = 512;

pub fn criterion_benchmark(c: &mut Criterion) {
    let src = Rgb8Surface::new(USizeVec2::new(SRC_W, SRC_H));
    let mut dst = Rgba32Surface::new(USizeVec2::new(SRC_W, SRC_H));

    let mut group = c.benchmark_group("to rgba32");
    group.bench_function("rgb8", |b| b.iter(|| src.set_rgba32(&mut dst, 1.)));
    let mut src = Rgba8Surface::new_filled(USizeVec2::new(SRC_W, SRC_H), [0, 0, 0, 255]);
    group.bench_function("opaque rgba8", |b| b.iter(|| src.set_rgba32(&mut dst)));
    src.fill([0; 4]);
    group.bench_function("transparent rgba8", |b| b.iter(|| src.set_rgba32(&mut dst)));
    group.finish();

    from_rgba32(c, Vec4::ONE, "from opaque rgba32");
    from_rgba32(c, Vec4::ZERO, "from transparent rgba32");
}

fn from_rgba32(c: &mut Criterion, color: Vec4, group_name: &str) {
    let src = Rgba32Surface::new_filled(USizeVec2::new(SRC_W, SRC_H), color);
    let mut dst = Rgb8Surface::new(USizeVec2::new(SRC_W, SRC_H));
    let mut group = c.benchmark_group(group_name);
    group.bench_function("rgb8", |b| b.iter(|| src.set_rgb8(&mut dst)));
    let mut dst = Rgba8Surface::new(USizeVec2::new(SRC_W, SRC_H));
    group.bench_function("rgba8", |b| b.iter(|| src.set_rgba8(&mut dst)));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
