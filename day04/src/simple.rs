use std::cmp::Ordering;

use crate::part;

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

pub fn part_1() -> usize {
    part(is_valid_password)
}

pub fn part_2() -> usize {
    part(is_valid_password_2)
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
    fn test_is_valid_password_2() {
        assert!(is_valid_password_2(112233));
        assert!(!is_valid_password_2(123444));
        assert!(is_valid_password_2(111122));
        assert!(is_valid_password_2(12233));
        assert!(is_valid_password_2(12344));
        assert!(!is_valid_password_2(12444));
        assert!(is_valid_password_2(112444));
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
