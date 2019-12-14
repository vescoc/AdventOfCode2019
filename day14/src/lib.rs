#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::cmp::Ordering;
use std::collections::HashMap;

use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)\s+([A-Z]+)").unwrap();
    static ref REACTIONS: Reactions<'static> = parse(include_str!("../data.txt"));
}

type Reactions<'a> = HashMap<&'a str, Reaction<'a>>;
type ReactionPart<'a> = (u64, &'a str);

#[derive(Debug, PartialEq)]
struct Reaction<'a> {
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

fn parse(data: &str) -> Reactions {
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

fn substitute_one<'a>(
    reactions: &Reactions<'a>,
    part: &ReactionPart<'a>,
    pool: &mut HashMap<&'a str, u64>,
) -> Vec<ReactionPart<'a>> {
    if let Some(reaction) = &reactions.get(part.1) {
        let target_value = part.0;

        let pool_value = pool.get(part.1).copied().unwrap_or_default();

        let quantity = reaction.quantity;

        let (multiplier, new_pool) = if target_value < pool_value {
            (0, pool_value - target_value)
        } else {
            let target_to_produce = target_value - pool_value;
            let (m, r) = (target_to_produce / quantity, target_to_produce % quantity);
            let m = if r > 0 { m + 1 } else { m };
            (m, quantity * m + pool_value - target_value)
        };

        pool.insert(part.1, new_pool);

        reaction
            .components
            .iter()
            .map(|(quantity, component)| (quantity * multiplier, *component))
            .collect()
    } else {
        vec![*part]
    }
}

fn substitute<'a>(
    reactions: &Reactions<'a>,
    parts: &[ReactionPart<'a>],
    pool: &mut HashMap<&'a str, u64>,
) -> Vec<ReactionPart<'a>> {
    parts
        .iter()
        .flat_map(|part| substitute_one(reactions, part, pool))
        .fold(
            HashMap::<&str, u64>::new(),
            |mut acc, (quantity, component)| {
                *acc.entry(component).or_default() += quantity;
                acc
            },
        )
        .into_iter()
        .map(|(component, quantity)| (quantity, component))
        .collect()
}

fn solve_1(reactions: &Reactions, part: ReactionPart) -> u64 {
    let mut pool = HashMap::new();
    let mut parts = vec![part];
    loop {
        parts = substitute(reactions, &parts, &mut pool);
        if parts.len() == 1 {
            break;
        }
    }

    let part = parts[0];
    assert_eq!(part.1, "ORE");

    part.0
}

#[allow(clippy::unreadable_literal)]
fn solve_2(reactions: &Reactions) -> u64 {
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

pub fn part_1() -> u64 {
    solve_1(&REACTIONS, (1, "FUEL"))
}

pub fn part_2() -> u64 {
    solve_2(&REACTIONS)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    lazy_static! {
        static ref REACTIONS_EXAMPLE_1: Reactions<'static> = parse(
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
        static ref REACTIONS_EXAMPLE_2: Reactions<'static> = parse(
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
        static ref REACTIONS_EXAMPLE_3: Reactions<'static> = parse(
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
    fn test_substitute_one() {
        let reactions = parse(
            r"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL",
        );

        let mut pool = vec![("FUEL", 1)].into_iter().collect();
        assert_eq!(
            substitute_one(&reactions, &(2, "FUEL"), &mut pool),
            vec![(7, "A"), (1, "E")]
        );

        assert_eq!(pool, vec![("FUEL", 0)].into_iter().collect());
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_substitute_one_ORE() {
        let reactions = parse(
            r"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL",
        );

        let mut pool = vec![("FUEL", 1)].into_iter().collect();
        assert_eq!(
            substitute_one(&reactions, &(2, "ORE"), &mut pool),
            vec![(2, "ORE")]
        );

        assert_eq!(pool, vec![("FUEL", 1)].into_iter().collect());
    }

    #[test]
    fn test_substitute() {
        let reactions = parse(
            r"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL",
        );

        let mut pool = vec![("E", 1)].into_iter().collect();

        let mut result = substitute(&reactions, &[(2, "E"), (2, "D")], &mut pool);
        result.sort_by(|(_, a), (_, b)| a.cmp(b));

        assert_eq!(result, vec![(21, "A"), (2, "C"), (1, "D")]);
        assert_eq!(pool, vec![("E", 0), ("D", 0)].into_iter().collect());
    }

    #[test]
    fn test_example_1_1() {
        assert_eq!(
            solve_1(
                &parse(
                    r"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"
                ),
                (1, "FUEL"),
            ),
            31
        )
    }

    #[test]
    fn test_example_1_2() {
        assert_eq!(
            solve_1(
                &parse(
                    r"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL"
                ),
                (1, "FUEL"),
            ),
            165
        )
    }

    #[test]
    fn test_example_1_3() {
        assert_eq!(solve_1(&REACTIONS_EXAMPLE_1, (1, "FUEL"),), 13312)
    }

    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_example_1_4() {
        assert_eq!(solve_1(&REACTIONS_EXAMPLE_2, (1, "FUEL"),), 180697)
    }

    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_example_1_5() {
        assert_eq!(solve_1(&REACTIONS_EXAMPLE_3, (1, "FUEL"),), 2210736)
    }

    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_example_2_1() {
        assert_eq!(solve_2(&REACTIONS_EXAMPLE_1), 82892753);
    }

    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_example_2_2() {
        assert_eq!(solve_2(&REACTIONS_EXAMPLE_2), 5586022);
    }

    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_example_2_3() {
        assert_eq!(solve_2(&REACTIONS_EXAMPLE_3), 460664);
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
