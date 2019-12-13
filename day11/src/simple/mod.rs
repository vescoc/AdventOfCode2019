use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use crate::DATA;
use intcode::{Memory, Run, CPU};

type Point = (i32, i32);

static DIRECTIONS: [Point; 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub struct Painter {
    cpu: CPU,
    panel: HashMap<Point, Memory>,
    position: Point,
    direction_index: usize,
}

impl Painter {
    pub fn new(istructions: &[Memory]) -> Self {
        Self {
            cpu: CPU::new(istructions.to_vec(), 0, None),
            panel: HashMap::new(),
            position: (0, 0),
            direction_index: 0,
        }
    }

    pub fn paint(&mut self) -> Option<(Point, Memory)> {
        let mut result = None;

        self.cpu.set_input(Some(
            self.panel
                .get(&self.position)
                .copied()
                .unwrap_or_default()
                .to_owned(),
        ));
        match self
            .cpu
            .run()
            .unwrap_or_else(|e| panic!("unexpected error {:?}", e))
        {
            Run::NeedInput => panic!("invalid input request"),
            Run::Output(value) => {
                self.panel.insert(self.position.to_owned(), value);
                result = Some((self.position.to_owned(), value));
            }
            Run::Halt => return result,
        }

        match self
            .cpu
            .run()
            .unwrap_or_else(|e| panic!("unexpected error {:?}", e))
        {
            Run::NeedInput => panic!("invalid input request"),
            Run::Output(value) => {
                self.direction_index = match value {
                    0 => (self.direction_index + DIRECTIONS.len() - 1) % DIRECTIONS.len(),
                    1 => (self.direction_index + 1) % DIRECTIONS.len(),
                    _ => unreachable!(),
                };
                self.position = (
                    self.position.0 + DIRECTIONS[self.direction_index].0,
                    self.position.1 + DIRECTIONS[self.direction_index].1,
                );
            }
            Run::Halt => {}
        }

        result
    }

    pub fn paint_panel(&mut self) {
        while let Some(_) = self.paint() {}
    }
}

impl Iterator for Painter {
    type Item = (Point, Memory);

    fn next(&mut self) -> Option<(Point, Memory)> {
        self.paint()
    }
}

impl Deref for Painter {
    type Target = HashMap<Point, Memory>;

    fn deref(&self) -> &HashMap<Point, Memory> {
        &self.panel
    }
}

impl DerefMut for Painter {
    fn deref_mut(&mut self) -> &mut HashMap<Point, Memory> {
        &mut self.panel
    }
}

pub fn part_1() -> usize {
    let mut painter = Painter::new(&DATA);

    painter.paint_panel();

    painter.len()
}

pub fn part_2() -> String {
    let mut painter = Painter::new(&DATA);

    painter.insert((0, 0), 1);

    painter.paint_panel();

    let ((min_x, min_y), (max_x, max_y)) = painter.iter().fold(
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
                match painter.get(&(x, y)) {
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
