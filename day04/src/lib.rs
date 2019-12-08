#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

pub mod functional;
pub mod simple;

lazy_static! {
    pub static ref DATA: Vec<u32> = include_str!("../data.txt")
        .trim()
        .split('-')
        .map(|v| v.parse().unwrap())
        .collect();
}

pub fn part(f: fn(u32) -> bool) -> usize {
    (DATA[0]..=DATA[1]).filter(|&n| f(n)).count()
}
