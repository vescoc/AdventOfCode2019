#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

pub mod functional;
pub mod iterator;
pub mod simple;

lazy_static! {
    pub static ref DATA: Vec<u32> = include_str!("../data.txt")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
}

pub fn part<I: IntoIterator<Item = u32>>(masses: &[u32], f: fn(u32) -> I) -> u32 {
    masses.iter().copied().flat_map(f).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_results_1() {
        let n = part(&DATA, simple::calculate_fuel);
        let i = part(&DATA, iterator::calculate_fuel);
        let f = part(&DATA, functional::calculate_fuel);

        assert_eq!(n, i);
        assert_eq!(n, f);
    }

    #[test]
    fn same_results_2() {
        let n = part(&DATA, simple::calculate_total_fuel);
        let i = part(&DATA, iterator::calculate_total_fuel);
        let f = part(&DATA, functional::calculate_total_fuel);

        assert_eq!(n, i);
        assert_eq!(n, f);
    }
}
