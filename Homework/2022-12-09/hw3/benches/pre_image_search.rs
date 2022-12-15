// Test whether we can see a timing difference based on whether key letters
// are near the begining of the range or near the end.

use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion};
use hw3::ex5_4::*;

fn bench_pre_image(c: &mut Criterion) {
    let mut group = c.benchmark_group("Preimage size");
    group.sample_size(10);

    for prefix in ["ab", "abc"] {
        group.bench_with_input(BenchmarkId::from_parameter(prefix), prefix, |b, prefix| {
            b.iter(|| find_preimage_for_prefix(prefix.as_bytes().into()))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_pre_image);
criterion_main!(benches);
