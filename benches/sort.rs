#![feature(rand)]
#![feature(plugin)]

#![plugin(criterion_macros)]

extern crate criterion;
extern crate rand;
extern crate sort;

use criterion::Bencher;
use rand::*;
use rand::isaac::Isaac64Rng;
use sort::*;
use std::cmp::Ordering;
use std::vec::Vec;

#[criterion]
fn empty(b: &mut Bencher) { b.iter(|| 0) }

#[criterion]
fn stock(b: &mut Bencher) {
    go::<isize, _, _>(b, |xs, cmp| xs.sort_unstable_by(cmp), &mut Isaac64Rng::new_unseeded())
}

#[criterion]
fn smooth(b: &mut Bencher) {
    go(b, smooth::sort_by::<isize, _>, &mut Isaac64Rng::new_unseeded())
}

#[criterion]
fn merge(b: &mut Bencher) {
    go(b, merge::sort_by::<isize, _>, &mut Isaac64Rng::new_unseeded())
}

fn go<T: Ord + Rand, F: Fn(&mut [T], fn(&T, &T) -> Ordering), G: Rng>(b: &mut Bencher, f: F, g: &mut G) {
    let test_size = 1 << 16;
    let mut xs: Vec<T> = g.gen_iter().take(test_size).collect();
    b.iter(|| f(&mut xs, ::std::cmp::Ord::cmp))
}
