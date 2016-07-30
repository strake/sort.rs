#![allow(dead_code)]

#[inline]
pub fn is_sorted<T: Ord>(xs: &[T]) -> bool { (1..xs.len()).all(|k| xs[k-1] <= xs[k]) }
