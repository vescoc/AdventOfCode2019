use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryInto;

use intcode;

pub type Point = (i32, i32);

#[derive(PartialEq, Copy, Clone)]
pub enum Tile {
    Empty,
    Wall,
    OxygenSystem,
}

impl TryInto<Tile> for intcode::Memory {
    type Error = &'static str;

    fn try_into(self) -> Result<Tile, &'static str> {
        match self {
            0 => Ok(Tile::Wall),
            1 => Ok(Tile::Empty),
            2 => Ok(Tile::OxygenSystem),
            _ => Err("invalid tile"),
        }
    }
}

pub enum Step {
    Searching(HashMap<Point, Tile>, Vec<Point>),
    Found(Point, Vec<Move>, intcode::CPU),
}

#[derive(Debug)]
pub enum Error {
    NotFound(usize),
}

#[derive(Copy, Clone, Debug)]
pub enum Move {
    North,
    South,
    West,
    East,
}

impl Into<Option<intcode::Memory>> for Move {
    fn into(self) -> Option<intcode::Memory> {
        Some(match self {
            Move::North => 1,
            Move::South => 2,
            Move::West => 3,
            Move::East => 4,
        })
    }
}

type State = (intcode::CPU, Point, Vec<Move>);

pub type Goal = fn(Tile) -> bool;

pub struct Search {
    queue: VecDeque<State>,
    visited: HashSet<Point>,
    found: Option<(Point, Vec<Move>, intcode::CPU)>,
    goal: Goal,
    depth: usize,
}

impl Search {
    pub fn new(program: &[intcode::Memory], goal: Goal) -> Self {
        Self {
            queue: vec![(intcode::CPU::new(program.to_vec(), 0, None), (0, 0), vec![])]
                .into_iter()
                .collect(),
            visited: HashSet::new(),
            found: None,
            goal,
            depth: 0,
        }
    }

    pub fn new_from_cpu(cpu: intcode::CPU, start_position: Point, goal: Goal) -> Self {
        Self {
            queue: vec![(cpu, start_position, vec![])].into_iter().collect(),
            visited: HashSet::new(),
            found: None,
            goal,
            depth: 0,
        }
    }

    pub fn step(&mut self) -> Result<Step, Error> {
        if self.found.is_some() {
            let (position, moves, cpu) = self.found.as_ref().unwrap();
            Ok(Step::Found(*position, moves.to_owned(), cpu.to_owned()))
        } else if let Some((cpu, current_position, moves)) = self.queue.pop_front() {
            self.visited.insert(current_position);

            let r = self.next_positions(current_position).try_fold(
                (HashMap::new(), vec![]),
                |mut acc, (p, m)| {
                    let mut cpu = cpu.copy_with_input(m.into());
                    match cpu.run().expect("invalid state") {
                        intcode::Run::NeedInput => Err(Err(())),
                        intcode::Run::Halt => Ok(acc),
                        intcode::Run::Output(value) => match value.try_into() {
                            Ok(Tile::Wall) => Ok({
                                acc.0.insert(p, Tile::Wall);
                                acc
                            }),
                            Ok(v) if !(self.goal)(v) => Ok({
                                acc.0.insert(p, v);
                                acc.1.push((cpu, p, {
                                    let mut moves = moves.to_owned();
                                    moves.push(m);
                                    moves
                                }));
                                acc
                            }),
                            Ok(Tile::OxygenSystem) => Err(Ok((p, {
                                let mut moves = moves.to_owned();
                                moves.push(m);
                                moves
                            }))),
                            _ => Err(Err(())),
                        },
                    }
                },
            );

            match r {
                Err(Ok((position, moves))) => {
                    let moves = moves.into_iter().rev().collect::<Vec<_>>();
                    self.found = Some((position, moves.to_owned(), cpu.to_owned()));
                    Ok(Step::Found(position, moves, cpu))
                }
                Ok((discovered, next)) => Ok({
                    let mut explore = vec![];
                    for (cpu, position, moves) in next {
                        self.depth = self.depth.max(moves.len());
                        self.queue.push_back((cpu, position, moves));
                        explore.push(position);
                    }
                    Step::Searching(discovered, explore)
                }),
                Err(Err(_)) => unreachable!(),
            }
        } else {
            Err(Error::NotFound(self.depth))
        }
    }

    fn next_positions(&self, position: Point) -> impl Iterator<Item = (Point, Move)> {
        const DIRECTIONS: [(Point, Move); 4] = [
            ((0, 1), Move::North),
            ((0, -1), Move::South),
            ((-1, 0), Move::West),
            ((1, 0), Move::East),
        ];

        DIRECTIONS
            .iter()
            .filter_map(|(p, m)| {
                let p = (position.0 + p.0, position.1 + p.1);
                if self.visited.contains(&p) {
                    None
                } else {
                    Some((p, *m))
                }
            })
            .collect::<Vec<(Point, Move)>>()
            .into_iter()
    }
}
