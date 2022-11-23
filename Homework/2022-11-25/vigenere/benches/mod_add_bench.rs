
// Test whether we can see a timing difference based on whether key letters
// are near the begining of the range or near the end.
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

fn mod_add_no_reduction() -> u32 {
    let start = 0;
    let end = 12;

    let modulus = 26;

    let mut c = 0;
    for a in start..end {
        for b in start..end {
            c = (a + b) % modulus;
        }
    }
    c
}

fn mod_add_with_reduction() -> u32 {
    let start = 13;
    let end = 25;

    let modulus = 26;

    let mut c = 0;
    for a in start..end {
        for b in start..end {
            c = (a + b) % modulus;
        }
    }
    c
}

fn bench_mod_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("mod_add");
    group.bench_function("No reductions", |b| b.iter(||  mod_add_no_reduction()));
    group.bench_function("With reductions", |b| b.iter(||  mod_add_with_reduction()));

    group.finish();
}

criterion_group!(benches, bench_mod_add);
criterion_main!(benches);

