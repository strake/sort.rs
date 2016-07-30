#![no_std]

#![cfg_attr(test, feature(plugin))]

#![cfg_attr(test, plugin(quickcheck_macros))]

#[cfg(test)] extern crate quickcheck;
#[cfg(test)] extern crate std;

pub mod smooth;

mod u2size;
