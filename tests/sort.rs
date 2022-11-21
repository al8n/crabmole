#![allow(warnings)]

use std::cell::{Cell, RefCell};

use crabmole::sort::{IndexSort, SliceSortExt};
use rand::Rng;

const INTS: &[isize] = &[
    74, 59, 238, -784, 9845, 959, 905, 0, 0, 42, 7586, -5467984, 7586,
];

const FLOATS: &[f64] = &[
    74.3,
    59.0,
    f64::INFINITY,
    238.2,
    -784.0,
    2.3,
    f64::NAN,
    f64::NAN,
    f64::INFINITY * -1f64,
    9845.768,
    -959.7485,
    905f64,
    7.8,
    7.8,
];

const STRINGS: &[&str] = &["", "Hello", "foo", "bar", "foo", "f00", "%*&^*&^&", "***"];

#[test]
fn test_sort_int_slice() {
    let mut data = INTS.to_vec();
    IndexSort::sort(&mut data);
    assert!(IndexSort::is_sorted(&data));

    let mut data = INTS.to_vec();
    IndexSort::sort_stable(&mut data);
    assert!(IndexSort::is_sorted(&data));
}

#[test]
fn test_sort_f64_slice() {
    let mut data = FLOATS.to_vec();
    IndexSort::sort(&mut data);
    assert!(IndexSort::is_sorted(&data));

    let mut data = FLOATS.to_vec();
    IndexSort::sort_stable(&mut data);
    assert!(IndexSort::is_sorted(&data));
}

#[test]
fn test_sort_string_slice() {
    let mut data = STRINGS.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    IndexSort::sort(&mut data);
    assert!(IndexSort::is_sorted(&data));

    let mut data = STRINGS.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    IndexSort::sort_stable(&mut data);
    assert!(IndexSort::is_sorted(&data));
}

#[test]
fn test_slice() {
    let mut data = STRINGS.iter().map(|s| s.to_string()).collect::<Vec<_>>();

    String::sort_slice(&mut data, |d, i, j| d[i] < d[j]);

    assert!(String::slice_is_sorted(&data, |i, j| { data[i] < data[j] }));

    let mut data = STRINGS.iter().map(|s| s.to_string()).collect::<Vec<_>>();

    String::sort_slice_stable(&mut data, |d, i, j| d[i] < d[j]);

    assert!(String::slice_is_sorted(&data, |i, j| { data[i] < data[j] }));
}

#[test]
fn test_sort_large_random() {
    let mut data = (0..1000000)
        .map(|_| rand::random::<isize>())
        .collect::<Vec<_>>();
    IndexSort::sort(&mut data);
    assert!(IndexSort::is_sorted(&data));

    let mut data = (0..1000000)
        .map(|_| rand::random::<isize>())
        .collect::<Vec<_>>();
    IndexSort::sort_stable(&mut data);
    assert!(IndexSort::is_sorted(&data));
}

#[test]
fn test_reverse_sort_int_slice() {
    let mut data = INTS.to_vec();
    let mut data1 = INTS.to_vec();

    IndexSort::sort(&mut data);
    IndexSort::sort_reverse(&mut data1);
    for i in 0..INTS.len() {
        assert_eq!(data[i], data1[INTS.len() - i - 1]);
        if i > data.len() / 2 {
            break;
        }
    }
}

#[derive(Debug)]
struct NonDeterministicTestingData;

impl IndexSort for NonDeterministicTestingData {
    fn len(&self) -> usize {
        500
    }

    fn less(&self, i: usize, j: usize) -> bool {
        if i >= self.len() || j >= self.len() {
            panic!("nondeterministic comparison out of bounds")
        }

        rand::thread_rng().gen_range(0f32..1f32) < 0.5f32
    }

    fn swap(&mut self, i: usize, j: usize) {
        if i >= self.len() || j >= self.len() {
            panic!("nondeterministic comparison out of bounds")
        }
    }
}

#[test]
fn test_non_deterministic_comparison() {
    for _ in 0..10 {
        IndexSort::sort(&mut NonDeterministicTestingData);
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
enum Distribution {
    Sawtooth,
    Rand,
    Stagger,
    Plateau,
    Shuffle,
    NDist,
}

impl From<usize> for Distribution {
    fn from(i: usize) -> Self {
        match i {
            0 => Distribution::Sawtooth,
            1 => Distribution::Rand,
            2 => Distribution::Stagger,
            3 => Distribution::Plateau,
            4 => Distribution::Shuffle,
            5 => Distribution::NDist,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
enum Mode {
    Copy,
    Reverse,
    ReverseFirstHalf,
    ReverseSecondHalf,
    Sorted,
    Dither,
    NMode,
}

impl From<usize> for Mode {
    fn from(i: usize) -> Self {
        match i {
            0 => Mode::Copy,
            1 => Mode::Reverse,
            2 => Mode::ReverseFirstHalf,
            3 => Mode::ReverseSecondHalf,
            4 => Mode::Sorted,
            5 => Mode::Dither,
            6 => Mode::NMode,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct TestingData {
    desc: String,
    data: Vec<usize>,
    max_swap: usize,
    ncmp: Cell<usize>,
    nswap: usize,
}

impl IndexSort for TestingData {
    fn len(&self) -> usize {
        self.data.len()
    }

    fn less(&self, i: usize, j: usize) -> bool {
        let cmp = self.ncmp.get();
        self.ncmp.set(cmp + 1);
        self.data[i] < self.data[j]
    }

    fn swap(&mut self, i: usize, j: usize) {
        if self.nswap >= self.max_swap {
            panic!(
                "{}: used {} swaps sorting slice of {}",
                self.desc,
                self.nswap,
                self.data.len()
            );
        }
        self.nswap += 1;
        self.data.swap(i, j);
    }
}

fn test_bentley_mc_ilroy<S, M>(sort: S, maxswap: M)
where
    S: Fn(&mut TestingData),
    M: Fn(usize) -> usize,
{
    let sizes = [100, 1023, 1024, 1025];
    let dists = ["sawtooth", "rand", "stagger", "plateau", "shuffle"];
    let modes = ["copy", "reverse", "reverse1", "reverse2", "sort", "dither"];

    let tmp1 = [0; 1025];
    let tmp2 = [0; 1025];
    for n in sizes {
        let mut m = 1;
        while m < 2 * n {
            for dist in 0..Distribution::NDist as usize {
                let mut j = 0;
                let mut k = 1;
                let mut data = tmp1[0..n].to_vec();
                for i in 0..n {
                    match Distribution::from(dist) {
                        Distribution::Sawtooth => {
                            data[i] = i % m;
                        }
                        Distribution::Rand => {
                            data[i] = rand::thread_rng().gen_range(0..m);
                        }
                        Distribution::Stagger => {
                            data[i] = (i * m + i) % n;
                        }
                        Distribution::Plateau => {
                            data[i] = i.min(m);
                        }
                        Distribution::Shuffle => {
                            let v = rand::thread_rng().gen_range(0..m);
                            if v != 0 {
                                j += 2;
                                data[i] = j;
                            } else {
                                k += 2;
                                data[i] = k;
                            }
                        }
                        _ => unreachable!(),
                    }
                }

                let mut mdata = tmp2[0..n].to_vec();
                for mode in 0..Mode::NMode as usize {
                    match Mode::from(mode) {
                        Mode::Copy => {
                            mdata.copy_from_slice(&data);
                        }
                        Mode::Reverse => {
                            for i in 0..n {
                                mdata[i] = data[n - i - 1];
                            }
                        }
                        Mode::ReverseFirstHalf => {
                            for i in 0..n / 2 {
                                mdata[i] = data[n / 2 - i - 1];
                            }
                            mdata[(n / 2)..n].copy_from_slice(&data[(n / 2)..n]);
                        }
                        Mode::ReverseSecondHalf => {
                            mdata[..(n / 2)].copy_from_slice(&data[..(n / 2)]);
                            for i in n / 2..n {
                                mdata[i] = data[n - (i - n / 2) - 1];
                            }
                        }
                        Mode::Sorted => {
                            mdata.copy_from_slice(&data);
                            mdata.sort();
                        }
                        Mode::Dither => {
                            for i in 0..n {
                                mdata[i] = data[i] + i % 5;
                            }
                        }
                        _ => unreachable!(),
                    }

                    let desc = format!(
                        "n={}, m={}, dist={}, mode={}",
                        n, m, dists[dist], modes[mode]
                    );
                    let mut d = TestingData {
                        desc,
                        data: mdata.clone(),
                        max_swap: maxswap(n),
                        ncmp: Cell::new(0),
                        nswap: 0,
                    };
                    sort(&mut d);

                    // Uncomment if you are trying to improve the number of compares/swaps.
                    //t.Logf("%s: ncmp=%d, nswp=%d", desc, d.ncmp, d.nswap)

                    // If we were testing C qsort, we'd have to make a copy
                    // of the slice and sort it ourselves and then compare
                    // x against it, to ensure that qsort was only permuting
                    // the data, not (for example) overwriting it with zeros.
                    //
                    // In go, we don't have to be so paranoid: since the only
                    // mutating method Sort can call is TestingData.swap,
                    // it suffices here just to check that the final slice is sorted.
                    assert!(
                        d.data.is_sorted(),
                        "{}: data not sorted {:?}",
                        d.desc,
                        mdata
                    );
                }
            }
            m *= 2;
        }
    }
}

fn lg(n: usize) -> usize {
    let mut i = 0;
    while (1 << i) < n {
        i += 1;
    }
    i
}

#[test]
fn test_sort_bm() {
    test_bentley_mc_ilroy(
        |data| {
            data.sort();
        },
        |n| n * lg(n) * 12 / 10,
    );
}

#[test]
fn test_stable_bm() {
    test_bentley_mc_ilroy(
        |data| {
            data.sort_stable();
        },
        |n| n * lg(n) * lg(n) / 3,
    );
}

// This is based on the "antiquicksort" implementation by M. Douglas McIlroy.
// See https://www.cs.dartmouth.edu/~doug/mdmspe.pdf for more info.
struct AdversaryTestingData {
    /// item values, initialized to special gas value and changed by Less
    data: RefCell<Vec<usize>>,
    /// number of comparisons allowed
    maxcmp: usize,
    /// number of comparisons (calls to Less)
    ncmp: Cell<usize>,
    /// number of elements that have been set to non-gas values
    nsolid: Cell<usize>,
    /// guess at current pivot
    candidate: Cell<usize>,
    /// special value for unset elements, higher than everything else
    gas: usize,
}

impl IndexSort for AdversaryTestingData {
    fn len(&self) -> usize {
        self.data.borrow().len()
    }

    fn less(&self, i: usize, j: usize) -> bool {
        let mut data = self.data.borrow_mut();
        let ncmp = self.ncmp.get();
        assert!(
            ncmp < self.maxcmp,
            "used {} comparisons sorting adversary data with size {}",
            ncmp,
            data.len()
        );
        self.ncmp.set(ncmp + 1);

        if data[i] == self.gas && data[j] == self.gas {
            let nsolid = self.nsolid.get();
            if i == self.candidate.get() {
                // freeze i
                data[i] = nsolid;
                self.nsolid.set(nsolid + 1);
            } else {
                // freeze j
                data[j] = nsolid;
                self.nsolid.set(nsolid + 1);
            }
        }

        if data[i] == self.gas {
            self.candidate.set(i);
        } else if data[j] == self.gas {
            self.candidate.set(j);
        }

        data[i] < data[j]
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.data.borrow_mut().swap(i, j);
    }
}

impl AdversaryTestingData {
    fn new(size: usize, maxcmp: usize) -> Self {
        let gas = size - 1;
        let data = vec![gas; size];
        AdversaryTestingData {
            data: RefCell::new(data),
            maxcmp,
            ncmp: Cell::new(0),
            nsolid: Cell::new(0),
            candidate: Cell::new(0),
            gas,
        }
    }
}

#[test]
fn test_adversary() {
    // large enough to distinguish between O(n^2) and O(n*log(n))
    const SIZE: usize = 10_000;

    // the factor 4 was found by trial and error
    let maxcmp = SIZE * lg(SIZE) * 4;

    let mut data = AdversaryTestingData::new(SIZE, maxcmp);
    // This should degenerate to heapsort.
    data.sort();
    // Check data is fully populated and sorted.
    for (i, v) in data.data.borrow().iter().enumerate() {
        assert_eq!(*v, i, "dversary data not fully sorted");
    }
}

#[derive(Clone, Copy)]
struct Pair {
    a: isize,
    b: isize,
}

impl core::fmt::Debug for Pair {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{{{} {}}}", self.a, self.b)
    }
}

#[derive(Clone)]
struct Pairs {
    data: Vec<Pair>,
}

impl IndexSort for Pairs {
    fn len(&self) -> usize {
        self.data.len()
    }

    fn less(&self, i: usize, j: usize) -> bool {
        self.data[i].a < self.data[j].a
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
    }
}

impl Pairs {
    fn init_b(&mut self) {
        for (i, mut p) in self.data.iter_mut().enumerate() {
            p.b = i as isize;
        }
    }

    fn in_order(&self) -> bool {
        let (mut last_a, mut last_b) = (-1, 0);
        for i in 0..self.data.len() {
            if last_a != self.data[i].a {
                last_a = self.data[i].a;
                last_b = self.data[i].b;
                continue;
            }
            if self.data[i].b <= last_b {
                return false;
            }
            last_b = self.data[i].b;
        }
        true
    }
}

#[test]
fn test_stability() {
    const N: usize = 100_000;
    const M: usize = 1000;
    let mut data = Pairs {
        data: vec![Pair { a: 0, b: 0 }; N],
    };

    // random distribution
    for i in 0..N {
        data.data[i].a = rand::thread_rng().gen_range(0..M as isize);
    }

    assert!(!data.is_sorted(), "terrible rand");

    data.init_b();
    data.sort_stable();
    assert!(data.is_sorted(), "Stable didn't sort {N} ints");
    assert!(data.in_order(), "Stable wasn't stable on {N} ints");

    // already sorted
    data.init_b();
    data.sort_stable();
    assert!(data.is_sorted(), "Stable shuffled sorted {N} ints (order)");

    assert!(
        data.in_order(),
        "Stable shuffled sorted {N} ints (stability)"
    );

    // sorted reversed
    let mut data = Pairs {
        data: vec![Pair { a: 0, b: 0 }; N],
    };
    for i in 0..N {
        data.data[i].a = (N - i) as isize;
    }
    data.init_b();
    data.sort_stable();
    assert!(data.is_sorted(), "Stable didn't sort {N} ints");
    assert!(data.in_order(), "Stable wasn't stable on {N} ints");
}

const COUNT_OPS_SIZES: &[usize] = &[100, 300, 1000, 3000, 10000, 30000, 100000, 300000, 1000000];

fn count_ops<F>(f: F, name: &'static str)
where
    F: Fn(&mut TestingData),
{
    for n in COUNT_OPS_SIZES.iter() {
        let mut td = TestingData {
            desc: name.to_string(),
            data: vec![0; *n],
            max_swap: 2_147_483_647,
            ncmp: Cell::new(0),
            nswap: 0,
        };
        for i in 0..*n {
            td.data[i] = rand::thread_rng().gen_range(0..*n / 5);
        }
        f(&mut td);
        eprintln!(
            "{} {:8} elements: {:11} swap, {:10} less",
            name,
            n,
            td.nswap,
            td.ncmp.get()
        );
    }
}

#[test]
fn test_count_stable_ops() {
    count_ops(|td| td.sort_stable(), "Stable");
}

#[test]
fn test_count_sort_ops() {
    count_ops(|td| td.sort(), "Sort");
}
