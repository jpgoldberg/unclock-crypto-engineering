use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion};
use hw3::ex5_3::*;
// use std::time::Duration;

fn bench_encrypt(c: &mut Criterion) {
    let mut group = c.benchmark_group("Collision search");
    group.sample_size(20);
    // group.measurement_time(Duration::from_secs(240));

    for length in [8_u16, 16, 24, 32, 48] {
        group.bench_with_input(BenchmarkId::from_parameter(length), &length, |b, length| {
            b.iter(|| hash_collisions(*length))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_encrypt);
criterion_main!(benches);
