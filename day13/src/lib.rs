#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use intcode;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use std::iter;

lazy_static! {
    pub static ref ISTRUCTIONS: Vec<intcode::Memory> = intcode::parse(include_str!("../data.txt"));
}

#[derive(Debug)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

#[derive(Debug)]
enum Error {
    InvalidTileId(intcode::Memory),
}

impl TryFrom<intcode::Memory> for Tile {
    type Error = Error;

    fn try_from(value: intcode::Memory) -> Result<Tile, Error> {
        match value {
            0 => Ok(Tile::Empty),
            1 => Ok(Tile::Wall),
            2 => Ok(Tile::Block),
            3 => Ok(Tile::HorizontalPaddle),
            4 => Ok(Tile::Ball),
            _ => Err(Error::InvalidTileId(value)),
        }
    }
}

pub fn part_1() -> usize {
    let mut cpu = intcode::CPU::new(ISTRUCTIONS.to_vec(), 0, None);
    let generator = || match cpu.run().expect("invalid state") {
        intcode::Run::Halt => None,
        intcode::Run::NeedInput => unreachable!(),
        intcode::Run::Output(value) => Some(value),
    };

    iter::from_fn(generator)
        .enumerate()
        .filter_map(|(index, value)| {
            if index % 3 == 2 {
                match Tile::try_from(value).expect("invalid tile id") {
                    Tile::Block => Some(1),
                    _ => None,
                }
            } else {
                None
            }
        })
        .count()
}

type Point = (intcode::Memory, intcode::Memory);

struct Game {
    ball_position: Option<Point>,
    horizontal_paddle_position: Option<Point>,
    score: Option<intcode::Memory>,
    cpu: intcode::CPU,
}

impl fmt::Debug for Game {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_fmt(format_args!(
            "Game[ball: {:?}, horizontal paddle: {:?}, score: {:?}]",
            self.ball_position, self.horizontal_paddle_position, self.score
        ))
    }
}

impl Game {
    pub fn new(istructions: &[intcode::Memory]) -> Self {
        Self {
            ball_position: None,
            horizontal_paddle_position: None,
            score: None,
            cpu: intcode::CPU::new(
                istructions
                    .iter()
                    .enumerate()
                    .map(|(index, &value)| if index == 0 { 2 } else { value })
                    .collect(),
                0,
                None,
            ),
        }
    }

    pub fn play(&mut self) -> intcode::Memory {
        const PADDLE_RIGHT: intcode::Memory = 1;
        const PADDLE_NEUTRAL: intcode::Memory = 0;
        const PADDLE_LEFT: intcode::Memory = -1;

        let mut output = [0, 0, 0];
        let mut index = 0;
        loop {
            match self.cpu.run().expect("invalid state") {
                intcode::Run::Halt => break,
                intcode::Run::NeedInput => {
                    match (self.horizontal_paddle_position, self.ball_position) {
                        (Some((paddle_x, _)), Some((ball_x, _))) => match paddle_x.cmp(&ball_x) {
                            Ordering::Less => self.cpu.set_input(Some(PADDLE_RIGHT)),
                            Ordering::Equal => self.cpu.set_input(Some(PADDLE_NEUTRAL)),
                            Ordering::Greater => self.cpu.set_input(Some(PADDLE_LEFT)),
                        },
                        _ => unreachable!(),
                    }
                }
                intcode::Run::Output(value) => {
                    output[index] = value;
                    if index == 2 {
                        if output[0] == -1 && output[1] == 0 {
                            self.score = Some(value);
                        } else {
                            match Tile::try_from(value).expect("invalid tile id") {
                                Tile::HorizontalPaddle => {
                                    self.horizontal_paddle_position = Some((output[0], output[1]));
                                }
                                Tile::Ball => {
                                    self.ball_position = Some((output[0], output[1]));
                                }
                                _ => {}
                            }
                        }
                    }
                    index = (index + 1) % 3;
                }
            }
        }

        self.score.expect("no score")
    }
}

pub fn part_2() -> intcode::Memory {
    let mut game = Game::new(&ISTRUCTIONS);

    game.play()
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
