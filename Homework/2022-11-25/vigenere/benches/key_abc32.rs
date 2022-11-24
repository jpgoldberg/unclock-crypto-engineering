// Test whether we can see a timing difference based on whether key letters
// are near the begining of the range or near the end.

use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion};
use vigenere::Vigenere;

const ABC: &str = "012345ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn encrypt_with_key_type(keytype: &str) {
    let key = match keytype {
        "early" => "012345a",
        "late" => "tuvwxyz",
        _ => panic!("I should have used an enum"),
    };
    let v = Vigenere::new_with_alphabet(&key, &ABC.to_string()).unwrap();
    let msg = "THEQUICKBROWNFOXJUMPEDOVERTHELAZYDOG012345".repeat(key.len());
    v.encrypt(&msg);
}

fn bench_encrypt(c: &mut Criterion) {
    let mut group = c.benchmark_group("32 char ABC");

    for keytype in ["early", "late"] {
        group.bench_with_input(BenchmarkId::from_parameter(keytype), keytype, |b, kt| {
            b.iter(|| encrypt_with_key_type(kt))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_encrypt);
criterion_main!(benches);
