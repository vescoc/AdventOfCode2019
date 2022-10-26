use std::collections::VecDeque;
use std::fmt::{self, Write};
use std::str;

#[derive(PartialEq, Eq, Debug)]
struct BugsLayer(u32);

impl BugsLayer {
    const DIM: usize = Bugs::DIM;

    fn get_left_inner(&self) -> usize {
        (0..5).map(|i| self.is_set((0, i)) as usize).sum()
    }

    fn get_right_inner(&self) -> usize {
        (0..5).map(|i| self.is_set((4, i)) as usize).sum()
    }

    fn get_top_inner(&self) -> usize {
        (0..5).map(|i| self.is_set((i, 0)) as usize).sum()
    }

    fn get_bottom_inner(&self) -> usize {
        (0..5).map(|i| self.is_set((i, 4)) as usize).sum()
    }

    fn get_right_outer(&self) -> usize {
        self.is_set((3, 2)) as usize
    }

    fn get_left_outer(&self) -> usize {
        self.is_set((1, 2)) as usize
    }

    fn get_top_outer(&self) -> usize {
        self.is_set((2, 1)) as usize
    }

    fn get_bottom_outer(&self) -> usize {
        self.is_set((2, 3)) as usize
    }

    fn get(
        &self,
        (x, y): (usize, usize),
        (dx, dy): (isize, isize),
        outer: &Option<&Self>,
        inner: &Option<&Self>,
    ) -> usize {
        let (x, y) = (x as isize + dx, y as isize + dy);
        if x == 2 && y == 2 {
            if dx < 0 {
                inner.map(Self::get_right_inner).unwrap_or_default()
            } else if dx > 0 {
                inner.map(Self::get_left_inner).unwrap_or_default()
            } else if dy < 0 {
                inner.map(Self::get_bottom_inner).unwrap_or_default()
            } else if dy > 0 {
                inner.map(Self::get_top_inner).unwrap_or_default()
            } else {
                unreachable!()
            }
        } else if x < 0 {
            outer.map(Self::get_left_outer).unwrap_or_default()
        } else if x >= Self::DIM as isize {
            outer.map(Self::get_right_outer).unwrap_or_default()
        } else if y < 0 {
            outer.map(Self::get_top_outer).unwrap_or_default()
        } else if y >= Self::DIM as isize {
            outer.map(Self::get_bottom_outer).unwrap_or_default()
        } else {
            self.is_set((x as usize, y as usize)) as usize
        }
    }

    fn is_set(&self, (x, y): (usize, usize)) -> bool {
        (self.0 >> (y * BugsLayer::DIM + x)) & 1 == 1
    }

    fn set(&mut self, (x, y): (usize, usize), value: bool) {
        if value {
            self.0 |= 1 << (y * Self::DIM + x);
        } else {
            self.0 &= !(1 << (y * Self::DIM + x));
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct Bugs(VecDeque<BugsLayer>);

impl str::FromStr for Bugs {
    type Err = &'static str;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let layer = data
            .lines()
            .flat_map(|line| {
                line.trim().chars().map(|c| match c {
                    '.' | '?' => Ok(0),
                    '#' => Ok(1),
                    _ => Err("invalid char"),
                })
            })
            .try_fold(0, |acc, v| v.map(|v| (acc << 1) + v))?;

        let queue = VecDeque::from([BugsLayer(layer)]);

        Ok(Bugs(queue))
    }
}

impl fmt::Debug for Bugs {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let Bugs(queue) = self;

        for (i, BugsLayer(value)) in queue.iter().enumerate() {
            writeln!(
                f,
                "bits: 0b{:025b} depth: {}",
                value,
                i as isize - queue.len() as isize / 2
            )?;
        }

        Ok(())
    }
}

impl fmt::Display for Bugs {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let Bugs(queue) = self;
        let len = queue.len();

        for (i, BugsLayer(value)) in queue.iter().enumerate() {
            write!(
                f,
                "Depth {} 0b{:025b}",
                i as isize - len as isize / 2,
                value
            )?;
            for i in 0..5 * 5 {
                if i % 5 == 0 {
                    writeln!(f)?;
                }
                let c = if i == 5 * 5 / 2 {
                    '?'
                } else if value & (1 << (5 * 5 - 1 - i)) != 0 {
                    '#'
                } else {
                    '.'
                };
                f.write_char(c)?;
            }

            if i + 1 < len {
                writeln!(f)?;
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl Iterator for Bugs {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let mut count = 0;
        let mut queue = VecDeque::new();

        match Self::evolve(&BugsLayer(0), &None, &self.0.front()) {
            (layer, layer_count) if layer_count > 0 => {
                queue.push_back(layer);
                count += layer_count;
            }
            _ => {}
        }

        let mut outer = None;
        while let Some(current) = self.0.pop_front() {
            let inner = self.0.front();

            let (layer, layer_count) = Self::evolve(&current, &outer.as_ref(), &inner);

            queue.push_back(layer);
            count += layer_count;

            outer = Some(current);
        }

        match Self::evolve(&BugsLayer(0), &outer.as_ref(), &None) {
            (layer, layer_count) if layer_count > 0 => {
                queue.push_back(layer);
                count += layer_count;
            }
            _ => {}
        }

        self.0 = queue;

        Some(count)
    }
}

impl Bugs {
    const DIM: usize = 5;

    fn evolve(
        current: &BugsLayer,
        outer: &Option<&BugsLayer>,
        inner: &Option<&BugsLayer>,
    ) -> (BugsLayer, usize) {
        let mut count = 0;
        let mut layer = BugsLayer(current.0);
        for x in 0..Self::DIM {
            for y in 0..Self::DIM {
                if (x, y) == (2, 2) {
                    continue;
                }

                match (
                    [(0, 1), (0, -1), (1, 0), (-1, 0)]
                        .iter()
                        .map(|d| current.get((x, y), *d, outer, inner))
                        .sum::<usize>(),
                    current.is_set((x, y)),
                ) {
                    (s, true) if s != 1 => layer.set((x, y), false),
                    (s, false) if s == 1 || s == 2 => {
                        layer.set((x, y), true);
                        count += 1;
                    }
                    (_, true) => {
                        count += 1;
                    }
                    _ => {}
                }
            }
        }

        (layer, count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            include_str!("../example.txt").parse(),
            Ok(Bugs(VecDeque::from([BugsLayer(
                0b0000110010100110010010000
            )])))
        );
    }
}
