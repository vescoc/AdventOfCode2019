#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use intcode;

mod path;

lazy_static! {
    static ref PROGRAM: Vec<intcode::Memory> = intcode::parse(include_str!("../data.txt"));
    static ref OUTPUT: String = {
        let mut cpu = intcode::CPU::new(PROGRAM.to_vec(), 0, None);
        let mut output: Vec<char> = vec![];
        loop {
            match cpu.run().expect("invalid cpu run state") {
                intcode::Run::Halt => break,
                intcode::Run::NeedInput => unreachable!(),
                intcode::Run::Output(value) => output.push(char::from(value as u8)),
            }
        }

        output.iter().collect()
    };
}

fn solve_1(map: &str) -> i32 {
    map.parse::<path::Path>()
        .unwrap()
        .intersections()
        .map(|(x, y)| x * y)
        .sum()
}

pub fn part_1() -> i32 {
    solve_1(&OUTPUT)
}

pub fn part_2() -> i64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    lazy_static! {
        static ref EXAMPLE: &'static str = r"..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..";
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(&EXAMPLE), 76);
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
