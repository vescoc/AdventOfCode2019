#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

pub mod generic;

const ASTEROID: char = '#';
const EMPTY_SPACE: char = '.';

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../data.txt");
}
