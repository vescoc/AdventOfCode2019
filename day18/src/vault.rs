use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::str::FromStr;

type Point = (usize, usize);

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    Door(char),
    Key(char),
}

#[derive(PartialEq, Eq, Hash)]
struct State(Point, Vec<char>);

impl State {
    fn new(point: Point, keys: &[char]) -> Self {
        let mut keys = keys.to_owned();
        keys.sort();
        Self(point, keys)
    }
}

pub struct Vault {
    tiles: Vec<Vec<Tile>>,
    start_position: Point,
    keys: Vec<char>,
}

impl Vault {
    fn width(&self) -> usize {
        self.tiles[0].len()
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn get(&self, x: usize, y: usize) -> Option<Tile> {
        self.tiles.get(y).and_then(|v| v.get(x).copied())
    }

    fn find_paths_to_keys(
        &self,
        start_position: Point,
        discovered_keys: &[char],
    ) -> Vec<Vec<Point>> {
        let mut remaining_keys = self
            .keys
            .iter()
            .copied()
            .collect::<HashSet<char>>()
            .difference(&discovered_keys.iter().copied().collect::<HashSet<char>>())
            .copied()
            .collect::<HashSet<char>>();

        let mut result = vec![];

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        let next = |(x, y), visited: &HashSet<Point>, remaining_keys: &HashSet<char>| {
            vec![(1, 0), (-1, 0), (0, 1), (0, -1)]
                .iter()
                .map(|(dx, dy)| ((x as i128 + dx) as usize, (y as i128 + dy) as usize))
                .filter(|p| !visited.contains(p))
                .filter_map(|(x, y)| match self.get(x, y) {
                    Some(Tile::Empty) => Some(((x, y), None)),
                    Some(Tile::Door(c)) if discovered_keys.contains(&c) => Some(((x, y), None)),
                    Some(Tile::Key(c)) => Some((
                        (x, y),
                        if remaining_keys.contains(&c) {
                            None
                        } else {
                            Some(c)
                        },
                    )),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .into_iter()
        };

        queue.push_back((start_position, vec![]));
        loop {
            if remaining_keys.is_empty() {
                break result;
            } else if let Some((position, moves)) = queue.pop_front() {
                visited.insert(position);
                next(position, &visited, &remaining_keys).for_each(|(new_position, new_key)| {
                    match new_key {
                        Some(new_key) => {
                            result.push({
                                let mut moves = moves.to_owned();
                                moves.push(new_position);
                                moves
                            });

                            remaining_keys.remove(&new_key);
                        }
                        None => {
                            queue.push_back((new_position, {
                                let mut moves = moves.to_owned();
                                moves.push(new_position);
                                moves
                            }));
                        }
                    }
                });
            } else {
                break result;
            }
        }
    }

    pub fn search(&self) -> Result<(Vec<Point>, Vec<char>), String> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        if true {
            unimplemented!();
        }

        let next = |(x, y), keys: &[char], visited: &HashSet<State>| {
            vec![(1, 0), (-1, 0), (0, 1), (0, -1)]
                .iter()
                .map(|(dx, dy)| ((x as i128 + dx) as usize, (y as i128 + dy) as usize))
                .filter(|(x, y)| !visited.contains(&State::new((*x, *y), keys)))
                .filter_map(|(x, y)| match self.get(x, y) {
                    Some(Tile::Empty) => Some(((x, y), None)),
                    Some(Tile::Door(c)) if keys.contains(&c) => Some(((x, y), None)),
                    Some(Tile::Key(c)) => {
                        Some(((x, y), if keys.contains(&c) { None } else { Some(c) }))
                    }
                    _ => None,
                })
                .collect::<Vec<_>>()
                .into_iter()
        };

        queue.push_back((self.start_position, vec![], vec![self.start_position]));
        loop {
            let (position, keys, moves) = queue
                .pop_front()
                .ok_or_else(|| String::from("no solution!"))?;
            //println!("analyze [{}] position: {:?}, keys: {:?}, moves: {:?}", moves.len(), position, keys, moves);
            if self.keys == {
                let mut keys = keys.to_owned();
                keys.sort();
                keys
            } {
                break Ok((moves, keys));
            } else {
                visited.insert(State::new(position, &keys));
                next(position, &keys, &visited).for_each(|(new_position, new_key)| match new_key {
                    Some(new_key) => {
                        queue.push_back((
                            new_position,
                            {
                                let mut keys = keys.to_owned();
                                keys.push(new_key);
                                keys
                            },
                            {
                                let mut moves = moves.to_owned();
                                moves.push(new_position);
                                moves
                            },
                        ));
                    }
                    None => {
                        queue.push_back((new_position, keys.to_owned(), {
                            let mut moves = moves.to_owned();
                            moves.push(new_position);
                            moves
                        }));
                    }
                });
            }
        }
    }
}

impl fmt::Display for Vault {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for y in 0..self.height() {
            for x in 0..self.width() {
                match self.get(x, y).unwrap() {
                    Tile::Empty => write!(fmt, ".")?,
                    Tile::Wall => write!(fmt, "#")?,
                    Tile::Door(c) => write!(fmt, "{}", c.to_ascii_uppercase())?,
                    Tile::Key(c) => write!(fmt, "{}", c)?,
                }
            }

            writeln!(fmt)?;
        }

        writeln!(fmt, "start position: {:?}", self.start_position)?;
        write!(fmt, "keys: {:?}", self.keys)
    }
}

impl FromStr for Vault {
    type Err = String;

    fn from_str(data: &str) -> Result<Self, String> {
        let (tiles, start_position, mut keys) = data
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.trim()
                    .chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => Ok((Tile::Wall, None, None)),
                        '.' => Ok((Tile::Empty, None, None)),
                        'a'..='z' => Ok((Tile::Key(c), None, Some(c))),
                        'A'..='Z' => Ok((Tile::Door(c.to_ascii_lowercase()), None, None)),
                        '@' => Ok((Tile::Empty, Some((x, y)), None)),
                        _ => Err(format!("invalid char {} at ({}, {})", c, x, y)),
                    })
                    .try_fold(
                        (vec![], None, vec![]),
                        |(mut v, position, mut keys), r| match r {
                            Ok((t, p, key)) => match (p, position, key) {
                                (Some(_), Some(_), _) => Err(format!(
                                    "duplicate start position, now {:?}, old {:?}",
                                    p, position
                                )),
                                (Some(_), None, key) => {
                                    v.push(t);
                                    if let Some(key) = key {
                                        keys.push(key);
                                    }
                                    Ok((v, p, keys))
                                }
                                (None, _, key) => {
                                    v.push(t);
                                    if let Some(key) = key {
                                        keys.push(key);
                                    }
                                    Ok((v, position, keys))
                                }
                            },
                            Err(e) => Err(e),
                        },
                    )
            })
            .try_fold(
                (vec![], None, vec![]),
                |(mut v, position, mut keys), r| match r {
                    Ok((t, p, mut ks)) => match (p, position) {
                        (Some(_), Some(_)) => Err(format!(
                            "duplicate start position, now {:?}, old {:?}",
                            p, position
                        )),
                        (Some(_), None) => {
                            v.push(t);
                            keys.append(&mut ks);
                            Ok((v, p, keys))
                        }
                        (None, _) => {
                            v.push(t);
                            keys.append(&mut ks);
                            Ok((v, position, keys))
                        }
                    },
                    Err(e) => Err(e),
                },
            )?;

        keys.sort();

        Ok(Self {
            tiles,
            start_position: start_position
                .ok_or_else(|| String::from("cannot find start_position"))?,
            keys,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_1() {
        let vault = r"#########
#b.A.@.a#
#########"
            .parse::<Vault>()
            .unwrap();

        assert_eq!(
            vault.find_paths_to_keys(vault.start_position, &['a']),
            vec![vec![(6, 1), (7, 1)]],
        )
    }
}
