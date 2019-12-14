use std::collections::HashMap;

use super::{solve_2, ReactionPart, Reactions, REACTIONS};

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

pub fn solve_1(reactions: &Reactions, part: ReactionPart) -> u64 {
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

pub fn part_1() -> u64 {
    solve_1(&REACTIONS, (1, "FUEL"))
}

pub fn part_2() -> u64 {
    solve_2(&REACTIONS, solve_1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    use crate::{
        parse,
        tests::{REACTIONS_EXAMPLE_1, REACTIONS_EXAMPLE_2, REACTIONS_EXAMPLE_3},
    };

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
        assert_eq!(solve_2(&REACTIONS_EXAMPLE_1, solve_1), 82892753);
    }

    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_example_2_2() {
        assert_eq!(solve_2(&REACTIONS_EXAMPLE_2, solve_1), 5586022);
    }

    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_example_2_3() {
        assert_eq!(solve_2(&REACTIONS_EXAMPLE_3, solve_1), 460664);
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
