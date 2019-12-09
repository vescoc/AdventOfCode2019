// NO COMPILE!

pub mod intcode;

#[allow(unused_imports)]
use crate::DATA;

#[allow(unused_imports)]
use intcode::{Step, CPU};

pub fn part_1<Memory>() -> Memory
    where Memory: ToOwned
{
    // let mut cpu: CPU = CPU::new(DATA.to_owned(), 0, Some(1));

    // let mut output: Vec<Memory> = vec![];
    // loop {
    //     match cpu.step() {
    //         Ok(Step::NeedInput) => panic!("invalid input request"),
    //         Ok(Step::Continue) => {}
    //         Ok(Step::Output(value)) => output.push(value),
    //         Ok(Step::Halt) => break,
    //         state => panic!("invalid state {:?}", state),
    //     }
    // }

    // output.pop().unwrap().to_owned();

    unimplemented!()
}

pub fn part_2<Memory>() -> Memory {
    // let mut cpu = CPU::new(DATA.to_owned(), 0, Some(2));

    // let mut output = vec![];
    // loop {
    //     match cpu.step() {
    //         Ok(Step::NeedInput) => panic!("invalid input request"),
    //         Ok(Step::Continue) => {}
    //         Ok(Step::Output(value)) => output.push(value),
    //         Ok(Step::Halt) => break,
    //         state => panic!("invalid state {:?}", state),
    //     }
    // }

    // output.pop().unwrap().to_owned()

    unimplemented!()
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use test::Bencher;

    // #[test]
    // fn test_part_1() {
    //     let mut cpu = CPU::new(DATA.to_owned(), 0, Some(1));

    //     let mut output = vec![];
    //     loop {
    //         match cpu.run() {
    //             Ok(Step::NeedInput) => panic!("invalid input request"),
    //             Ok(Step::Output(value)) => output.push(value),
    //             Ok(Step::Halt) => break,
    //             state => panic!("invalid state {:?}", state),
    //         }
    //     }

    //     assert_eq!(output.len(), 1);
    // }

    // #[test]
    // fn test_part_2() {
    //     let mut cpu = CPU::new(DATA.to_owned(), 0, Some(2));

    //     let mut output = vec![];
    //     loop {
    //         match cpu.run() {
    //             Ok(Step::NeedInput) => panic!("invalid input request"),
    //             Ok(Step::Output(value)) => output.push(value),
    //             Ok(Step::Halt) => break,
    //             state => panic!("invalid state {:?}", state),
    //         }
    //     }

    //     assert_eq!(output.len(), 1);
    // }

    // #[bench]
    // fn bench_part_1(b: &mut Bencher) {
    //     b.iter(part_1::<i128>);
    // }

    // #[bench]
    // fn bench_part_2(b: &mut Bencher) {
    //     b.iter(part_2::<i128>);
    // }
}
