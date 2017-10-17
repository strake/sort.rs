use core::cmp::*;
use core::iter::*;
use core::{mem, ptr};

/// Inserts `v[0]` into pre-sorted sequence `v[1..]` so that whole `v[..]` becomes sorted.
///
/// This is the integral subroutine of insertion sort.
#[inline]
fn insert_head<T, F: Fn(&T, &T) -> bool>(v: &mut [T], less: F) {
    if v.len() >= 2 && less(&v[1], &v[0]) {
        unsafe {
            // There are three ways to implement insertion here:
            //
            // 1. Swap adjacent elements until the first one gets to its final destination.
            //    However, this way we copy data around more than is necessary. If elements are big
            //    structures (costly to copy), this method will be slow.
            //
            // 2. Iterate until the right place for the first element is found. Then shift the
            //    elements succeeding it to make room for it and finally place it into the
            //    remaining hole. This is a good method.
            //
            // 3. Copy the first element into a temporary variable. Iterate until the right place
            //    for it is found. As we go along, copy every traversed element into the slot
            //    preceding it. Finally, copy data from the temporary variable into the remaining
            //    hole. This method is very good. Benchmarks demonstrated slightly better
            //    performance than with the 2nd method.
            //
            // All methods were benchmarked, and the 3rd showed best results. So we chose that one.
            let mut tmp = mem::ManuallyDrop::new(ptr::read(&v[0]));

            // Intermediate state of the insertion process is always tracked by `hole`, which
            // serves two purposes:
            // 1. Protects integrity of `v` from panics in `less`.
            // 2. Fills the remaining hole in `v` in the end.
            //
            // Panic safety:
            //
            // If `less` panics at any point during the process, `hole` will get dropped and
            // fill the hole in `v` with `tmp`, thus ensuring that `v` still holds every object it
            // initially held exactly once.
            let mut hole = InsertionHole {
                src: &mut *tmp,
                dest: &mut v[1],
            };
            ptr::copy_nonoverlapping(&v[1], &mut v[0], 1);

            for i in 2..v.len() {
                if !less(&v[i], &*tmp) { break; }
                ptr::copy_nonoverlapping(&v[i], &mut v[i - 1], 1);
                hole.dest = &mut v[i];
            }
            // `hole` gets dropped and thus copies `tmp` into the remaining hole in `v`.
        }
    }

    // When dropped, copies from `src` into `dest`.
    struct InsertionHole<T> {
        src: *mut T,
        dest: *mut T,
    }

    impl<T> Drop for InsertionHole<T> {
        fn drop(&mut self) {
            unsafe { ptr::copy_nonoverlapping(self.src, self.dest, 1); }
        }
    }
}

#[inline]
pub fn sort_by<T, Cmp: Fn(&T, &T) -> Ordering>(xs: &mut [T], cmp: Cmp) {
    go(xs, &|a, b| Ordering::Less == cmp(a, b))
}

fn go<T, Less: Fn(&T, &T) -> bool>(xs: &mut [T], less: &Less) {
    const MAX_INSERTION: usize = 20;

    let n = xs.len();

    if MAX_INSERTION >= n {
        if 1 < n { for i in (0..n-1).rev() { insert_head(&mut xs[i..], less); } }
        return;
    }

    let m = n >> 1;
    go(&mut xs[0..m], less);
    go(&mut xs[m..],  less);
    merge(xs, m, less);
}

fn merge<T, Less: Fn(&T, &T) -> bool>(mut xs: &mut [T], mut m: usize, less: &Less) {
    while m != 0 && m != xs.len() {
        let (mut i, mut j) = (m, m);
        while i > 0 && j < xs.len() && less(&xs[j], &xs[i-1]) { i -= 1; j += 1; }
        xs[i..j].rotate_left(j-m);
        let (a, b) = {xs}.split_at_mut(m);
        if a.len() >= b.len() {
            merge(b, j-m, less);
            xs = a;
            m = i;
        } else {
            merge(a, i, less);
            xs = b;
            m = j-m;
        }
    }
}

#[cfg(test)] mod tests {
    use quickcheck::TestResult;
    use std::vec::*;

    use super::*;
    use util::is_sorted;

    #[quickcheck] fn test_sort(xv: Vec<isize>) -> TestResult {
        let mut yv: Vec<_> = xv.iter().enumerate().map(|(k, a)| (a, k)).collect();
        sort_by(&mut yv, |&(a, _), &(b, _)| Ord::cmp(&a, &b));
        TestResult::from_bool(is_sorted(&yv))
    }
}
