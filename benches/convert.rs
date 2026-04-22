use blittle::*;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

const SRC_W: usize = 512;
const SRC_H: usize = 512;

macro_rules! convert {
    ($group:ident, $from:tt, $to:tt, $name:literal) => {
        let src = $from::new(Size::new(SRC_W, SRC_H));
        $group.bench_function($name, |b| {
            b.iter(|| {
                black_box($to::from(&src));
            })
        });
    };
}

macro_rules! convert_group {
    ($c:ident, $from:tt, $group_name:literal, $($to:tt, $name:literal),*) => {
        let mut group = $c.benchmark_group($group_name);
        $(
        {
            convert!(group, $from, $to, $name);
        }
        )*
        group.finish();
    };
}

pub fn criterion_benchmark(c: &mut Criterion) {
    convert_group!(
        c,
        L8Surface,
        "l8",
        La8Surface,
        "la8",
        L32Surface,
        "l32",
        La32Surface,
        "la32",
        Rgb8Surface,
        "rgb8",
        Rgba8Surface,
        "rgba8",
        Rgba32Surface,
        "rgba32"
    );

    convert_group!(
        c,
        La8Surface,
        "la8",
        L8Surface,
        "l8",
        L32Surface,
        "l32",
        La32Surface,
        "la32",
        Rgb8Surface,
        "rgb8",
        Rgba8Surface,
        "rgba8",
        Rgba32Surface,
        "rgba32"
    );

    convert_group!(
        c,
        L32Surface,
        "l32",
        L8Surface,
        "l8",
        La8Surface,
        "la8",
        La32Surface,
        "la32",
        Rgb8Surface,
        "rgb8",
        Rgba8Surface,
        "rgba8",
        Rgba32Surface,
        "rgba32"
    );

    convert_group!(
        c,
        La32Surface,
        "la32",
        L8Surface,
        "l8",
        La8Surface,
        "la8",
        L32Surface,
        "l32",
        Rgb8Surface,
        "rgb8",
        Rgba8Surface,
        "rgba8",
        Rgba32Surface,
        "rgba32"
    );

    convert_group!(
        c,
        Rgb8Surface,
        "rgb8",
        L8Surface,
        "l8",
        La8Surface,
        "la8",
        L32Surface,
        "l32",
        La32Surface,
        "la32",
        Rgba8Surface,
        "rgba8",
        Rgba32Surface,
        "rgba32"
    );

    convert_group!(
        c,
        Rgba8Surface,
        "rgba8",
        L8Surface,
        "l8",
        La8Surface,
        "la8",
        L32Surface,
        "l32",
        La32Surface,
        "la32",
        Rgb8Surface,
        "rgb8",
        Rgba32Surface,
        "rgba32"
    );

    convert_group!(
        c,
        Rgba32Surface,
        "rgba32",
        L8Surface,
        "l8",
        La8Surface,
        "la8",
        L32Surface,
        "l32",
        La32Surface,
        "la32",
        Rgb8Surface,
        "rgb8",
        Rgba8Surface,
        "rgba8"
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
