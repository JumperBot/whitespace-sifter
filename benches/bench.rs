use criterion::measurement::WallTime;
use criterion::{criterion_group, criterion_main, BenchmarkGroup, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    benchmark_sift_with_carriage_return_preserved(c);
    benchmark_sift_preserved(c);
    benchmark_sift_with_carriage_return(c);
    benchmark_sift(c);
}

fn benchmark_sift_with_carriage_return_preserved(c: &mut Criterion) {
    let mut g: BenchmarkGroup<WallTime> = c.benchmark_group("Sift preserved");
    let input: &str = &format!(
        "{}\r\n\r\n{}\r\n\r\n{}\r\n\r\n\r\n",
        "This. \r\n\r\nis. \r\n\r\na. \r\n\r\nsentence... \r\n\r\n",
        "With. \r\n\r\nsome. \r\n\r\nduplicate... \r\n\r\n",
        "Whitespaces. \r\n\r\n"
    );
    g.bench_with_input("Iterator Sift", &input, |c, input| {
        c.iter(|| {
            input
                .split("\r\n")
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .fold(String::new(), |acc, e| {
                    acc + "\n"
                        + &e.chars()
                            .map(|c| (c.to_string(), c.is_ascii_whitespace(), c))
                            .fold(("!".to_string(), false, '!'), |(a, aw, ac), (b, bw, bc)| {
                                if bw && aw && (ac, bc) != ('\r', '\n') {
                                    return (a, aw, ac);
                                }
                                (a + &b, bw, bc)
                            })
                            .0[1..]
                            .trim()
                            .to_string()
                })
                .trim()
                .to_string()
        });
    });
    g.bench_with_input("Loop Sift", &input, |c, input| {
        c.iter(|| {
            let mut out: String = String::with_capacity(input.len());
            for val in input.trim().split("\r\n") {
                let val: &str = val.trim();
                if val.is_empty() {
                    continue;
                }
                let mut is_last_whitespace: bool = false;
                let mut is_last_carriage_return: bool = false;
                for c in val.trim().chars() {
                    let is_carriage_return: bool = c == '\r';
                    let is_newline: bool = c == '\n';
                    let is_whitespace: bool = c.is_ascii_whitespace();
                    if is_newline && is_last_carriage_return {
                        out.push(c);
                        is_last_carriage_return = false;
                        continue;
                    }
                    if is_whitespace && is_last_whitespace {
                        continue;
                    }
                    out.push(c);
                    is_last_carriage_return = is_carriage_return;
                    is_last_whitespace = is_whitespace;
                }
                out.push_str("\r\n");
            }
            out.remove(out.len() - 1);
            out.remove(out.len() - 1);
            out
        });
    });
    g.finish();
}

fn benchmark_sift_preserved(c: &mut Criterion) {
    let mut g: BenchmarkGroup<WallTime> = c.benchmark_group("Sift preserved");
    let input: &str = &format!(
        "{}\n\n{}\n\n{}\n\n\n",
        "This. \n\nis. \n\na. \n\nsentence... \n\n",
        "With. \n\nsome. \n\nduplicate... \n\n",
        "Whitespaces. \n\n"
    );
    g.bench_with_input("Iterator Sift", &input, |c, input| {
        c.iter(|| {
            input
                .split('\n')
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .fold(String::new(), |acc, e| {
                    acc + "\n"
                        + &e.chars()
                            .map(|c| (c.to_string(), c.is_ascii_whitespace()))
                            .fold(("!".to_string(), false), |(a, aw), (b, bw)| {
                                if bw && aw {
                                    return (a, aw);
                                }
                                (a + &b, bw)
                            })
                            .0[1..]
                            .trim()
                            .to_string()
                })
                .trim()
                .to_string()
        });
    });
    g.bench_with_input("Loop Sift", &input, |c, input| {
        c.iter(|| {
            let mut out: String = String::with_capacity(input.len());
            for val in input.trim().split('\n') {
                let val: &str = val.trim();
                if val.is_empty() {
                    continue;
                }
                let mut is_last_whitespace: bool = false;
                for c in val.trim().chars() {
                    let is_whitespace: bool = c.is_ascii_whitespace();
                    if is_whitespace && is_last_whitespace {
                        continue;
                    }
                    is_last_whitespace = is_whitespace;
                    out.push(c);
                }
                out.push('\n');
            }
            out.remove(out.len() - 1);
            out
        });
    });
    g.finish();
}

fn benchmark_sift_with_carriage_return(c: &mut Criterion) {
    let mut g: BenchmarkGroup<WallTime> = c.benchmark_group("Sift with carriage return");
    let input: &str = &format!(
        "{}\r\n\r\n{}\r\n\r\n{}\r\n\r\n\r\n",
        "This\u{0020}\u{0020}is\u{0020}\u{0020}\u{0020}a\u{0020}\u{0020}sentence...",
        "With\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}some\u{0020}\u{0020}duplicate...",
        "Whitespaces."
    );
    g.bench_with_input("Iterator Sift", &input, |c, input| {
        c.iter(|| {
            input
                .chars()
                .map(|c| (c.to_string(), c.is_ascii_whitespace(), c))
                .fold(("!".to_string(), false, '!'), |(a, aw, ac), (b, bw, bc)| {
                    if bw && aw && (ac, bc) != ('\r', '\n') {
                        return (a, aw, ac);
                    }
                    (a + &b, bw, bc)
                })
                .0[1..]
                .trim()
                .to_string()
        });
    });
    g.bench_with_input("Loop Sift", &input, |c, input| {
        c.iter(|| {
            let mut out: String = String::with_capacity(input.len());
            let mut is_last_whitespace: bool = false;
            let mut is_last_carriage_return: bool = false;
            for c in input.trim().chars() {
                let is_carriage_return: bool = c == '\r';
                let is_newline: bool = c == '\n';
                let is_whitespace: bool = c.is_ascii_whitespace();
                if is_newline && is_last_carriage_return {
                    out.push(c);
                    is_last_carriage_return = false;
                    continue;
                }
                if is_whitespace && is_last_whitespace {
                    continue;
                }
                out.push(c);
                is_last_carriage_return = is_carriage_return;
                is_last_whitespace = is_whitespace;
            }
            out
        });
    });
}

fn benchmark_sift(c: &mut Criterion) {
    let mut g: BenchmarkGroup<WallTime> = c.benchmark_group("Sift");
    let input: &str = &format!(
        "{}\n\n{}\n\n{}\n\n\n",
        "This\u{0020}\u{0020}is\u{0020}\u{0020}\u{0020}a\u{0020}\u{0020}sentence...",
        "With\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}some\u{0020}\u{0020}duplicate...",
        "Whitespaces."
    );
    g.bench_with_input("Iterator Sift", &input, |c, input| {
        c.iter(|| {
            input
                .chars()
                .map(|c| (c.to_string(), c.is_ascii_whitespace()))
                .fold(("!".to_string(), false), |(a, aw), (b, bw)| {
                    if bw && aw {
                        return (a, aw);
                    }
                    (a + &b, bw)
                })
                .0[1..]
                .trim()
                .to_string()
        });
    });
    g.bench_with_input("Loop Sift", &input, |c, input| {
        c.iter(|| {
            let mut out: String = String::with_capacity(input.len());
            let mut is_last_whitespace: bool = false;
            for c in input.trim().chars() {
                let is_whitespace: bool = c.is_ascii_whitespace();
                if is_whitespace && is_last_whitespace {
                    continue;
                }
                is_last_whitespace = is_whitespace;
                out.push(c);
            }
            out
        });
    });
    g.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
