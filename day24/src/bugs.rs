use std::fmt;
use std::str::FromStr;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Bugs(u32);

const DIM: isize = 5;

impl Bugs {
    fn get(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 || x >= DIM || y >= DIM {
            false
        } else {
            self.0 & (1 << (y * DIM + x)) != 0
        }
    }

    fn set(&mut self, x: isize, y: isize, value: bool) {
        if x >= 0 && y >= 0 && x < DIM && y < DIM {
            if value {
                self.0 |= 1 << (y * DIM + x);
            } else {
                self.0 &= !(1 << (y * DIM + x));
            }
        }
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl FromStr for Bugs {
    type Err = String;

    fn from_str(data: &str) -> Result<Self, String> {
        u32::from_str_radix(
            &data
                .lines()
                .flat_map(|line| {
                    line.trim().chars().map(|c| match c {
                        '.' => Ok('0'),
                        '#' => Ok('1'),
                        _ => Err(format!("invalid char {}", c)),
                    })
                })
                .rev()
                .collect::<Result<String, String>>()?,
            2,
        )
        .map(Self)
        .map_err(|e| format!("{}", e))
    }
}

impl fmt::Display for Bugs {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str(
            &(0..(DIM * DIM))
                .flat_map(|i| {
                    (String::from(if i % DIM == 0 && i != 0 { "\n" } else { "" })
                        + if (self.0 & (1 << i)) != 0 { "#" } else { "." })
                    .chars()
                    .collect::<Vec<char>>()
                })
                .collect::<String>(),
        )
    }
}

impl Iterator for Bugs {
    type Item = Bugs;

    fn next(&mut self) -> Option<Bugs> {
        let mut bugs = Self(self.0);
        for x in 0..DIM {
            for y in 0..DIM {
                match (
                    [(0, 1), (0, -1), (1, 0), (-1, 0)]
                        .iter()
                        .map(|(dx, dy)| self.get(x + dx, y + dy) as u32)
                        .sum::<u32>(),
                    self.get(x, y),
                ) {
                    (s, true) if s != 1 => bugs.set(x, y, false),
                    (s, false) if s == 1 || s == 2 => bugs.set(x, y, true),
                    _ => {}
                }
            }
        }

        self.0 = bugs.0;

        Some(self.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let data = include_str!("../example.txt");

        assert_eq!(format!("{}\n", data.parse::<Bugs>().unwrap()), data);
    }
}
