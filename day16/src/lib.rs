#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DATA: Vec<i32> = parse(include_str!("../data.txt"));
}

const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

fn parse(data: &str) -> Vec<i32> {
    data.trim()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .unwrap_or_else(|| panic!("invalid digit {}", c)) as i32
        })
        .collect()
}

fn join(data: &[i32]) -> String {
    data.iter()
        .map(|&v| {
            std::char::from_digit(v as u32, 10).unwrap_or_else(|| panic!("invalid number {}", v))
        })
        .collect()
}

fn pattern(i: usize) -> impl Iterator<Item = i32> {
    BASE_PATTERN
        .iter()
        .flat_map(move |&v| std::iter::repeat(v).take(i))
        .cycle()
        .skip(1)
}

fn phase(data: &[i32]) -> Vec<i32> {
    std::iter::repeat(data)
        .take(data.len())
        .enumerate()
        .map(|(i, data)| {
            data.iter()
                .zip(pattern(i + 1))
                .map(|(a, b)| a * b)
                .sum::<i32>()
                .abs()
                % 10
        })
        .collect()
}

fn solve_1(data: &[i32], count: usize) -> Vec<i32> {
    let mut current = data.to_owned();
    for _ in 0..count {
        current = phase(&current);
    }
    current
}

pub fn part_1() -> String {
    join(&solve_1(&DATA, 100)[0..8])
}

pub fn part_2() -> String {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_pattern() {
        assert_eq!(pattern(1).take(4).collect::<Vec<i32>>(), vec![1, 0, -1, 0]);
        assert_eq!(
            pattern(2).take(8).collect::<Vec<i32>>(),
            vec![0, 1, 1, 0, 0, -1, -1, 0]
        );
    }

    #[test]
    fn test_phase() {
        assert_eq!(join(&phase(&parse("12345678"))), "48226158");
    }

    #[test]
    fn test_example_1_1() {
        assert_eq!(
            join(&solve_1(&parse("80871224585914546619083218645595"), 100)[0..8]),
            String::from("24176176")
        );
    }

    #[test]
    fn test_example_1_2() {
        assert_eq!(
            join(&solve_1(&parse("19617804207202209144916044189917"), 100)[0..8]),
            String::from("73745418")
        );
    }

    #[test]
    fn test_example_1_3() {
        assert_eq!(
            join(&solve_1(&parse("69317163492948606335995924319873"), 100)[0..8]),
            String::from("52432133")
        );
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
