#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DATA: Vec<i32> = parse(include_str!("../data.txt"));
}

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

fn phase(data: &[i32]) -> Vec<i32> {
    let len = data.len();
    let mut result = Vec::with_capacity(len);

    for i in 1..=len {
        let mut a = 0;
        let mut b = 0;
        let t = len / (4 * i) + if len % (4 * i) != 0 { 1 } else { 0 };
        for k in 0..t {
            let k = 4 * i * k;
            let idx = k + i - 1;
            if idx < len {
                a += data[idx..(idx + i).min(len)].iter().sum::<i32>();
            }

            let idx = k + 3 * i - 1;
            if idx < len {
                b += data[idx..(idx + i).min(len)].iter().sum::<i32>();
            }
        }

        let v = (a - b).abs() % 10;

        result.push(v);
    }

    result
}

fn solve_1(data: &[i32], count: usize) -> Vec<i32> {
    let mut current = data.to_owned();
    for _ in 0..count {
        current = phase(&current);
    }
    current
}

fn solve_2(data: &[i32], count: usize) -> String {
    let mut current = std::iter::repeat(data.to_vec())
        .take(10_000)
        .flatten()
        .collect::<Vec<i32>>();
    for _i in 0..count {
        println!("{}", _i);
        current = phase(&current);
    }

    let index = join(&data[0..7]).parse().expect("invalid number");

    join(&current[index..index + 8])
}

pub fn part_1() -> String {
    join(&solve_1(&DATA, 100)[0..8])
}

pub fn part_2() -> String {
    solve_2(&DATA, 100)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_phase_1() {
        assert_eq!(join(&phase(&parse("12345678"))), "48226158");
    }

    #[test]
    fn test_phase_2() {
        assert_eq!(join(&phase(&phase(&parse("12345678")))), "34040438");
    }

    #[test]
    fn test_phase_failing() {
        assert_eq!(join(&phase(&parse("48226158"))), "34040438");
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

    #[test]
    fn test_example_2_1() {
        assert_eq!(
            solve_2(&parse("03036732577212944063491565474664"), 100),
            String::from("84462026")
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
