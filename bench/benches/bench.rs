//use collapse::collapse;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
//use fast_whitespace_collapse::collapse_whitespace;
use std::fs::read_to_string;
use whitespace_sifter::WhitespaceSifter;

fn criterion_benchmark(c: &mut Criterion) {
    let input: String = read_to_string("../Bee_Movie_Script.txt").unwrap();
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
    /*c.bench_with_input(
        BenchmarkId::new("Comparison", "Collapse"),
        &input,
        |c, input| {
            c.iter(|| collapse(input));
        },
    );
    c.bench_with_input(
        BenchmarkId::new("Comparison", "Fast_Whitespace_Collapse SIMD"),
        &input,
        |c, input| {
            c.iter(|| collapse_whitespace(input));
        },
    );*/
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
