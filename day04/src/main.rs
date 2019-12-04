#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::cmp::Ordering;

lazy_static! {
    pub static ref DATA: Vec<u32> = include_str!("../data.txt")
        .trim()
        .split('-')
        .map(|v| v.parse().unwrap())
        .collect();
}

pub fn is_valid_password(password: u32) -> bool {
    let mut double_digit_found = false;

    let mut last_digit = password % 10;
    let mut password = password / 10;
    for _ in 1..6 {
        let digit = password % 10;
        match digit.cmp(&last_digit) {
            Ordering::Greater => return false,
            Ordering::Equal => {
                last_digit = digit;
                password /= 10;
                double_digit_found = true;
            }
            Ordering::Less => {
                last_digit = digit;
                password /= 10;
            }
        }
    }

    double_digit_found
}

pub fn is_valid_password_f(password: u32) -> bool {
    (1..6)
        .try_fold(
            (password / 10, password % 10, false),
            |(password, last_digit, double_digit_found), _| {
                let digit = password % 10;
                match digit.cmp(&last_digit) {
                    Ordering::Greater => Err(()),
                    Ordering::Equal => Ok((password / 10, digit, true)),
                    Ordering::Less => Ok((password / 10, digit, double_digit_found)),
                }
            },
        )
        .map(|(_, _, double_digit_found)| double_digit_found)
        .unwrap_or(false)
}

pub fn is_valid_password_2(password: u32) -> bool {
    let mut double_digit_found = false;

    let mut group_size = 1;

    let mut last_digit = password % 10;
    let mut password = password / 10;
    for _ in 1..6 {
        let digit = password % 10;
        match digit.cmp(&last_digit) {
            Ordering::Greater => return false,
            Ordering::Equal => {
                last_digit = digit;
                password /= 10;
                group_size += 1;
            }
            Ordering::Less => {
                if group_size == 2 {
                    double_digit_found = true;
                }
                last_digit = digit;
                password /= 10;
                group_size = 1;
            }
        }
    }

    if group_size == 2 {
        double_digit_found = true;
    }

    double_digit_found
}

pub fn is_valid_password_2_f(password: u32) -> bool {
    (1..6)
        .try_fold(
            (password / 10, password % 10, 1, false),
            |(password, last_digit, group_size, double_digit_found), _| {
                let digit = password % 10;
                match digit.cmp(&last_digit) {
                    Ordering::Greater => Err(()),
                    Ordering::Equal => {
                        Ok((password / 10, digit, group_size + 1, double_digit_found))
                    }
                    Ordering::Less => Ok((
                        password / 10,
                        digit,
                        1,
                        double_digit_found || group_size == 2,
                    )),
                }
            },
        )
        .map(|(_, _, group_size, double_digit_found)| double_digit_found || group_size == 2)
        .unwrap_or(false)
}

pub fn part(f: fn(u32) -> bool) -> usize {
    (DATA[0]..=DATA[1]).filter(|&n| f(n)).count()
}

pub fn part_1() -> usize {
    part(is_valid_password)
}

pub fn part_2() -> usize {
    part(is_valid_password_2)
}

pub fn part_1_f() -> usize {
    part(is_valid_password_f)
}

pub fn part_2_f() -> usize {
    part(is_valid_password_2_f)
}

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_is_valid_password() {
        assert!(is_valid_password(111123));
        assert!(!is_valid_password(135679));
        assert!(is_valid_password(111111));
        assert!(!is_valid_password(223450));
        assert!(!is_valid_password(123789));
    }

    #[test]
    fn test_is_valid_password_f() {
        assert!(is_valid_password_f(111123));
        assert!(!is_valid_password_f(135679));
        assert!(is_valid_password_f(111111));
        assert!(!is_valid_password_f(223450));
        assert!(!is_valid_password_f(123789));
    }

    #[test]
    fn test_is_valid_password_2() {
        assert!(is_valid_password_2(112233));
        assert!(!is_valid_password_2(123444));
        assert!(is_valid_password_2(111122));
        assert!(is_valid_password_2(12233));
        assert!(is_valid_password_2(12344));
        assert!(!is_valid_password_2(12444));
        assert!(is_valid_password_2(112444));
    }

    #[test]
    fn test_is_valid_password_2_f() {
        assert!(is_valid_password_2_f(112233));
        assert!(!is_valid_password_2_f(123444));
        assert!(is_valid_password_2_f(111122));
        assert!(is_valid_password_2_f(12233));
        assert!(is_valid_password_2_f(12344));
        assert!(!is_valid_password_2_f(12444));
        assert!(is_valid_password_2_f(112444));
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part_1);
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part_2);
    }

    #[bench]
    fn bench_part_1_f(b: &mut Bencher) {
        b.iter(part_1_f);
    }

    #[bench]
    fn bench_part_2_f(b: &mut Bencher) {
        b.iter(part_2_f);
    }
}
