#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::cmp::Ordering;
use std::collections::HashMap;

use regex::Regex;

pub mod fast;
pub mod simple;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)\s+([A-Z]+)").unwrap();
    static ref REACTIONS: Reactions<'static> = parse(include_str!("../data.txt"));
}

pub type Reactions<'a> = HashMap<&'a str, Reaction<'a>>;
pub type ReactionPart<'a> = (u64, &'a str);

#[derive(Debug, PartialEq)]
pub struct Reaction<'a> {
    quantity: u64,
    components: Vec<ReactionPart<'a>>,
}

impl<'a> Reaction<'a> {
    fn new(quantity: u64, components: Vec<ReactionPart<'a>>) -> Self {
        Self {
            quantity,
            components,
        }
    }
}

fn parse_part(part: &str) -> ReactionPart {
    let cap = RE.captures(part.trim()).expect("invalid part");

    (
        cap[1].parse().expect("number invalid"),
        cap.get(2).unwrap().as_str(),
    )
}

pub fn parse(data: &str) -> Reactions {
    data.lines()
        .map(|l| {
            let mut parts = l.split("=>");
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .map(|(lhs, rhs)| {
            let rhs = parse_part(rhs);
            (
                rhs.1,
                Reaction::new(
                    rhs.0,
                    lhs.split(',').map(|p| parse_part(p)).collect::<Vec<_>>(),
                ),
            )
        })
        .collect()
}

#[allow(clippy::unreadable_literal)]
fn solve_2(reactions: &Reactions, solve_1: fn(&Reactions, ReactionPart<'static>) -> u64) -> u64 {
    const TARGET: u64 = 1000000000000;

    let step = solve_1(reactions, (1, "FUEL"));

    let mut lower = TARGET / step;
    loop {
        match TARGET.cmp(&solve_1(reactions, (lower, "FUEL"))) {
            Ordering::Equal => return lower,
            Ordering::Greater => break,
            Ordering::Less => lower /= 10,
        }
    }

    let mut upper = lower * 10;
    loop {
        match TARGET.cmp(&solve_1(reactions, (upper, "FUEL"))) {
            Ordering::Equal => return upper,
            Ordering::Less => break,
            Ordering::Greater => upper *= 10,
        }
    }

    while upper - lower > 1 {
        let middle = (upper - lower) / 2 + lower;

        match TARGET.cmp(&solve_1(reactions, (middle, "FUEL"))) {
            Ordering::Equal => return middle,
            Ordering::Greater => lower = middle,
            Ordering::Less => upper = middle,
        }
    }

    lower
}

#[cfg(test)]
pub mod tests {
    use super::*;

    lazy_static! {
        pub static ref REACTIONS_EXAMPLE_1: Reactions<'static> = parse(
            r"157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
        );
        pub static ref REACTIONS_EXAMPLE_2: Reactions<'static> = parse(
            r"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF"
        );
        pub static ref REACTIONS_EXAMPLE_3: Reactions<'static> = parse(
            r"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX"
        );
    }

    #[test]
    fn test_parse_part() {
        assert_eq!(parse_part("10 ORE"), (10, "ORE"));
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(
                r"10 ORE => 1 A
9 ORE => 2 B"
            ),
            vec![
                ("A", Reaction::new(1, vec![(10, "ORE")])),
                ("B", Reaction::new(2, vec![(9, "ORE")])),
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn same_results_part_1() {
        assert_eq!(simple::part_1(), fast::part_1());
    }

    #[test]
    fn same_results_part_2() {
        assert_eq!(simple::part_1(), fast::part_1());
    }
}
