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

pub fn is_valid_password_2(password: u32) -> bool {
    false
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
            _ => {
                last_digit = digit;
                password /= 10;
            }
        }
    }
    double_digit_found
}

fn part_1() -> usize {
    (DATA[0]..=DATA[1])
        .filter(|&n| is_valid_password(n))
        .count()
}

fn main() {
    println!("{}-{}", DATA[0], DATA[1]);
    println!("part 1: {}", part_1());
}

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_password() {
        assert!(is_valid_password(111123));
        assert!(!is_valid_password(135679));
        assert!(is_valid_password(111111));
        assert!(!is_valid_password(223450));
        assert!(!is_valid_password(123789));
    }

    #[test]
    fn test_is_valid_password_2() {
        assert!(is_valid_password_2(112233));
        assert!(!is_valid_password(123444));
        assert!(is_valid_password(111122));
    }
}
