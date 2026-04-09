use blittle::*;
use criterion::{Criterion, criterion_group, criterion_main};
use glam::USizeVec2;

const SRC_W: usize = 512;
const SRC_H: usize = 512;

pub fn criterion_benchmark(c: &mut Criterion) {
    let src = Zrgb8Surface::new(USizeVec2::new(SRC_W, SRC_H));
    let mut group = c.benchmark_group("zrgb8");
    group.bench_function("rgba", |b| {
        b.iter(|| {
            let _ = Rgba8Surface::from(&src);
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
