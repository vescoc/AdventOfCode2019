#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::iter::FromIterator;
use std::str::FromStr;

lazy_static! {
    static ref DATA: &'static str = include_str!("../data.txt");
    static ref DEAL_WITH_INCREMENT_RE: Regex = Regex::new(r"deal with increment (\d+)").unwrap();
    static ref CUT_RE: Regex = Regex::new(r"cut ((:?-)?\d+)").unwrap();
    static ref DEAL_INTO_NEW_STACK: &'static str = r"deal into new stack";
}

trait Technique {
    fn shuffle(&self, deck: &[usize]) -> Vec<usize>;
}

struct Techniques(Vec<Box<dyn Technique>>);

impl Techniques {
    fn make_deal_with_increment(increment: usize) -> Box<dyn Technique> {
        Box::new(DealWithIncrement(increment))
    }

    fn make_cut(size: isize) -> Box<dyn Technique> {
        Box::new(Cut(size))
    }

    fn make_deal_into_new_stack() -> Box<dyn Technique> {
        Box::new(DealIntoNewStack)
    }
}

impl FromIterator<Box<dyn Technique>> for Techniques {
    fn from_iter<I: IntoIterator<Item = Box<dyn Technique>>>(iter: I) -> Self {
        Techniques(iter.into_iter().collect())
    }
}

struct DealWithIncrement(usize);
struct Cut(isize);
struct DealIntoNewStack;

impl Technique for DealWithIncrement {
    fn shuffle(&self, desk: &[usize]) -> Vec<usize> {
        let len = desk.len();

        desk.iter().zip((0..len).map(|i| (i * self.0) % len)).fold(
            desk.to_owned(),
            |mut acc, (&v, i)| {
                acc[i] = v;
                acc
            },
        )
    }
}

impl Technique for Cut {
    fn shuffle(&self, desk: &[usize]) -> Vec<usize> {
        let (a, b) = if self.0 > 0 {
            desk.split_at(self.0 as usize)
        } else {
            desk.split_at((desk.len() as isize + self.0) as usize)
        };

        [b, a].concat()
    }
}

impl Technique for DealIntoNewStack {
    fn shuffle(&self, desk: &[usize]) -> Vec<usize> {
        desk.iter().copied().rev().collect()
    }
}

impl Technique for Techniques {
    fn shuffle(&self, desk: &[usize]) -> Vec<usize> {
        self.0.iter().fold(desk.to_vec(), |acc, t| t.shuffle(&acc))
    }
}

impl FromStr for Techniques {
    type Err = String;

    fn from_str(data: &str) -> Result<Techniques, String> {
        data.lines()
            .map(|line| {
                let line = line.trim();
                if let Some(cap) = DEAL_WITH_INCREMENT_RE.captures(line) {
                    Ok(Techniques::make_deal_with_increment(
                        cap.get(1).unwrap().as_str().parse().unwrap(),
                    ))
                } else if let Some(cap) = CUT_RE.captures(line) {
                    Ok(Techniques::make_cut(
                        cap.get(1).unwrap().as_str().parse().unwrap(),
                    ))
                } else if *DEAL_INTO_NEW_STACK == line {
                    Ok(Techniques::make_deal_into_new_stack())
                } else {
                    Err(format!("invalid line '{}'", line))
                }
            })
            .collect()
    }
}

fn shuffle(deck_size: usize, shuffle_techniques: &Techniques) -> Vec<usize> {
    shuffle_techniques.shuffle(&(0..deck_size).collect::<Vec<usize>>())
}

pub fn part_1() -> usize {
    let deck = shuffle(10_007, &DATA.parse().unwrap());

    deck.into_iter().position(|c| c == 2019).unwrap()
}

pub fn part_2() -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

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

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part_1);
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part_2);
    }
}
