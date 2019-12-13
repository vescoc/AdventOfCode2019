use std::collections::HashMap;

use crate::DATA;
use intcode::{Memory, Run, CPU};

type Point = (i32, i32);

trait Paint {
    fn paint(self, istructions: &[Memory]) -> Self;
}

impl Paint for HashMap<Point, Memory> {
    fn paint(mut self, istructions: &[Memory]) -> Self {
        let mut cpu = CPU::new(istructions.to_owned(), 0, None);

        let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        let mut direction_index = 0;
        let mut position = (0, 0);
        loop {
            cpu.set_input(Some(
                self.get(&position).copied().unwrap_or_default().to_owned(),
            ));
            match cpu
                .run()
                .unwrap_or_else(|e| panic!("unexpected error {:?}", e))
            {
                Run::NeedInput => panic!("invalid input request"),
                Run::Output(value) => {
                    self.insert(position.to_owned(), value);
                }
                Run::Halt => break,
            }

            match cpu
                .run()
                .unwrap_or_else(|e| panic!("unexpected error {:?}", e))
            {
                Run::NeedInput => panic!("invalid input request"),
                Run::Output(value) => {
                    direction_index = match value {
                        0 => (direction_index + directions.len() - 1) % directions.len(),
                        1 => (direction_index + 1) % directions.len(),
                        _ => unreachable!(),
                    };
                    position = (
                        position.0 + directions[direction_index].0,
                        position.1 + directions[direction_index].1,
                    );
                }
                Run::Halt => break,
            }
        }

        self
    }
}

pub fn part_1() -> usize {
    let panel = HashMap::new().paint(&DATA);

    panel.len()
}

pub fn part_2() -> String {
    let panel = vec![((0, 0), 1)]
        .into_iter()
        .collect::<HashMap<Point, Memory>>()
        .paint(&DATA);

    let ((min_x, min_y), (max_x, max_y)) = panel.iter().fold(
        (
            (i32::max_value(), i32::max_value()),
            (i32::min_value(), i32::min_value()),
        ),
        |((min_x, min_y), (max_x, max_y)), ((x, y), v)| {
            if *v == 1 {
                (
                    (min_x.min(*x), min_y.min(*y)),
                    (max_x.max(*x), max_y.max(*y)),
                )
            } else {
                ((min_x, min_y), (max_x, max_y))
            }
        },
    );

    let capacity = ((max_y - min_y + 1) * (max_x - min_x + 2)) as usize;
    let mut data = String::with_capacity(capacity);
    for y in min_y..=max_y {
        for x in min_x..=max_x + 1 {
            data.push(if x == max_x + 1 {
                '\n'
            } else {
                match panel.get(&(x, y)) {
                    Some(&value) if value == 1 => '#',
                    _ => ' ',
                }
            });
        }
    }

    assert_eq!(data.capacity(), capacity);
    assert_eq!(data.len(), capacity);

    data
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part_1);
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part_2);
    }
}
