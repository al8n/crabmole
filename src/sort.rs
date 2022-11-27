struct LessSwap<'a, T, L> {
    data: &'a mut [T],
    less: L,
}

struct ImmutableLessSwap<'a, T, L> {
    data: &'a [T],
    less: L,
}

impl<'a, T, L> Sort for ImmutableLessSwap<'a, T, L>
where
    L: Fn(usize, usize) -> bool,
{
    fn len(&self) -> usize {
        self.data.len()
    }

    fn swap(&mut self, _i: usize, _j: usize) {
        unreachable!()
    }

    fn less(&self, i: usize, j: usize) -> bool {
        (self.less)(i, j)
    }
}

impl<'a, T, L> Sort for LessSwap<'a, T, L>
where
    L: Fn(&[T], usize, usize) -> bool,
{
    fn len(&self) -> usize {
        self.data.len()
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
    }

    fn less(&self, i: usize, j: usize) -> bool {
        (self.less)(self.data, i, j)
    }
}

/// Golang's `sort.Slice`, `sort.SliceStable` and `sort.SliceIsSorted` in Rust
pub trait SliceSortExt {
    /// Item
    type Item;

    /// Slice sorts the slice x given the provided less function.
    ///
    /// The sort is not guaranteed to be stable: equal elements
    /// may be reversed from their original order.
    /// For a stable sort, use `slice_stable`.
    #[inline]
    fn sort_slice<L>(data: &mut [Self::Item], less: L)
    where
        L: Fn(&[Self::Item], usize, usize) -> bool,
    {
        let mut sorter = LessSwap { data, less };
        sorter.sort()
    }

    /// Sorts the slice data using the provided less
    /// function, keeping equal elements in their original order.
    #[inline]
    fn sort_slice_stable<L>(data: &mut [Self::Item], less: L)
    where
        L: Fn(&[Self::Item], usize, usize) -> bool,
    {
        let mut sorter = LessSwap { data, less };
        sorter.sort_stable()
    }

    /// Returns whether the slice x is sorted according to the provided less function.
    #[inline]
    fn slice_is_sorted<L>(data: &[Self::Item], less: L) -> bool
    where
        L: Fn(usize, usize) -> bool,
    {
        let sorter = ImmutableLessSwap { data, less };
        sorter.is_sorted()
    }
}

impl<T> SliceSortExt for T {
    type Item = T;
}

/// Slice sorts the slice x given the provided less function.
///
/// The sort is not guaranteed to be stable: equal elements
/// may be reversed from their original order.
/// For a stable sort, use `slice_stable`.
#[inline]
pub fn sort_slice<T, L>(data: &mut [T], less: L)
where
    L: Fn(&[T], usize, usize) -> bool,
{
    let mut sorter = LessSwap { data, less };
    sorter.sort()
}

/// Sorts the slice data using the provided less
/// function, keeping equal elements in their original order.
#[inline]
pub fn sort_slice_stable<T, L>(data: &mut [T], less: L)
where
    L: Fn(&[T], usize, usize) -> bool,
{
    let mut sorter = LessSwap { data, less };
    sorter.sort_stable()
}

/// Returns whether the slice x is sorted according to the provided less function.
#[inline]
pub fn slice_is_sorted<T, L>(data: &[T], less: L) -> bool
where
    L: Fn(usize, usize) -> bool,
{
    let sorter = ImmutableLessSwap { data, less };
    sorter.is_sorted()
}

/// Sort in reverse helper structure
struct Reverse<'a, T>(&'a mut T);

impl<'a, T: Sort> Sort for Reverse<'a, T> {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.0.swap(i, j);
    }

    fn less(&self, i: usize, j: usize) -> bool {
        self.0.less(j, i)
    }
}

struct ImmutableReverse<'a, T>(&'a T);

impl<'a, T: Sort> Sort for ImmutableReverse<'a, T> {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn swap(&mut self, _i: usize, _j: usize) {
        unreachable!()
    }

    fn less(&self, i: usize, j: usize) -> bool {
        self.0.less(j, i)
    }
}

/// Golang sort interface in Rust.
///
/// Implement this trait for type to unlock all the sorting methods in Go standard library.
#[allow(clippy::len_without_is_empty)]
pub trait Sort {
    /// Len is the number of elements in the collection.
    fn len(&self) -> usize;

    /// Less reports whether the element with index i
    /// must sort before the element with index j.
    ///
    /// If both Less(i, j) and Less(j, i) are false,
    /// then the elements at index i and j are considered equal.
    /// Sort may place equal elements in any order in the final result,
    /// while Stable preserves the original input order of equal elements.
    ///
    /// Less must describe a transitive ordering:
    ///  - if both less(i, j) and Less(j, k) are true, then Less(i, k) must be true as well.
    ///  - if both less(i, j) and less(j, k) are false, then less(i, k) must be false as well.
    fn less(&self, i: usize, j: usize) -> bool;

    /// Swaps the elements with indexes i and j.
    fn swap(&mut self, i: usize, j: usize);

    /// Sort data.
    /// It makes one call to data.Len to determine n and O(n*log(n)) calls to
    /// data.Less and data.Swap. The sort is not guaranteed to be stable.
    #[inline]
    fn sort(&mut self)
    where
        Self: Sized,
    {
        let n = self.len();
        quick_sort(self, 0, n, max_depth(n));
    }

    /// Notes on stable sorting:
    /// The used algorithms are simple and provable correct on all input and use
    /// only logarithmic additional stack space. They perform well if compared
    /// experimentally to other stable in-place sorting algorithms.
    ///
    /// Remarks on other algorithms evaluated:
    ///  - GCC's 4.6.3 stable_sort with merge_without_buffer from libstdc++:
    ///    Not faster.
    ///  - GCC's __rotate for block rotations: Not faster.
    ///  - "Practical in-place mergesort" from  Jyrki Katajainen, Tomi A. Pasanen
    ///    and Jukka Teuhola; Nordic Journal of Computing 3,1 (1996), 27-40:
    ///    The given algorithms are in-place, number of Swap and Assignments
    ///    grow as n log n but the algorithm is not stable.
    ///  - "Fast Stable In-Place Sorting with O(n) Data Moves" J.I. Munro and
    ///    V. Raman in Algorithmica (1996) 16, 115-160:
    ///    This algorithm either needs additional 2n bits or works only if there
    ///    are enough different elements available to encode some permutations
    ///    which have to be undone later (so not stable on any input).
    ///  - All the optimal in-place sorting/merging algorithms I found are either
    ///    unstable or rely on enough different elements in each step to encode the
    ///    performed block rearrangements. See also "In-Place Merging Algorithms",
    ///    Denham Coates-Evely, Department of Computer Science, Kings College,
    ///    January 2004 and the references in there.
    ///  - Often "optimal" algorithms are optimal in the number of assignments
    ///    but Interface has only Swap as operation.
    ///
    /// Stable sorts data in ascending order as determined by the Less method,
    /// while keeping the original order of equal elements.
    ///
    /// It makes one call to data.Len to determine n, O(n*log(n)) calls to
    /// data.Less and O(n*log(n)*log(n)) calls to data.Swap.
    #[inline]
    fn sort_stable(&mut self)
    where
        Self: Sized,
    {
        let n = self.len();
        stable(self, n);
    }

    /// Returns whether the data is sorted.
    #[inline]
    fn is_sorted(&self) -> bool {
        let len = self.len();

        let n = (len >> 1) + 1;
        for i in 1..n {
            if self.less(i, i - 1) {
                return false;
            }
            let tail_off = len - i;

            if self.less(tail_off, tail_off - 1) {
                return false;
            }
        }
        true
    }

    /// Returns whether the data is sorted in reverse order.
    #[inline]
    fn is_reverse_sorted(&self) -> bool {
        let n = self.len();
        for i in (1..n).rev() {
            if self.less(i - 1, i) {
                return false;
            }
        }
        true
    }

    /// Sorts data in reverse order.
    /// It makes one call to data.Len to determine n and O(n*log(n)) calls to
    /// data.Less and data.Swap. The sort is not guaranteed to be stable.
    #[inline]
    fn sort_reverse(&mut self)
    where
        Self: Sized,
    {
        let n = self.len();
        quick_sort(&mut Reverse(self), 0, n, max_depth(n));
    }

    /// Sorts (stable) data in reverse order.
    ///  
    /// Notes on stable sorting:
    /// The used algorithms are simple and provable correct on all input and use
    /// only logarithmic additional stack space. They perform well if compared
    /// experimentally to other stable in-place sorting algorithms.
    ///
    /// Remarks on other algorithms evaluated:
    ///  - GCC's 4.6.3 stable_sort with merge_without_buffer from libstdc++:
    ///    Not faster.
    ///  - GCC's __rotate for block rotations: Not faster.
    ///  - "Practical in-place mergesort" from  Jyrki Katajainen, Tomi A. Pasanen
    ///    and Jukka Teuhola; Nordic Journal of Computing 3,1 (1996), 27-40:
    ///    The given algorithms are in-place, number of Swap and Assignments
    ///    grow as n log n but the algorithm is not stable.
    ///  - "Fast Stable In-Place Sorting with O(n) Data Moves" J.I. Munro and
    ///    V. Raman in Algorithmica (1996) 16, 115-160:
    ///    This algorithm either needs additional 2n bits or works only if there
    ///    are enough different elements available to encode some permutations
    ///    which have to be undone later (so not stable on any input).
    ///  - All the optimal in-place sorting/merging algorithms I found are either
    ///    unstable or rely on enough different elements in each step to encode the
    ///    performed block rearrangements. See also "In-Place Merging Algorithms",
    ///    Denham Coates-Evely, Department of Computer Science, Kings College,
    ///    January 2004 and the references in there.
    ///  - Often "optimal" algorithms are optimal in the number of assignments
    ///    but Interface has only Swap as operation.
    ///
    /// Stable sorts data in ascending order as determined by the Less method,
    /// while keeping the original order of equal elements.
    ///
    /// It makes one call to data.Len to determine n, O(n*log(n)) calls to
    /// data.Less and O(n*log(n)*log(n)) calls to data.Swap.
    #[inline]
    fn sort_stable_reverse(&mut self)
    where
        Self: Sized,
    {
        let n = self.len();
        stable(&mut Reverse(self), n);
    }
}

#[inline]
fn __swap_slice<T>(data: &mut [T], i: usize, j: usize) {
    data.swap(i, j)
}

#[inline]
const fn __slice_len<T>(data: &[T]) -> usize {
    data.len()
}

#[cfg(feature = "alloc")]
impl<T: PartialOrd + core::fmt::Debug> Sort for ::alloc::vec::Vec<T> {
    fn len(&self) -> usize {
        __slice_len(self)
    }

    fn less(&self, i: usize, j: usize) -> bool {
        self[i] < self[j]
    }

    fn swap(&mut self, i: usize, j: usize) {
        __swap_slice(self, i, j);
    }
}

impl<'a, T: PartialOrd + core::fmt::Debug> Sort for &'a mut [T] {
    fn len(&self) -> usize {
        __slice_len(self)
    }

    fn less(&self, i: usize, j: usize) -> bool {
        self[i] < self[j]
    }

    fn swap(&mut self, i: usize, j: usize) {
        __swap_slice(self, i, j);
    }
}

impl<const N: usize, T: PartialOrd + core::fmt::Debug> Sort for [T; N] {
    fn len(&self) -> usize {
        __slice_len(self)
    }

    fn less(&self, i: usize, j: usize) -> bool {
        self[i] < self[j]
    }

    fn swap(&mut self, i: usize, j: usize) {
        __swap_slice(self, i, j);
    }
}

#[cfg(feature = "alloc")]
impl<T: PartialOrd + core::fmt::Debug> Sort for ::alloc::boxed::Box<[T]> {
    fn len(&self) -> usize {
        __slice_len(self)
    }

    fn less(&self, i: usize, j: usize) -> bool {
        self[i] < self[j]
    }

    fn swap(&mut self, i: usize, j: usize) {
        __swap_slice(self, i, j);
    }
}

/// Sorts data[a:b] using insertion sort.
#[inline]
fn insertion_sort(data: &mut impl Sort, a: usize, b: usize) {
    for i in a + 1..b {
        let mut j = i;
        while j > a && data.less(j, j - 1) {
            data.swap(j, j - 1);
            j -= 1;
        }
    }
}

/// Implements the heap property on data[lo:hi].
/// first is an offset into the array where the root of the heap lies.
#[inline]
fn sift_down(data: &mut impl Sort, lo: usize, hi: usize, first: usize) {
    let mut root = lo;
    loop {
        let mut child = 2 * root + 1;
        if child >= hi {
            break;
        }

        if child + 1 < hi && data.less(first + child, first + child + 1) {
            child += 1;
        }

        if !data.less(first + root, first + child) {
            return;
        }

        data.swap(first + root, first + child);
        root = child;
    }
}

#[inline]
fn heap_sort(data: &mut impl Sort, a: usize, b: usize) {
    let first = a;
    let lo = 0;
    let hi = b - a;

    // Build heap with greatest element at top.
    let mut i = (hi - 1) / 2;
    loop {
        sift_down(data, i, hi, first);
        match i.checked_sub(1) {
            Some(v) => i = v,
            None => break,
        }
    }

    // Pop elements, largest first, into end of data.
    let mut i = hi - 1;
    loop {
        data.swap(first, first + i);
        sift_down(data, lo, i, first);
        match i.checked_sub(1) {
            Some(v) => i = v,
            None => break,
        }
    }
}

#[inline]
fn median_of_three(data: &mut impl Sort, m1: usize, m0: usize, m2: usize) {
    // sort 3 elements
    if data.less(m1, m0) {
        data.swap(m1, m0);
    }

    // data[m0] <= data[m1]
    if data.less(m2, m1) {
        data.swap(m2, m1);
        // data[m0] <= data[m2] && data[m1] < data[m2]
        if data.less(m1, m0) {
            data.swap(m1, m0);
        }
    }
    // now data[m0] <= data[m1] <= data[m2]
}

#[inline]
fn swap_range(data: &mut impl Sort, a: usize, b: usize, n: usize) {
    for i in 0..n {
        data.swap(a + i, b + i);
    }
}

#[inline]
fn do_pivot(data: &mut impl Sort, lo: usize, hi: usize) -> (usize, usize) {
    let m = (lo + hi) >> 1;
    if hi - lo > 40 {
        // Tukey ninther, median of three medians of three
        let s = (hi - lo) / 8;
        median_of_three(data, lo, lo + s, lo + 2 * s);
        median_of_three(data, m, m - s, m + s);
        median_of_three(data, hi - 1, hi - 1 - s, hi - 1 - 2 * s);
    }
    median_of_three(data, lo, m, hi - 1);
    // Invariants are:
    //	data[lo] = pivot (set up by ChoosePivot)
    //	data[lo < i < a] < pivot
    //	data[a <= i < b] <= pivot
    //	data[b <= i < c] unexamined
    //	data[c <= i < hi-1] > pivot
    //	data[hi-1] >= pivot
    let pivot = lo;
    let (mut a, mut c) = (lo + 1, hi - 1);
    while a < c && data.less(a, pivot) {
        a += 1;
    }
    let mut b = a;
    loop {
        // data[b] <= pivot
        while b < c && !data.less(pivot, b) {
            b += 1;
        }
        // data[c-1] > pivot
        while b < c && data.less(pivot, c - 1) {
            c -= 1;
        }
        if b >= c {
            break;
        }
        // data[b] > pivot; data[c-1] <= pivot
        data.swap(b, c - 1);
        b += 1;
        c -= 1;
    }

    // If hi-c<3 then there are duplicates (by property of median of nine).
    // Let's be a bit more conservative, and set border to 5.
    let mut protect = hi - c < 5;
    if !protect && hi - c < (hi - lo) / 4 {
        // Lets test some points for equality to pivot
        let mut dups = 0;
        // data[hi-1] = pivot
        if !data.less(pivot, hi - 1) {
            data.swap(c, hi - 1);
            c += 1;
            dups += 1;
        }

        // data[b-1] = pivot
        if !data.less(b - 1, pivot) {
            b -= 1;
            dups += 1;
        }

        // m-lo = (hi-lo)/2 > 6
        // b-lo > (hi-lo)*3/4-1 > 8
        // ==> m < b ==> data[m] <= pivot
        if !data.less(m, pivot) {
            data.swap(m, b - 1);
            b -= 1;
            dups += 1;
        }

        // if at least 2 points are equal to pivot, assume skewed distribution
        protect = dups > 1;
    }

    if protect {
        // Protect against a lot of duplicates
        // Add invariant:
        //	data[a <= i < b] unexamined
        //	data[b <= i < c] = pivot
        loop {
            // data[b] == pivot
            while a < b && !data.less(b - 1, pivot) {
                b -= 1;
            }
            // data[a] < pivot
            while a < b && data.less(a, pivot) {
                a += 1;
            }
            if a >= b {
                break;
            }
            // data[a] == pivot; data[b-1] < pivot
            data.swap(a, b - 1);
            a += 1;
            b -= 1;
        }
    }
    // Swap pivot into middle
    data.swap(pivot, b - 1);
    (b - 1, c)
}

#[inline]
fn quick_sort(data: &mut impl Sort, mut a: usize, mut b: usize, mut max_depth: usize) {
    while b - a > 12 {
        if max_depth == 0 {
            heap_sort(data, a, b);
            return;
        }

        max_depth -= 1;
        let (mlo, mhi) = do_pivot(data, a, b);
        // Avoiding recursion on the larger subproblem guarantees
        // a stack depth of at most lg(b-a).
        if mlo - a < b - mhi {
            quick_sort(data, a, mlo, max_depth);
            a = mhi; // i.e., quickSort(data, mhi, b)
        } else {
            quick_sort(data, mhi, b, max_depth);
            b = mlo; // i.e., quickSort(data, a, mlo)
        }
    }
    if b - a > 1 {
        // Do ShellSort pass with gap 6
        // It could be written in this simplified form cause b-a <= 12
        for i in a + 6..b {
            if data.less(i, i - 6) {
                data.swap(i, i - 6);
            }
        }
        insertion_sort(data, a, b)
    }
}

#[inline]
fn stable(data: &mut impl Sort, n: usize) {
    let mut block_size = 5;
    let (mut a, mut b) = (0, block_size);
    while b <= n {
        insertion_sort(data, a, b);
        a = b;
        b += block_size;
    }

    insertion_sort(data, a, n);
    while block_size < n {
        a = 0;
        b = 2 * block_size;
        while b <= n {
            syn_merge(data, a, a + block_size, b);
            a = b;
            b += 2 * block_size;
        }
        let m = a + block_size;
        if m < n {
            syn_merge(data, a, m, n);
        }
        block_size *= 2;
    }
}

/// Merges the two sorted subsequences data[a:m] and data[m:b] using
/// the SymMerge algorithm from Pok-Son Kim and Arne Kutzner, "Stable Minimum
/// Storage Merging by Symmetric Comparisons", in Susanne Albers and Tomasz
/// Radzik, editors, Algorithms - ESA 2004, volume 3221 of Lecture Notes in
/// Computer Science, pages 714-723. Springer, 2004.
///
/// Let M = m-a and N = b-n. Wolog M < N.
/// The recursion depth is bound by ceil(log(N+M)).
/// The algorithm needs O(M*log(N/M + 1)) calls to data.Less.
/// The algorithm needs O((M+N)*log(M)) calls to data.Swap.
///
/// The paper gives O((M+N)*log(M)) as the number of assignments assuming a
/// rotation algorithm which uses O(M+N+gcd(M+N)) assignments. The argumentation
/// in the paper carries through for Swap operations, especially as the block
/// swapping rotate uses only O(M+N) Swaps.
///
/// symMerge assumes non-degenerate arguments: a < m && m < b.
/// Having the caller check this condition eliminates many leaf recursion calls,
/// which improves performance.
#[inline]
fn syn_merge(data: &mut impl Sort, a: usize, m: usize, b: usize) {
    // Avoid unnecessary recursions of symMerge
    // by direct insertion of data[a] into data[m:b]
    // if data[a:m] only contains one element.
    if m - a == 1 {
        // Use binary search to find the lowest index i
        // such that data[i] >= data[a] for m <= i < b.
        // Exit the search loop with i == b in case no such index exists.
        let mut i = m;
        let mut j = b;
        while i < j {
            let h = (i + j) >> 1;
            if data.less(h, a) {
                i = h + 1;
            } else {
                j = h;
            }
        }

        // Swap values until data[a] reaches the position before i.
        for k in a..i - 1 {
            data.swap(k, k + 1);
        }
        return;
    }

    // Avoid unnecessary recursions of sym_merge
    // by direct insertion of data[m] into data[a:m]
    // if data[m:b] only contains one element.
    if b - m == 1 {
        // Use binary search to find the lowest index i
        // such that data[i] > data[m] for a <= i < m.
        // Exit the search loop with i == m in case no such index exists.
        let mut i = a;
        let mut j = m;
        while i < j {
            let h = (i + j) >> 1;
            if !data.less(m, h) {
                i = h + 1;
            } else {
                j = h;
            }
        }

        // Swap values until data[m] reaches the position i.
        let mut k = m;
        while k > i {
            data.swap(k, k - 1);
            k -= 1;
        }
        return;
    }

    let mid = (a + b) >> 1;
    let n = mid + m;
    let (mut start, mut r) = if m > mid { (n - b, mid) } else { (a, m) };

    let p = n - 1;
    while start < r {
        let c = (start + r) >> 1;
        if !data.less(p - c, c) {
            start = c + 1;
        } else {
            r = c;
        }
    }

    let end = n - start;
    if start < m && m < end {
        rotate(data, start, m, end);
    }

    if a < start && start < mid {
        syn_merge(data, a, start, mid);
    }

    if mid < end && end < b {
        syn_merge(data, mid, end, b);
    }
}

/// Rotates two consecutive blocks u = data[a:m] and v = data[m:b] in data:
/// Data of the form 'x u v y' is changed to 'x v u y'.
/// rotate performs at most b-a many calls to data.Swap,
/// and it assumes non-degenerate arguments: a < m && m < b.
#[inline]
fn rotate(data: &mut impl Sort, a: usize, m: usize, b: usize) {
    let mut i = m - a;
    let mut j = b - m;

    while i != j {
        if i > j {
            swap_range(data, m - i, m, j);
            i -= j;
        } else {
            swap_range(data, m - i, m + j - i, i);
            j -= i;
        }
    }

    // i == j
    swap_range(data, m - i, m, i);
}

/// Returns a threshold at which quicksort should switch
/// to heapsort. It returns 2*ceil(lg(n+1)).
#[inline]
fn max_depth(n: usize) -> usize {
    let mut depth = 0;
    let mut i = n;
    while i > 0 {
        depth += 1;
        i >>= 1;
    }
    depth * 2
}

/// Sort data.
/// It makes one call to `data.len` to determine n and `O(n*log(n))` calls to
/// `data.less` and `data.swap`. The sort is not guaranteed to be stable.
#[inline]
pub fn sort(data: &mut impl Sort) {
    let n = data.len();
    quick_sort(data, 0, n, max_depth(n));
}

/// Sort data (stable).
#[inline]
pub fn sort_stable(data: &mut impl Sort) {
    let n = data.len();
    stable(data, n);
}

/// Sort data in reverse order.
#[inline]
pub fn sort_reverse(data: &mut impl Sort) {
    let n = data.len();
    quick_sort(&mut Reverse(data), 0, n, max_depth(n));
}

/// Sort data in reverse order (stable).
#[inline]
pub fn sort_stable_reverse(data: &mut impl Sort) {
    let n = data.len();
    stable(&mut Reverse(data), n);
}

/// Golang's `sort.Search` in Rust.
#[inline]
pub fn search<F>(n: usize, mut f: F) -> usize
where
    F: FnMut(usize) -> bool,
{
    let mut i = 0;
    let mut j = n;
    while i < j {
        let h = (i + j) >> 1;
        if !f(h) {
            i = h + 1;
        } else {
            j = h;
        }
    }
    i
}

#[cfg(test)]
#[allow(warnings)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::cell::{Cell, RefCell};

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
        Sort::sort(&mut data);
        assert!(Sort::is_sorted(&data));

        let mut data = INTS.to_vec();
        Sort::sort_stable(&mut data);
        assert!(Sort::is_sorted(&data));
    }

    #[test]
    fn test_sort_f64_slice() {
        let mut data = FLOATS.to_vec();
        Sort::sort(&mut data);
        assert!(Sort::is_sorted(&data));

        let mut data = FLOATS.to_vec();
        Sort::sort_stable(&mut data);
        assert!(Sort::is_sorted(&data));
    }

    #[test]
    fn test_sort_string_slice() {
        let mut data = STRINGS.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        Sort::sort(&mut data);
        assert!(Sort::is_sorted(&data));

        let mut data = STRINGS.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        Sort::sort_stable(&mut data);
        assert!(Sort::is_sorted(&data));
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
        Sort::sort(&mut data);
        assert!(Sort::is_sorted(&data));

        let mut data = (0..1000000)
            .map(|_| rand::random::<isize>())
            .collect::<Vec<_>>();
        Sort::sort_stable(&mut data);
        assert!(Sort::is_sorted(&data));
    }

    #[test]
    fn test_reverse_sort_int_slice() {
        let mut data = INTS.to_vec();
        let mut data1 = INTS.to_vec();

        Sort::sort(&mut data);
        Sort::sort_reverse(&mut data1);
        for i in 0..INTS.len() {
            assert_eq!(data[i], data1[INTS.len() - i - 1]);
            if i > data.len() / 2 {
                break;
            }
        }
    }

    #[derive(Debug)]
    struct NonDeterministicTestingData;

    impl Sort for NonDeterministicTestingData {
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
            Sort::sort(&mut NonDeterministicTestingData);
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

    impl Sort for TestingData {
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

    impl Sort for AdversaryTestingData {
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

    impl Sort for Pairs {
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

    const COUNT_OPS_SIZES: &[usize] =
        &[100, 300, 1000, 3000, 10000, 30000, 100000, 300000, 1000000];

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
}
