
// Test whether we can see a timing difference based on whether key letters
// are near the begining of the range or near the end.

use vigenere::Vigenere;
use criterion::{Criterion, BenchmarkId};
use criterion::{criterion_group, criterion_main};

fn encrypt_with_key_type(keytype: &str) {
    let key = match keytype {
        "early" => "abcdefg",
        "late" => "tuvwxyz",
        _ => panic!("I should have used an enum"),
    };
    let v = Vigenere::new(&key).unwrap();
    let msg= "THEQUICKBROWNFOXJUMPEDOVERTHELAZYDOG".repeat(key.len());
    v.encrypt(&msg);

}

fn bench_encrypt(c: &mut Criterion) {
    let mut group = c.benchmark_group("Encryption key type");
    
    for keytype in ["early", "late"] {
        group.bench_with_input(
                BenchmarkId::from_parameter(keytype),
                keytype,
                |b, kt| b.iter(||  encrypt_with_key_type(kt))
            );
    }

    group.finish();
}

criterion_group!(benches, bench_encrypt);
criterion_main!(benches);

