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

fn fft(data: &[i32]) -> Vec<i32> {
    let len = data.len();
    let mut partial_sum = vec![0; len];

    let mut sum = 0;
    for i in (0..len).rev() {
        sum += data[i];
        partial_sum[i] = sum;
    }

    let mut result = Vec::with_capacity(len);

    for i in 1..=len {
        let mut a = 0;
        let mut b = 0;
        let t = len / (4 * i) + if len % (4 * i) != 0 { 1 } else { 0 };
        for k in 0..t {
            let k = 4 * i * k;
            let idx = k + i - 1;
            if idx < len {
                a += partial_sum[idx]
                    - if idx + i >= len {
                        0
                    } else {
                        partial_sum[idx + i]
                    };
            }

            let idx = k + 3 * i - 1;
            if idx < len {
                b += partial_sum[idx]
                    - if idx + i >= len {
                        0
                    } else {
                        partial_sum[idx + i]
                    };
            }
        }

        let v = (a - b).abs() % 10;

        result.push(v);
    }

    result
}

fn solve_1(data: &[i32]) -> String {
    let mut current = data.to_owned();
    for _ in 0..100 {
        current = fft(&current);
    }

    join(&current[0..8])
}

fn solve_2(data: &[i32]) -> String {
    let mut current = std::iter::repeat(data.to_vec())
        .take(10_000)
        .flatten()
        .collect::<Vec<i32>>();
    for _ in 0..100 {
        current = fft(&current);
    }

    let index = join(&data[0..7]).parse().expect("invalid number");

    join(&current[index..index + 8])
}

pub fn part_1() -> String {
    solve_1(&DATA)
}

pub fn part_2() -> String {
    solve_2(&DATA)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_fft_1() {
        assert_eq!(join(&fft(&parse("12345678"))), "48226158");
    }

    #[test]
    fn test_fft_2() {
        assert_eq!(join(&fft(&fft(&parse("12345678")))), "34040438");
    }

    #[test]
    fn test_fft_failing() {
        assert_eq!(join(&fft(&parse("48226158"))), "34040438");
    }

    #[test]
    fn test_example_1_1() {
        assert_eq!(
            solve_1(&parse("80871224585914546619083218645595")),
            String::from("24176176")
        );
    }

    #[test]
    fn test_example_1_2() {
        assert_eq!(
            solve_1(&parse("19617804207202209144916044189917")),
            String::from("73745418")
        );
    }

    #[test]
    fn test_example_1_3() {
        assert_eq!(
            solve_1(&parse("69317163492948606335995924319873")),
            String::from("52432133")
        );
    }

    #[test]
    fn test_example_2_1() {
        assert_eq!(
            solve_2(&parse("03036732577212944063491565474664")),
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
