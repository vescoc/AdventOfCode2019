use lazy_static::lazy_static;

use regex::Regex;
use std::str::FromStr;

lazy_static! {
    static ref DATA: &'static str = include_str!("../data.txt");
    static ref DEAL_WITH_INCREMENT_RE: Regex = Regex::new(r"deal with increment (\d+)").unwrap();
    static ref CUT_RE: Regex = Regex::new(r"cut ((:?-)?\d+)").unwrap();
    static ref DEAL_INTO_NEW_STACK: &'static str = r"deal into new stack";
}

#[derive(Debug, Copy, Clone)]
enum Technique {
    DealWithIncrement(i128),
    Cut(i128),
    DealIntoNewStack,
}

#[derive(Debug)]
struct Techniques<T>(Vec<T>);

impl FromStr for Technique {
    type Err = &'static str;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        fn invalid_number<E>(_: E) -> &'static str {
            "invalid number"
        }

        if let Some(cap) = DEAL_WITH_INCREMENT_RE.captures(data) {
            Ok(Technique::DealWithIncrement(
                cap[1].parse().map_err(invalid_number)?,
            ))
        } else if let Some(cap) = CUT_RE.captures(data) {
            Ok(Technique::Cut(cap[1].parse().map_err(invalid_number)?))
        } else if data == *DEAL_INTO_NEW_STACK {
            Ok(Technique::DealIntoNewStack)
        } else {
            Err("invalid line")
        }
    }
}

impl<T: FromStr> FromStr for Techniques<T> {
    type Err = <T as FromStr>::Err;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        Ok(Techniques(
            data.lines()
                .map(|line| line.parse::<T>())
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

fn mod_inverse(a: i128, n: i128) -> Option<i128> {
    let mut t = 0;
    let mut r = n;
    let mut newt = 1;
    let mut newr = a;

    while newr != 0 {
        let q = r / newr;
        (t, newt) = (newt, t - q * newt);
        (r, newr) = (newr, r - q * newr);
    }

    if r > 1 {
        None
    } else if t < 0 {
        Some(t + n)
    } else {
        Some(t)
    }
}

fn mod_pow(mut a: i128, mut b: i128, n: i128) -> i128 {
    let mut r = if n == 1 { 0 } else { 1 };
    while b > 0 {
        if b & 1 != 0 {
            r = (r * a).rem_euclid(n);
        }
        b >>= 1;
        a = (a * a).rem_euclid(n);
    }
    r
}

fn card_position(n: i128, techniques: &Techniques<Technique>, card: i128) -> i128 {
    techniques
        .0
        .iter()
        .fold(card, |acc, technique| match technique {
            Technique::DealWithIncrement(increment) => (acc * increment).rem_euclid(n),
            Technique::Cut(cut) => (acc - cut).rem_euclid(n),
            Technique::DealIntoNewStack => (-acc - 1).rem_euclid(n),
        })
}

fn linear_compose(n: i128, techniques: &Techniques<Technique>) -> (i128, i128) {
    techniques
        .0
        .iter()
        .rev()
        .fold((1, 0), |(a1, b1), technique| match technique {
            Technique::DealWithIncrement(increment) => {
                let a = mod_inverse(*increment, n).unwrap();
                ((a * a1).rem_euclid(n), (a * b1).rem_euclid(n))
            }
            Technique::Cut(cut) => (a1, (b1 + cut).rem_euclid(n)),
            Technique::DealIntoNewStack => ((-a1).rem_euclid(n), (-b1 - 1).rem_euclid(n)),
        })
}

fn solve_1(input: &str) -> i128 {
    card_position(10007, &input.parse().unwrap(), 2019)
}

fn solve_2(input: &str) -> i128 {
    const N: i128 = 119_315_717_514_047;
    const TIMES: i128 = 101_741_582_076_661;
    const C: i128 = 2020;

    let (a, b) = linear_compose(N, &input.parse().unwrap());

    let a_pow = mod_pow(a, TIMES, N);

    let p1 = a_pow * C;
    let p2 = (b * (mod_pow(a, TIMES, N) - 1)).rem_euclid(N);
    let p3 = mod_inverse(a - 1, N).unwrap();

    (p1 + p2 * p3).rem_euclid(N)
}

pub fn part_1() -> i128 {
    solve_1(&DATA)
}

pub fn part_2() -> i128 {
    solve_2(&DATA)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shuffle(n: i128, techniques: &Techniques<Technique>) -> Vec<i128> {
        (0..n)
            .map(|v| {
                techniques
                    .0
                    .iter()
                    .rev()
                    .fold(v, |acc, technique| match technique {
                        Technique::DealWithIncrement(increment) => {
                            (acc * mod_inverse(*increment, n).unwrap()).rem_euclid(n)
                        }
                        Technique::Cut(cut) => (acc + cut).rem_euclid(n),
                        Technique::DealIntoNewStack => (-acc - 1).rem_euclid(n),
                    })
            })
            .collect()
    }

    fn shuffle_calc(n: i128, techniques: &Techniques<Technique>) -> Vec<i128> {
        let (a, b) = linear_compose(n, techniques);

        (0..n).map(|v| (v * a + b).rem_euclid(n)).collect()
    }

    #[test]
    fn test_inverse() {
        assert_eq!(mod_inverse(3, 10), Some(7));
    }

    #[test]
    fn test_deal_into_new_stack() {
        assert_eq!(
            shuffle(10, &r"deal into new stack".parse().unwrap(),),
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
        )
    }

    #[test]
    fn test_deal_into_new_stack_calc() {
        assert_eq!(
            shuffle_calc(10, &r"deal into new stack".parse().unwrap(),),
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
        )
    }

    #[test]
    fn test_cut() {
        assert_eq!(
            shuffle(10, &r"cut 3".parse().unwrap(),),
            vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2],
        )
    }

    #[test]
    fn test_cut_calc() {
        assert_eq!(
            shuffle_calc(10, &r"cut 3".parse().unwrap(),),
            vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2],
        )
    }

    #[test]
    fn test_cut_minor_zero() {
        assert_eq!(
            shuffle(10, &r"cut -4".parse().unwrap(),),
            vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5],
        )
    }

    #[test]
    fn test_cut_minor_zero_calc() {
        assert_eq!(
            shuffle_calc(10, &r"cut -4".parse().unwrap(),),
            vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5],
        )
    }

    #[test]
    fn test_deal_with_increment() {
        assert_eq!(
            shuffle(10, &r"deal with increment 3".parse().unwrap(),),
            vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3],
        )
    }

    #[test]
    fn test_deal_with_increment_calc() {
        assert_eq!(
            shuffle_calc(10, &r"deal with increment 3".parse().unwrap(),),
            vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3],
        )
    }

    #[test]
    fn test_example_1_1() {
        assert_eq!(
            shuffle(
                10,
                &r"deal with increment 7
deal into new stack
deal into new stack"
                    .parse()
                    .unwrap(),
            ),
            vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7],
        )
    }

    #[test]
    fn test_example_1_1_calc() {
        assert_eq!(
            shuffle_calc(
                10,
                &r"deal with increment 7
deal into new stack
deal into new stack"
                    .parse()
                    .unwrap(),
            ),
            vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7],
        )
    }

    #[test]
    fn test_example_1_2() {
        assert_eq!(
            shuffle(
                10,
                &r"cut 6
deal with increment 7
deal into new stack"
                    .parse()
                    .unwrap(),
            ),
            vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6],
        )
    }

    #[test]
    fn test_example_1_2_calc() {
        assert_eq!(
            shuffle_calc(
                10,
                &r"cut 6
deal with increment 7
deal into new stack"
                    .parse()
                    .unwrap(),
            ),
            vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6],
        )
    }

    #[test]
    fn test_example_1_3() {
        assert_eq!(
            shuffle(
                10,
                &r"deal with increment 7
deal with increment 9
cut -2"
                    .parse()
                    .unwrap(),
            ),
            vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9],
        )
    }

    #[test]
    fn test_example_1_4() {
        assert_eq!(
            shuffle(
                10,
                &r"deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1"
                    .parse()
                    .unwrap(),
            ),
            vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6],
        )
    }

    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(3, 3, 10), 7);
    }
}
