use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::fs::read_to_string;
use whitespace_sifter::WhitespaceSifter;

fn criterion_benchmark(c: &mut Criterion) {
    let input: String = read_to_string("Bee_Movie_Script.txt").unwrap();
    c.bench_with_input(BenchmarkId::new("Sift", "Sift"), &input, |c, input| {
        c.iter(|| input.sift());
    });
    c.bench_with_input(
        BenchmarkId::new("Sift Preserved", "Sift Preserved"),
        &input,
        |c, input| {
            c.iter(|| input.sift_preserve_newlines());
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
