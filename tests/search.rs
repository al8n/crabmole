#![allow(warnings)]

use crabmole::sort::search;

const DATA: &[isize] = &[-10, -5, 0, 1, 2, 3, 5, 7, 11, 100, 100, 100, 1000, 10000];

fn tests() -> [Test; 21] {
    [
        Test {
            name: "1 1",
            n: 1,
            f: Box::new(|i| i >= 1),
            i: 1,
        },
        Test {
            name: "1 true",
            n: 1,
            f: Box::new(|_| true),
            i: 0,
        },
        Test {
            name: "1 false",
            n: 1,
            f: Box::new(|_| false),
            i: 1,
        },
        Test {
            name: "1e9 991",
            n: 1e9 as usize,
            f: Box::new(|i| i >= 991),
            i: 991,
        },
        Test {
            name: "1e9 true",
            n: 1e9 as usize,
            f: Box::new(|_| true),
            i: 0,
        },
        Test {
            name: "1e9 false",
            n: 1e9 as usize,
            f: Box::new(|_| false),
            i: 1e9 as usize,
        },
        Test {
            name: "data -20",
            n: DATA.len(),
            f: Box::new(f(DATA, -20)),
            i: 0,
        },
        Test {
            name: "data -10",
            n: DATA.len(),
            f: Box::new(f(DATA, -10)),
            i: 0,
        },
        Test {
            name: "data -9",
            n: DATA.len(),
            f: Box::new(f(DATA, -9)),
            i: 1,
        },
        Test {
            name: "data -6",
            n: DATA.len(),
            f: Box::new(f(DATA, -6)),
            i: 1,
        },
        Test {
            name: "data -5",
            n: DATA.len(),
            f: Box::new(f(DATA, -5)),
            i: 1,
        },
        Test {
            name: "data 3",
            n: DATA.len(),
            f: Box::new(f(DATA, 3)),
            i: 5,
        },
        Test {
            name: "data 11",
            n: DATA.len(),
            f: Box::new(f(DATA, 11)),
            i: 8,
        },
        Test {
            name: "data 99",
            n: DATA.len(),
            f: Box::new(f(DATA, 99)),
            i: 9,
        },
        Test {
            name: "data 100",
            n: DATA.len(),
            f: Box::new(f(DATA, 100)),
            i: 9,
        },
        Test {
            name: "data 101",
            n: DATA.len(),
            f: Box::new(f(DATA, 101)),
            i: 12,
        },
        Test {
            name: "data 10000",
            n: DATA.len(),
            f: Box::new(f(DATA, 10000)),
            i: 13,
        },
        Test {
            name: "data 10001",
            n: DATA.len(),
            f: Box::new(f(DATA, 10001)),
            i: 14,
        },
        Test {
            name: "descending a",
            n: 7,
            f: Box::new(|i| [99, 99, 59, 42, 7, 0, -1, -1][i] <= 7),
            i: 4,
        },
        Test {
            name: "descending 7",
            n: 1e9 as usize,
            f: Box::new(|i| 1e9 as usize - i <= 7),
            i: 1e9 as usize - 7,
        },
        Test {
            name: "overflow",
            n: 2e9 as usize,
            f: Box::new(|_| false),
            i: 2e9 as usize,
        },
    ]
}

fn f<'a>(a: &'a [isize], x: isize) -> impl FnMut(usize) -> bool + 'a {
    move |i| a[i] >= x
}

struct Test {
    name: &'static str,
    n: usize,
    f: Box<dyn FnMut(usize) -> bool>,
    i: usize,
}

#[test]
fn test_search() {
    for mut t in tests() {
        let i = search(t.n, &mut t.f);
        assert_eq!(i, t.i, "{}: expected index {}; got {}", t.name, t.i, i);
    }
}

#[inline]
const fn log2(x: usize) -> usize {
    let mut n = 0;
    let mut p = 1;
    while p < x {
        n += 1;
        p += p;
    }
    n
}

#[test]
fn test_search_efficiency() {
    let mut n = 100;
    let mut step = 1;

    for _ in 2..10 {
        let max = log2(n);
        for x in (0..n).step_by(step) {
            let mut count = 0;
            let i = search(n, move |i| {
                count += 1;
                i >= x
            });
            assert_eq!(i, x, "n = {}: expected index {}; got {}", n, x, i);
            assert!(
                count <= max,
                "n = {}, x = {}: expected <= {} calls; got {}",
                n,
                x,
                max,
                count
            );
        }

        n *= 10;
        step *= 10;
    }
}

#[test]
fn test_search_exhaustive() {
    for n in 0..=100 {
        for x in 0..=n {
            let i = search(n, move |i| i >= x);
            assert_eq!(i, x, "search({}, {}) = {}", n, x, i);
        }
    }
}
