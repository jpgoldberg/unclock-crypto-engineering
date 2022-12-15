use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion};
use hw3::ex5_4::*;

fn bench_pre_image_which_hash(c: &mut Criterion) {
    let mut group = c.benchmark_group("Preimage Sha v BLAKE");
    group.sample_size(10);

    for prefix in ["ab", "abc"] {
        group.bench_with_input(BenchmarkId::new("Sha512", prefix), prefix, |b, prefix| {
            b.iter(|| find_preimage_for_prefix(prefix.as_bytes().into()))
        });
        group.bench_with_input(BenchmarkId::new("BLAKE3", prefix), prefix, |b, prefix| {
            b.iter(|| b3_find_preimage_for_prefix(prefix.as_bytes().into()))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_pre_image_which_hash);
criterion_main!(benches);
