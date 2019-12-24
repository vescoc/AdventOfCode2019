#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::iter::FromIterator;
use std::str::FromStr;
use regex::Regex;

lazy_static! {
    static ref DATA: &'static str = include_str!("../data.txt");
    static ref DEAL_WITH_INCREMENT_RE: Regex = Regex::new(r"deal with increment (\d+)").unwrap();
    static ref CUT_RE: Regex = Regex::new(r"cut ((:?-)?\d+)").unwrap();
    static ref DEAL_INTO_NEW_STACK: &'static str = r"deal into new stack";
}

trait Technique {
    fn shuffle(&self, deck: &[usize]) -> Vec<usize>;
    fn shuffle_index(&self, deck_size: u128, index: u128) -> u128;
    fn shuffle_index_rev(&self, deck_size: u128, index: u128) -> u128;
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

// fn mmi(a: u128, n: u128) -> Result<u128, String> {
//     let (mut t, mut new_t) = (0, 1);
//     let (mut r, mut new_r) = (a as i128, n as i128);

//     while new_r != 0 {
// 	let q = r / new_r;
	
// 	let tmp = t - q * new_t;
// 	t = new_t;
// 	new_t = tmp;

// 	let tmp = r - q * new_r;
// 	r = new_r;
// 	new_r = tmp;
//     }

//     if r > 1 {
// 	Err(format!("{} is not invertible mod {}", a, n))
//     } else {
// 	Ok(if new_t < 0 { (new_t + n as i128) as u128 } else { new_t as u128 })
//     }
// }

fn mmi(a: u128, base: u128) -> Result<u128, String> {
    if base == 1 {
        return Ok(0);
    }

    let mut a = a as i128;
    let mut base = base as i128;
    
    let orig = base;

    let mut x = 1;
    let mut y = 0;

    while a > 1 {
        let q = a / base;
        let tmp = base;
        base = a % base;
        a = tmp;
        let tmp = y;
        y = x - q * y;
        x = tmp;
    }

    if x < 0 {
        Ok((x + orig) as u128)
    } else {
        Ok(x as u128)
    }
}

fn pow(mut base: u128, mut exponent: u128, modulus: u128) -> u128 {
    let mut result = 1;
    loop {
	if exponent <= 0 {
	    break result;
	}

	if exponent & 1 == 1 {
	    result = (result * base) % modulus;
	}

	exponent >>= 1;
	base = (base * base) % modulus;
    }
}

#[allow(dead_code)]
fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
	let t = a % b;
	a = b;
	b = t;
    }

    a
}

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

    fn shuffle_index(&self, deck_size: u128, index: u128) -> u128 {
	(index * self.0 as u128) % deck_size
    }

    fn shuffle_index_rev(&self, deck_size: u128, index: u128) -> u128 {
	mmi(self.0 as u128, deck_size).unwrap() * index % deck_size
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

    fn shuffle_index(&self, deck_size: u128, index: u128) -> u128 {
	((deck_size as i128 + index as i128 - self.0 as i128) % deck_size as i128) as u128
    }

    fn shuffle_index_rev(&self, deck_size: u128, index: u128) -> u128 {
	((deck_size as i128 + index as i128 + self.0 as i128) % deck_size as i128) as u128
    }
}

impl Technique for DealIntoNewStack {
    fn shuffle(&self, desk: &[usize]) -> Vec<usize> {
        desk.iter().copied().rev().collect()
    }

    fn shuffle_index(&self, deck_size: u128, index: u128) -> u128 {
	deck_size - index - 1
    }

    fn shuffle_index_rev(&self, deck_size: u128, index: u128) -> u128 {
	deck_size - index - 1
    }
}

impl Technique for Techniques {
    fn shuffle(&self, desk: &[usize]) -> Vec<usize> {
        self.0.iter().fold(desk.to_vec(), |acc, t| t.shuffle(&acc))
    }

    fn shuffle_index(&self, deck_size: u128, index: u128) -> u128 {
	self.0.iter().fold(index, |acc, t| t.shuffle_index(deck_size, acc))
    }

    fn shuffle_index_rev(&self, deck_size: u128, index: u128) -> u128 {
	self.0.iter().rev().fold(index, |acc, t| t.shuffle_index_rev(deck_size, acc))
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

#[allow(dead_code)]
fn shuffle(deck_size: usize, shuffle_techniques: &Techniques) -> Vec<usize> {
    shuffle_techniques.shuffle(&(0..deck_size).collect::<Vec<usize>>())
}

fn shuffle_index(deck_size: u128, index: u128, shuffle_techniques: &Techniques) -> u128 {
    shuffle_techniques.shuffle_index(deck_size, index)
}

fn shuffle_index_rev(deck_size: u128, index: u128, shuffle_techniques: &Techniques) -> u128 {
    shuffle_techniques.shuffle_index_rev(deck_size, index)
}

fn solve_2(deck_size: u128, shuffle_times: u128, tracked_index: u128, shuffle_techniques: &Techniques) -> u128 {
    // |a*x+b=y=f(2020)
    // |a*y+b=z=f(f(2020))
    // a*(x-y)=y-z
    // a=(y-z)/(x-y)
    // b=y-a*x
    // maxima: solve([a*x+b=y,a*y+b=z],[a,b]);

    let y = shuffle_index_rev(deck_size, tracked_index, &shuffle_techniques);
    let z = shuffle_index_rev(deck_size, y, &shuffle_techniques);

    let a = ((deck_size + y - z) * mmi((deck_size + tracked_index - y) % deck_size, deck_size).unwrap()) % deck_size;
    let b = (deck_size + y - (a * tracked_index) % deck_size) % deck_size;

    // f(f(x)) = a*(a*x+b)+b = a^2*x+ab+b
    // f(f(f(x))) = a*(a*(a*x+b)+b)+b = a^3*x+a^2b+ab+b
    // f^n(x) = a^n*x+(a^(n-1)+a^(n-2)+...+1)*b = a^n*x+(a^n-1)/(a-1)*b
    // maxima: nusum(a^(i-1), i, 1, n);

    let p = pow(a, shuffle_times, deck_size);
    let mmi = mmi(a - 1, deck_size).unwrap();

    println!("maxima: a:{}; b:{}; mmi:{} p:{}", a, b, mmi, p);
    println!("maxima: mod({} * {} + ({} - 1) * {} * {}, {})", p, tracked_index, p, mmi, b, deck_size);
    
    ((p * tracked_index + (p - 1) * mmi * b)) % deck_size
}

pub fn part_1() -> u128 {
    shuffle_index(10_007, 2019, &DATA.parse().unwrap())
}

pub fn part_2() -> u128 {
    const DECK_SIZE: u128 = 101_741_582_076_661;
    const SHUFFLE_TIMES: u128 = 101_741_582_076_661;
    const TRACKED_INDEX: u128 = 2020;
    
    solve_2(DECK_SIZE, SHUFFLE_TIMES, TRACKED_INDEX, &DATA.parse().unwrap())
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

    #[test]
    fn test_same_results_part_1() {
	let shuffles = DATA.parse().unwrap();
	
	let deck = shuffle(10_007, &shuffles);
	let position = deck.into_iter().position(|c| c == 2019).unwrap();
	    
	assert_eq!(
	    shuffle_index(10_007, 2019, &shuffles),
	    position as u128,
	)		
    }

    #[test]
    fn test_same_results_rev() {
	let shuffles = DATA.parse().unwrap();
	
	let deck = shuffle(10_007, &shuffles);
	let position = deck.into_iter().position(|c| c == 2019).unwrap();
	    
	assert_eq!(
	    shuffle_index_rev(10_007, position as u128, &shuffles),
	    2019,
	)		
    }
    
    #[test]
    fn test_deal_with_increment_rev() {
	let source = vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3];
	let shuffles = "deal with increment 3".parse().unwrap();

	for (i, &v) in source.iter().enumerate() {
	    assert_eq!(shuffle_index_rev(source.len() as u128, i as u128, &shuffles), v);
	}
    }

    #[test]
    fn test_cut_rev() {
	let source = vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
	let shuffles = "cut 3".parse().unwrap();

	for (i, &v) in source.iter().enumerate() {
	    assert_eq!(shuffle_index_rev(source.len() as u128, i as u128, &shuffles), v);
	}
    }

    #[test]
    fn test_deal_into_new_stack_rev() {
	let source = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
	let shuffles = "deal into new stack".parse().unwrap();

	for (i, &v) in source.iter().enumerate() {
	    assert_eq!(shuffle_index_rev(source.len() as u128, i as u128, &shuffles), v);
	}
    }

    #[test]
    fn test_shuffle_unshuffle() {
	let shuffles = DATA.parse().unwrap();
	
	let deck = shuffle(10_007, &shuffles);
	let card = deck.into_iter().position(|c| c == 2019).unwrap();
	    
	assert_eq!(
	    shuffle_index(10_007, 2019, &shuffles),
	    card as u128,
	)		
    }

    #[test]
    fn test_shuffle_rev() {
	let shuffles = DATA.parse::<Techniques>().unwrap();

	const SHUFFLES: usize = 1;

	let deck = (0..SHUFFLES).fold((0..10_007).collect::<Vec<usize>>(),
			       |acc, _| shuffles.shuffle(&acc));
	
	assert_eq!(
	    (0..SHUFFLES).fold(2019, |index, _| shuffle_index_rev(10_007, index, &shuffles)),
	    deck[2019] as u128,
	);
	
	assert_eq!(
	    solve_2(10_007, SHUFFLES as u128, 2019, &shuffles),
	    deck[2019] as u128,
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
