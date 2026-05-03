use blittle::png::Png;
use blittle::*;
use criterion::{Criterion, criterion_group, criterion_main};
use std::io::Cursor;
use std::time::Instant;

pub fn criterion_benchmark(c: &mut Criterion) {
    let dst = Rgba32Surface::from(
        &Rgba8Surface::read_png(Cursor::new(include_bytes!("../test_images/plasma.png"))).unwrap(),
    );

    let mut src = BlendableSurface::new(Rgba32Surface::from(
        &Rgb8Surface::read_png(Cursor::new(include_bytes!("../test_images/text.png"))).unwrap(),
    ));
    src.surface_mut()
        .unwrap()
        .set_position(PositionI::ZERO, &dst)
        .unwrap();

    let mut group = c.benchmark_group("blend");

    for (blend_mode, id) in [
        BlendMode::Normal,
        BlendMode::Multiply,
        BlendMode::Screen,
        BlendMode::Overlay,
        BlendMode::HardLight,
        BlendMode::SoftLight,
        BlendMode::Dodge,
        BlendMode::Burn,
        BlendMode::VividLight,
        BlendMode::Divide,
    ]
    .into_iter()
    .zip([
        "normal",
        "multiply",
        "screen",
        "overlay",
        "hardlight",
        "softlight",
        "dodge",
        "burn",
        "vividlight",
        "divide",
    ]) {
        group.bench_function(id, |b| {
            b.iter_custom(|iters| {
                let mut dst = dst.clone();
                let t = Instant::now();
                for _ in 0..iters {
                    src.blend(blend_mode, 0.5, &mut dst).unwrap();
                }
                t.elapsed()
            })
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
