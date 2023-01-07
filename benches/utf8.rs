use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn bench_char_count_ten_ascii_chars(c: &mut Criterion) {
    let _s = "0123456789".as_bytes();
    c.bench_function("char_count_ten_ascii_chars", |b| {
        b.iter(|| crabmole::unicode::utf8::char_count(_s))
    });
}

criterion_group! {
    name = benches_utf8;
    config = Criterion::default();
    targets = bench_char_count_ten_ascii_chars
}

criterion_main!(benches_utf8);
