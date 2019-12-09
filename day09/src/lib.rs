#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

pub mod intcode;

use intcode::{parse, Memory, Step, CPU};

lazy_static! {
    pub static ref DATA: Vec<Memory> = parse(include_str!("../data.txt"));
}

pub fn part_1() -> Memory {
    let mut cpu = CPU::new(DATA.to_owned(), 0, Some(1));

    let mut output = vec![];
    loop {
        match cpu.step() {
            Ok(Step::NeedInput) => panic!("invalid input request"),
            Ok(Step::Continue) => {}
            Ok(Step::Output(value)) => output.push(value),
            Ok(Step::Halt) => break,
            state => panic!("invalid state {:?}", state),
        }
    }

    output.pop().unwrap().to_owned()
}

pub fn part_2() -> Memory {
    let mut cpu = CPU::new(DATA.to_owned(), 0, Some(2));

    let mut output = vec![];
    loop {
        match cpu.step() {
            Ok(Step::NeedInput) => panic!("invalid input request"),
            Ok(Step::Continue) => {}
            Ok(Step::Output(value)) => output.push(value),
            Ok(Step::Halt) => break,
            state => panic!("invalid state {:?}", state),
        }
    }

    output.pop().unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        let mut cpu = CPU::new(DATA.to_owned(), 0, Some(1));

        let mut output = vec![];
        loop {
            match cpu.run() {
                Ok(Step::NeedInput) => panic!("invalid input request"),
                Ok(Step::Output(value)) => output.push(value),
                Ok(Step::Halt) => break,
                state => panic!("invalid state {:?}", state),
            }
        }

        assert_eq!(output.len(), 1);
    }

    #[test]
    fn test_part_2() {
        let mut cpu = CPU::new(DATA.to_owned(), 0, Some(2));

        let mut output = vec![];
        loop {
            match cpu.run() {
                Ok(Step::NeedInput) => panic!("invalid input request"),
                Ok(Step::Output(value)) => output.push(value),
                Ok(Step::Halt) => break,
                state => panic!("invalid state {:?}", state),
            }
        }

        assert_eq!(output.len(), 1);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part_1);
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part_2);
    }
}
