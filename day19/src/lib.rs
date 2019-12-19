#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use intcode;

lazy_static! {
    static ref PROGRAM: Vec<intcode::Memory> = intcode::parse(include_str!("../data.txt"));
}

fn check(x: usize, y: usize) -> bool {
    let mut cpu = intcode::CPU::new(PROGRAM.to_vec(), 0, Some(x as intcode::Memory));

    loop {
        match cpu.run().expect("got error") {
            intcode::Run::NeedInput => {
                cpu.set_input(Some(y as intcode::Memory));
                match cpu.run().expect("got error") {
                    intcode::Run::Output(value) => {
                        break match value {
                            0 => false,
                            1 => true,
                            r => panic!("invalid response: {}", r),
                        }
                    }
                    _ => unreachable!(),
                }
            }
            s => panic!("invalid state: {:?} as ({}, {})", s, x, y),
        }
    }
}

pub fn part_1() -> usize {
    (0..50)
        .map(|x| {
            (0..50)
                .map(|y| if check(x, y) { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

pub fn part_2() -> usize {
    let (mut x, mut y) = (1, 1);
    loop {
        while !check(x + 99, y) {
            y += 1;
        }
        while !check(x, y + 99) {
            x += 1;
        }
        if check(x + 99, y) {
            break x * 10000 + y;
        } else {
            x += 1
        }
    }
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
