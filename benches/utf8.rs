use crabmole::unicode::utf8::*;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn bench_char_count_ten_ascii_chars(c: &mut Criterion) {
    let _s = "0123456789".as_bytes();
    c.bench_function("char_count_ten_ascii_chars", |b| {
        b.iter(|| crabmole::unicode::utf8::char_count(_s))
    });
}

fn bench_char_count_ten_japanese_chars(c: &mut Criterion) {
    let _s = "日本語日本語日本語日".as_bytes();
    c.bench_function("char_count_ten_japanese_chars", |b| {
        b.iter(|| char_count(_s))
    });
}

fn bench_valid_ten_ascii_chars(c: &mut Criterion) {
    let _s = "0123456789".as_bytes();
    c.bench_function("valid_ten_ascii_chars", |b| b.iter(|| valid(_s)));
}

fn ascii100000() -> String {
    "0123456789".repeat(100_000)
}

fn bench_valid_100k_ascii_chars(c: &mut Criterion) {
    c.bench_function("valid_100k_ascii_chars", |b| {
        b.iter(|| valid(ascii100000().as_bytes()))
    });
}

fn bench_valid_ten_japanese_chars(c: &mut Criterion) {
    let _s = "日本語日本語日本語日".as_bytes();
    c.bench_function("valid_ten_japanese_chars", |b| b.iter(|| valid(_s)));
}

const JAPANESE: &str = "日本語日本語日本語日";

fn long_string_mostly_ascii() -> String {
    let mut _s = String::new();
    let mut i = 0;
    while _s.len() < 100_000 {
        if i % 100 == 0 {
            _s += JAPANESE;
        } else {
            _s += "0123456789";
        }
        i += 1;
    }
    _s
}

fn long_string_japanese() -> String {
    JAPANESE.repeat(100_000 / JAPANESE.len())
}

fn bench_valid_long_mostly_ascii_chars(c: &mut Criterion) {
    c.bench_function("valid_long_mostly_ascii_chars", |b| {
        b.iter(|| valid(long_string_mostly_ascii().as_bytes()))
    });
}

fn bench_valid_long_japanese(c: &mut Criterion) {
    c.bench_function("valid_long_japanese", |b| {
        b.iter(|| valid(long_string_japanese().as_bytes()))
    });
}

fn bench_encode_ascii_char(c: &mut Criterion) {
    let mut buf = vec![0; UTF_MAX];
    c.bench_function("encode_ascii_char", |b| {
        b.iter(|| encode_char(&mut buf, 'a'))
    });
}

fn bench_encode_japanese_char(c: &mut Criterion) {
    let mut buf = vec![0; UTF_MAX];
    c.bench_function("encode_japanese_char", |b| {
        b.iter(|| encode_char(&mut buf, '本'))
    });
}

fn bench_append_ascii_char(c: &mut Criterion) {
    let mut buf = vec![0; UTF_MAX];
    c.bench_function("append_ascii_char", |b| {
        b.iter(|| append_char(&mut buf, 'a'))
    });
}

fn bench_append_japanese_char(c: &mut Criterion) {
    let mut buf = vec![0; UTF_MAX];
    c.bench_function("append_japanese_char", |b| {
        b.iter(|| append_char(&mut buf, '本'))
    });
}

fn bench_decode_ascii_char(c: &mut Criterion) {
    let a = "a".as_bytes();
    c.bench_function("decode_ascii_char", |b| b.iter(|| decode_char(a)));
}

fn bench_decode_japanese_char(c: &mut Criterion) {
    let nihon = "本".as_bytes();
    c.bench_function("decode_japanese_char", |b| b.iter(|| decode_char(nihon)));
}

struct BenchM {
    name: &'static str,
    data: Vec<u8>,
}

fn bench_marks() -> Vec<BenchM> {
    vec![
        BenchM {
            name: "ASCII",
            data: "a".as_bytes().to_vec(),
        },
        BenchM {
            name: "Incomplete",
            data: b"\xf0\x90\x80".to_vec(),
        },
        BenchM {
            name: "Japanese",
            data: "本".as_bytes().to_vec(),
        },
    ]
}

fn bench_full_char(c: &mut Criterion) {
    for bm in bench_marks() {
        let mut bool_sink: bool = true;
        c.bench_function(bm.name, |b| {
            b.iter(|| {
                bool_sink = full_char(&bm.data);
            })
        });
    }
}

criterion_group! {
    name = benches_utf8;
    config = Criterion::default();
    targets = bench_char_count_ten_ascii_chars,
        bench_char_count_ten_japanese_chars,
        bench_valid_ten_ascii_chars,
        bench_valid_100k_ascii_chars,
        bench_valid_ten_japanese_chars,
        bench_valid_long_mostly_ascii_chars,
        bench_valid_long_japanese,
        bench_encode_ascii_char,
        bench_encode_japanese_char,
        bench_append_ascii_char,
        bench_append_japanese_char,
        bench_decode_ascii_char,
        bench_decode_japanese_char,
        bench_full_char
}

criterion_main!(benches_utf8);
