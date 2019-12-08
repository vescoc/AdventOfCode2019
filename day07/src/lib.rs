#![feature(test)]
extern crate test;    

#[macro_use]
extern crate lazy_static;

pub mod intcode;
pub mod simple;
pub mod spawn;

use intcode::{Memory, parse};

lazy_static! {
    pub static ref DATA: Vec<Memory> = parse(include_str!("../data.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part_1_impls() {
        assert_eq!(simple::part_1(), spawn::part_1());
    }

    #[test]
    fn test_part_2_impls() {
        assert_eq!(simple::part_2(), spawn::part_2());
    }
}
