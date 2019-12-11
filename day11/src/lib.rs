#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::str::FromStr;

pub mod simple;

const SEPARATOR: char = ',';

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../data.txt");
    pub static ref DATA: Vec<i128> = parse(&INPUT);
}

pub fn parse<T: FromStr>(data: &str) -> Vec<T> {
    data.trim()
        .split(SEPARATOR)
        .map(|s| s.parse().unwrap_or_else(|_| panic!("cannot parse: {}", s)))
        .collect()
}
