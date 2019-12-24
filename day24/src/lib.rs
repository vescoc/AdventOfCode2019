#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::collections::HashSet;

mod bugs;
mod rbugs;

lazy_static! {
    static ref BUGS: bugs::Bugs = include_str!("../data.txt").parse().unwrap();
}

fn solve_1(bugs: &bugs::Bugs) -> u32 {
    let mut seen = HashSet::<bugs::Bugs>::new();
    for b in bugs.to_owned() {
        if seen.contains(&b) {
            return b.value();
        } else {
            seen.insert(b);
        }
    }

    unreachable!()
}

pub fn part_1() -> u32 {
    solve_1(&BUGS)
}

pub fn part_2() -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_example_1_1() {
        let bugs = r"....#
#..#.
#..##
..#..
#...."
            .parse()
            .unwrap();

        assert_eq!(solve_1(&bugs), 2129920);
    }

    #[test]
    fn test_example_1_0() {
        let mut bugs: bugs::Bugs = r"....#
#..#.
#..##
..#..
#...."
            .parse()
            .unwrap();

        assert_eq!(
            format!("{}", bugs.next().unwrap()),
            r"#..#.
####.
###.#
##.##
.##.."
        );
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
