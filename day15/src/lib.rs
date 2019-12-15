#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use intcode;

lazy_static! {
    pub static ref PROGRAM: Vec<intcode::Memory> = intcode::parse(include_str!("../data.txt"));
}

pub mod search;

fn solve_1(program: &[intcode::Memory]) -> usize {
    let mut search = search::Search::new(program, |v| v == search::Tile::OxygenSystem);
    loop {
        match search.step().expect("no solution...") {
            search::Step::Found(_, m, _) => break m.len(),
            search::Step::Searching(_, _) => {}
        }
    }
}

fn solve_2(program: &[intcode::Memory]) -> usize {
    let mut search = search::Search::new(program, |v| v == search::Tile::OxygenSystem);
    'outher: loop {
        match search.step().expect("no solution...") {
            search::Step::Found(p, _, cpu) => {
                let mut search = search::Search::new_from_cpu(cpu, p, |_| false);
                loop {
                    if let Err(search::Error::NotFound(depth)) = search.step() {
                        break 'outher depth + 1;
                    }
                }
            }
            search::Step::Searching(_, _) => {}
        }
    }
}

pub fn part_1() -> usize {
    solve_1(&PROGRAM)
}

pub fn part_2() -> usize {
    solve_2(&PROGRAM)
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
