use std::collections::HashMap;

use super::{solve_2, Reaction, ReactionPart, Reactions, REACTIONS};

pub fn solve_1(reactions: &Reactions, part: ReactionPart) -> u64 {
    let mut ore = 0;

    let mut pool: HashMap<&str, u64> = HashMap::new();
    let mut work = vec![part];
    while let Some((quantity, component)) = work.pop() {
        match reactions.get(component) {
            None => ore += quantity,
            Some(Reaction {
                quantity: recipe_quantity,
                components,
            }) => {
                let pool_quantity = pool.entry(component).or_default();
                if *pool_quantity >= quantity {
                    *pool_quantity -= quantity;
                } else {
                    let requested = quantity - *pool_quantity;
                    let multiplier = requested / recipe_quantity
                        + if requested % recipe_quantity > 0 {
                            1
                        } else {
                            0
                        };
                    *pool_quantity = multiplier * recipe_quantity + *pool_quantity - quantity;
                    components
                        .iter()
                        .for_each(|(q, c)| work.push((q * multiplier, c)));
                }
            }
        }
    }

    ore
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
    fn test_solve_1_simple() {
        assert_eq!(solve_1(&parse(r"10 ORE => 1 FUEL"), (1, "FUEL"),), 10)
    }

    #[test]
    fn test_solve_1_simple_bis() {
        assert_eq!(
            solve_1(
                &parse(
                    r"2 A => 1 FUEL
10 ORE => 1 A
"
                ),
                (1, "FUEL"),
            ),
            20
        )
    }

    #[test]
    fn test_solve_1_simple_sum() {
        assert_eq!(
            solve_1(
                &parse(
                    r"1 A, 1 B => 1 FUEL
10 ORE => 1 A
20 ORE => 1 B
"
                ),
                (1, "FUEL"),
            ),
            30
        )
    }

    #[test]
    fn test_solve_1_simple_sum_bis() {
        assert_eq!(
            solve_1(
                &parse(
                    r"1 A, 1 C => 1 FUEL
10 ORE => 1 A
1 A, 1 B => 2 C
10 ORE => 2 B
"
                ),
                (1, "FUEL"),
            ),
            30
        )
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
