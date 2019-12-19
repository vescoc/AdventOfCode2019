#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use intcode;

lazy_static! {
    static ref PROGRAM: Vec<intcode::Memory> = intcode::parse(include_str!("../data.txt"));
}

pub fn part_1() -> usize {
    (0 as intcode::Memory .. 50 as intcode::Memory)
	.map(|x| {
	    (0 as intcode::Memory .. 50 as intcode::Memory).map(|y| {
		let mut cpu = intcode::CPU::new(PROGRAM.to_vec(), 0, Some(x));
		
		loop {
		    match cpu.run().expect("got error") {
			intcode::Run::NeedInput => {
			    cpu.set_input(Some(y));
			    match cpu.run().expect("got error") {
				intcode::Run::Output(value) => break value as usize,
				_ => unreachable!(),
			    }
			}
			s => panic!("invalid state: {:?} as ({}, {})", s, x, y),
		    }
		}
	    })
		.sum::<usize>()
	})
	.sum()	     
}

pub fn part_2() -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part_1);
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part_2);
    }
}
