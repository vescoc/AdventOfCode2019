use lazy_static::lazy_static;

use std::collections::HashSet;

mod bugs;
mod rbugs;

lazy_static! {
    static ref DATA: &'static str = include_str!("../data.txt");
}

fn solve_1(input: &str) -> u32 {
    let bugs = input.parse::<bugs::Bugs>().unwrap();
    let mut seen = HashSet::<bugs::Bugs>::new();
    for b in bugs {
        if seen.contains(&b) {
            return b.value();
        } else {
            seen.insert(b);
        }
    }

    unreachable!()
}

fn solve_2(input: &str, minutes: usize) -> usize {
    input
        .parse::<rbugs::Bugs>()
        .unwrap()
        .nth(minutes - 1)
        .unwrap()
}

pub fn part_1() -> u32 {
    solve_1(&DATA)
}

pub fn part_2() -> usize {
    solve_2(&DATA, 200)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_example_1_1() {
        assert_eq!(solve_1(include_str!("../example.txt")), 2129920);
    }

    #[test]
    fn test_example_1_0() {
        let mut bugs: bugs::Bugs = include_str!("../example.txt").parse().unwrap();

        assert_eq!(
            format!("{}", bugs.next().unwrap()),
            r"#..#.
####.
###.#
##.##
.##.."
        );
    }

    #[test]
    fn test_example_2_1() {
        assert_eq!(solve_2(include_str!("../example.txt"), 10), 99);
    }
}
