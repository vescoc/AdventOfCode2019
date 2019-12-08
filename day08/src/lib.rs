#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::iter;

const LAYER_WIDE: usize = 25;
const LAYER_TALL: usize = 6;
const TRANSPARENT_PIXEL: u32 = 2;

lazy_static! {
    static ref DATA: Vec<u32> = parse(include_str!("../data.txt"));
}

fn parse(data: &str) -> Vec<u32> {
    data.trim()
        .chars()
        .flat_map(|c| c.to_digit(10))
        .collect::<Vec<_>>()
}

pub fn solve_1(data: &[u32], layer_wide: usize, layer_tall: usize) -> usize {
    let layer_size = layer_wide * layer_tall;

    let (min_zero_layer, _) = data
        .chunks(layer_size)
        .enumerate()
        .map(|(i, l)| (i, l.iter().filter(|&d| *d == 0).count()))
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    let layer = &data[(min_zero_layer * layer_size)..((min_zero_layer + 1) * layer_size)];

    layer.iter().filter(|&d| *d == 1).count() * layer.iter().filter(|&d| *d == 2).count()
}

fn decode(data: &[u32], layer_wide: usize, layer_tall: usize) -> Vec<u32> {
    let layer_size = layer_wide * layer_tall;
    let n_layers = data.len() / layer_size;

    let mut image = vec![2; layer_size];
    for i in 0..layer_size {
        image[i] = (0..n_layers)
            .try_fold(TRANSPARENT_PIXEL, |acc, l| {
                let value = data[i + l * layer_size];
                if value != TRANSPARENT_PIXEL {
                    Err(value)
                } else {
                    Ok(acc)
                }
            })
            .unwrap_err();
    }

    image
}

pub fn solve_2(data: &[u32], layer_wide: usize, layer_tall: usize) -> String {
    decode(data, layer_wide, layer_tall)
        .iter()
        .map(|&p| match p {
            0 => ' ',
            1 => '#',
            _ => unreachable!(),
        })
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % layer_wide == 0 {
                Some('\n')
            } else {
                None
            }
            .into_iter()
            .chain(iter::once(c))
        })
        .collect::<String>()
}

pub fn part_1() -> usize {
    solve_1(&DATA, LAYER_WIDE, LAYER_TALL)
}

pub fn part_2() -> String {
    solve_2(&DATA, LAYER_WIDE, LAYER_TALL)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_example_1_1() {
        let data = parse("123456789012");

        assert_eq!(solve_1(&data, 3, 2), 1);
    }

    #[test]
    fn test_example_2_1() {
        let data = parse("0222112222120000");

        assert_eq!(decode(&data, 2, 2), &[0, 1, 1, 0]);
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
