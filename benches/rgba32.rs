use blittle::*;
use criterion::{Criterion, criterion_group, criterion_main};
use glam::USizeVec2;

const SRC_W: usize = 512;
const SRC_H: usize = 512;

macro_rules! get_rgba32 {
    ($c:ident, $surface:tt, $name:literal) => {
        let src = $surface::new(USizeVec2::new(SRC_W, SRC_H));
        $c.bench_function($name, |b| b.iter(|| Rgba32Surface::from(&src)));
    };
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let src = Rgb8Surface::new(USizeVec2::new(SRC_W, SRC_H));
    let mut dst = Rgba32Surface::new(USizeVec2::new(SRC_W, SRC_H));

    let mut group = c.benchmark_group("get rgba32");
    get_rgba32!(group, Rgb8Surface, "rgb8");
    get_rgba32!(group, Rgba8Surface, "rgba8");
    group.finish();

    let mut group = c.benchmark_group("set rgba32");
    group.bench_function("rgb8", |b| b.iter(|| src.set_rgba32(&mut dst, 1.)));
    let src = Rgba8Surface::new(USizeVec2::new(SRC_W, SRC_H));
    let mut dst = Rgba32Surface::new(USizeVec2::new(SRC_W, SRC_H));
    group.bench_function("rgba8", |b| b.iter(|| src.set_rgba32(&mut dst)));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
