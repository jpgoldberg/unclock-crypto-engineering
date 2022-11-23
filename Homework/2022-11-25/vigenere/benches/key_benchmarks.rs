
#![allow(unused)]


// Test whether we can see a timing difference based on whether key letters
// are near the begining of the range or near the end.

use vigenere::Vigenere;
use std::iter;
use std::fmt;
use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;
use criterion::{criterion_group, criterion_main};

fn encrypt_with_early_key() {
    let key = "abcdefg";

    let v = Vigenere::new(&key).unwrap();

    let msg= "THEQUICKBROWNFOXJUMPEDOVERTHELAZYDOG".repeat(7);
    v.encrypt(&msg);

}

fn encrypt_with_late_key() {
    let key = "tuvwxyz";

    let v = Vigenere::new(&key).unwrap();

    let msg= "THEQUICKBROWNFOXJUMPEDOVERTHELAZYDOG".repeat(7);

    v.encrypt(&msg);

}

fn bench_encrypt(c: &mut Criterion) {
    let mut group = c.benchmark_group("Encryption");
    group.bench_function("Early key", |b| b.iter(||  encrypt_with_early_key()));
    group.bench_function("Late key", |b| b.iter(||  encrypt_with_late_key()));

    group.finish();
}

criterion_group!(benches, bench_encrypt);
criterion_main!(benches);

