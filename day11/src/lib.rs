#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use intcode;

pub mod simple;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../data.txt");
    pub static ref DATA: Vec<i128> = intcode::parse(&INPUT);
}
