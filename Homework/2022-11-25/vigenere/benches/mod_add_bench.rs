
// Test whether we can see a timing difference based on whether key letters
// are near the begining of the range or near the end.
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

fn mod_add_no_reduction() -> usize {
    let start: usize = 0;
    let end: usize = 12;

    let modulus = 26;

    let mut c = 0;
    let mut prev = c;
    for a in start..end {
        for b in start..end {
            prev = c;
            c = (a + b) % modulus;
        }
    }
    prev
}

fn mod_add_with_reduction() -> usize {
    let start: usize = 12;
    let end: usize = 25;

    let modulus = 26;

    let mut c = 0;
    let mut prev = c;
    for a in start..end {
        for b in start..end {
            prev = c;
            c = (a + b) % modulus;
        }
    }
    prev
}

fn bench_mod_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("mod_add");
    group.bench_function("No reductions", |b| b.iter(||  mod_add_no_reduction()));
    group.bench_function("With reductions", |b| b.iter(||  mod_add_with_reduction()));

    group.finish();
}

criterion_group!(benches, bench_mod_add);
criterion_main!(benches);

