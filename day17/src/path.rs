use std::collections::{HashMap, HashSet};
use std::str::FromStr;

type Point = (i32, i32);

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Copy, Clone, Debug)]
pub enum Move {
    Left,
    Right,
    Forward,
}

#[derive(Debug)]
pub struct Path {
    path: Vec<Point>,
    moves: Vec<Move>,
    start_position: Point,
    start_direction: Direction,
}

impl Path {
    pub fn intersections(&self) -> impl Iterator<Item = &Point> {
        self.path
            .iter()
            .fold(HashMap::<&Point, u32>::new(), |mut acc, p| {
                *acc.entry(p).or_default() += 1;
                acc
            })
            .into_iter()
            .filter_map(|(k, v)| if v > 1 { Some(k) } else { None })
    }

    pub fn moves(&self) -> &Vec<Move> {
        &self.moves
    }
}

impl FromStr for Path {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, &'static str> {
        let (position_info, map) = input.lines().enumerate().try_fold(
            (None, HashSet::new()),
            |(mut position_info, mut map), (y, l)| {
                l.trim()
                    .chars()
                    .enumerate()
                    .try_for_each(|(x, c)| {
                        let (x, y): (i32, i32) = (x as i32, y as i32);
                        match (&position_info, c) {
                            (_, '#') => {
                                map.insert((x, y));
                                Ok(())
                            }
                            (_, '.') => Ok(()),
                            (None, '^') => {
                                map.insert((x, y));
                                position_info = Some(((x, y), Direction::North));
                                Ok(())
                            }
                            (None, '<') => {
                                map.insert((x, y));
                                position_info = Some(((x, y), Direction::West));
                                Ok(())
                            }
                            (None, 'v') => {
                                map.insert((x, y));
                                position_info = Some(((x, y), Direction::South));
                                Ok(())
                            }
                            (None, '>') => {
                                map.insert((x, y));
                                position_info = Some(((x, y), Direction::East));
                                Ok(())
                            }
                            (Some(_), c) if c == '^' || c == '<' || c == '>' || c == 'v' => {
                                Err("start position already found")
                            }
                            _ => Err("invalid char"),
                        }
                    })
                    .map(|_| (position_info, map))
            },
        )?;

        if let Some((start_position, start_direction)) = position_info {
            let mut path = vec![];
            let mut moves = vec![];
            let mut position = start_position;
            let mut direction = start_direction;
            loop {
                path.push(position);
                let target_position = next(position, direction);
                if map.contains(&target_position) {
                    position = target_position;
                    moves.push(Move::Forward);
                } else {
                    let mut v = choose_directions(direction).filter_map(|(d, m)| {
                        let p = next(position, d);
                        if map.contains(&p) {
                            Some((p, d, m))
                        } else {
                            None
                        }
                    });
                    match (v.next(), v.next()) {
                        (Some((new_position, new_direction, m)), None) => {
                            position = new_position;
                            direction = new_direction;
                            moves.push(m);
                            moves.push(Move::Forward);
                        }
                        (_, Some(_)) => panic!("path ambiguity"),
                        (None, None) => break,
                    }
                }
            }

            Ok(Self {
                path,
                moves,
                start_position,
                start_direction,
            })
        } else {
            Err("cannot find start position")
        }
    }
}

fn next(position: Point, direction: Direction) -> Point {
    match direction {
        Direction::North => (position.0, position.1 - 1),
        Direction::South => (position.0, position.1 + 1),
        Direction::West => (position.0 - 1, position.1),
        Direction::East => (position.0 + 1, position.1),
    }
}

fn choose_directions(direction: Direction) -> impl Iterator<Item = (Direction, Move)> {
    match direction {
        Direction::North => vec![
            (Direction::West, Move::Left),
            (Direction::East, Move::Right),
        ]
        .into_iter(),
        Direction::South => vec![
            (Direction::West, Move::Right),
            (Direction::East, Move::Left),
        ]
        .into_iter(),
        Direction::West => vec![
            (Direction::North, Move::Right),
            (Direction::South, Move::Left),
        ]
        .into_iter(),
        Direction::East => vec![
            (Direction::North, Move::Left),
            (Direction::South, Move::Right),
        ]
        .into_iter(),
    }
}
