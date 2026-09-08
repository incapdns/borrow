#![no_std]
#![deny(unsafe_code)]

use borrow::partial as p;
use borrow::traits::*;

#[derive(borrow::Partial)]
#[module(crate)]
pub struct Pair {
    pub left: u32,
    pub right: u32,
}

fn increment(pair: p!(&<mut left, mut right> Pair)) {
    **pair.left += 1;
    **pair.right += 1;
}

pub fn increment_both(pair: &mut Pair) {
    increment(p!(&mut pair));
}
