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

#[derive(Debug, PartialEq)]
pub enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

#[derive(Debug)]
pub enum Error {
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

pub type Point = (intcode::Memory, intcode::Memory);

#[derive(PartialEq)]
pub enum Event {
    Draw(Tile, Point),
    Score(intcode::Memory),
    NeedInput,
    Halt,
}

pub enum Joystick {
    Left,
    Neutral,
    Right,
}

const PADDLE_RIGHT: intcode::Memory = 1;
const PADDLE_NEUTRAL: intcode::Memory = 0;
const PADDLE_LEFT: intcode::Memory = -1;

impl From<&Joystick> for i128 {
    fn from(joystick: &Joystick) -> i128 {
        match joystick {
            Joystick::Left => PADDLE_LEFT,
            Joystick::Neutral => PADDLE_NEUTRAL,
            Joystick::Right => PADDLE_RIGHT,
        }
    }
}

pub struct Game {
    ball_position: Option<Point>,
    horizontal_paddle_position: Option<Point>,
    score: Option<intcode::Memory>,
    output: [intcode::Memory; 3],
    output_index: usize,
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
    pub fn score(&self) -> Option<intcode::Memory> {
        self.score
    }

    pub fn new(istructions: &[intcode::Memory], coins: Option<intcode::Memory>) -> Self {
        let istructions = if let Some(coins) = coins {
            istructions
                .iter()
                .enumerate()
                .map(|(index, &value)| if index == 0 { coins } else { value })
                .collect()
        } else {
            istructions.to_vec()
        };

        Self {
            ball_position: None,
            horizontal_paddle_position: None,
            score: None,
            output: [0, 0, 0],
            output_index: 0,
            cpu: intcode::CPU::new(istructions, 0, None),
        }
    }

    pub fn step(&mut self, joystick: Option<Joystick>) -> Event {
        loop {
            match self.cpu.run().expect("invalid state") {
                intcode::Run::Halt => break Event::Halt,
                intcode::Run::NeedInput => {
                    if let Some(joystick) = &joystick {
                        self.cpu.set_input(Some(i128::from(joystick)));
                    } else {
                        break Event::NeedInput;
                    }
                }
                intcode::Run::Output(value) => {
                    self.output[self.output_index] = value;
                    if self.output_index == 2 {
                        if self.output[0] == -1 && self.output[1] == 0 {
                            self.score = Some(value);

                            self.output_index = (self.output_index + 1) % 3;

                            break Event::Score(value);
                        } else {
                            let tile = Tile::try_from(value).expect("invalid tile id");
                            match tile {
                                Tile::HorizontalPaddle => {
                                    self.horizontal_paddle_position =
                                        Some((self.output[0], self.output[1]));
                                }
                                Tile::Ball => {
                                    self.ball_position = Some((self.output[0], self.output[1]));
                                }
                                _ => {}
                            }

                            self.output_index = (self.output_index + 1) % 3;

                            break Event::Draw(tile, (self.output[0], self.output[1]));
                        }
                    } else {
                        self.output_index = (self.output_index + 1) % 3;
                    }
                }
            }
        }
    }

    pub fn play(&mut self) -> Event {
        match self.step(None) {
            Event::NeedInput => {
                match (self.horizontal_paddle_position, self.ball_position) {
                    (Some((paddle_x, _)), Some((ball_x, _))) => match paddle_x.cmp(&ball_x) {
                        Ordering::Less => self.cpu.set_input(Some(PADDLE_RIGHT)),
                        Ordering::Equal => self.cpu.set_input(Some(PADDLE_NEUTRAL)),
                        Ordering::Greater => self.cpu.set_input(Some(PADDLE_LEFT)),
                    },
                    _ => unreachable!(),
                }

                Event::NeedInput
            }
            e => e,
        }
    }
}

pub fn part_1() -> usize {
    let mut game = Game::new(&ISTRUCTIONS, None);
    let generator = || match game.step(None) {
        Event::Halt => None,
        Event::Draw(tile, _) => Some(tile),
        _ => unreachable!(),
    };

    iter::from_fn(generator)
        .filter(|v| match v {
            Tile::Block => true,
            _ => false,
        })
        .count()
}

pub fn part_2() -> intcode::Memory {
    let mut game = Game::new(&ISTRUCTIONS, Some(2));

    while game.play() != Event::Halt {}

    game.score().unwrap()
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
