#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::str::FromStr;

pub mod simple;
//pub mod generic;

lazy_static! {
    pub static ref DATA: Vec<i128> = parse(include_str!("../data.txt"));
}

const SEPARATOR: char = ',';

pub fn parse<T: FromStr>(data: &str) -> Vec<T> {
    data.trim()
        .split(SEPARATOR)
        .map(|s| {
            s.parse()
                .unwrap_or_else(|_| panic!("cannot parse: {}", s))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_same_results_part_1() {
    //     assert_eq!(simple::part_1(), generic::part_1());
    // }

    // #[test]
    // fn test_same_results_part_2() {
    //     assert_eq!(simple::part_2(), generic::part_2());
    // }
}
