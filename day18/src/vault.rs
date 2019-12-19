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

pub struct Vault {
    tiles: Vec<Vec<Tile>>,
    start_position: Point,
    keys: Vec<char>,
    keys_set: HashSet<char>,
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
        start_position: &Point,
        discovered_keys: &HashSet<char>,
	prune: usize,
    ) -> Vec<(char, Vec<Point>)> {
        let mut remaining_keys = self
            .keys_set
            .difference(discovered_keys)
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
                            Some(c)
                        } else {
                            None
                        },
                    )),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .into_iter()
        };

        queue.push_back((*start_position, vec![]));
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
                                (new_key, moves)
                            });

                            remaining_keys.remove(&new_key);
                        }
                        None => {
			    if moves.len() + 1 < prune {
				queue.push_back((new_position, {
                                    let mut moves = moves.to_owned();
                                    moves.push(new_position);
                                    moves
				}));
			    }
                        }
                    }
                });
            } else {
                break result;
            }
        }
    }

    pub fn search(&self) -> Result<(Vec<Point>, Vec<char>), String> {
	let mut candidate: Option<(Vec<Point>, Vec<char>)> = None;
	let mut queue = vec![]; // VecDeque::new();

	queue.push((vec![self.start_position], vec![]));
	loop {
	    if let Some((path, discovered_keys)) = queue.pop() {
		let prune = candidate.as_ref().map(|(v, _)| v.len()).unwrap_or(std::usize::MAX);
		let path_len = path.len();
		//println!("{} [{}] {:?} [{}]", prune, path_len, discovered_keys, queue.len());
		if path_len < prune {
		    let discovered_keys_set = discovered_keys.iter().copied().collect::<HashSet<char>>();
		    for (c, mut path_to) in self.find_paths_to_keys(path.last().unwrap(), &discovered_keys_set, prune - path_len) {
			let mut discovered_keys = discovered_keys.to_owned();
			discovered_keys.push(c);

			let mut discovered_keys_set = discovered_keys_set.to_owned();
			discovered_keys_set.insert(c);

			let mut path = path.to_owned();
			path.append(&mut path_to);
			
			if self.keys_set == discovered_keys_set {
			    candidate = Some((path, discovered_keys));
			} else {
			    queue.push((path, discovered_keys));
			}
		    }
		}
	    } else {
		break;
	    }
	}

	candidate.ok_or_else(|| String::from("no solution!"))
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
            keys: keys.to_owned(),
	    keys_set: keys.into_iter().collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_path_simple() {
        let vault = r"#########
#b.A.@.a#
#########"
            .parse::<Vault>()
            .unwrap();

        assert_eq!(
            vault.find_paths_to_keys(&vault.start_position,
				     &HashSet::new(),
				     std::usize::MAX),
            vec![('a', vec![(6, 1), (7, 1)])],
        )
    }

    #[test]
    fn test_find_path_discovered() {
        let vault = r"#########
#b.A.@.a#
#########"
            .parse::<Vault>()
            .unwrap();

        assert_eq!(
            vault.find_paths_to_keys(&vault.start_position,
				     &['a'].iter().copied().collect(),
				     std::usize::MAX),
            vec![('b', vec![(4, 1), (3, 1), (2, 1), (1, 1)])],
        )
    }

    #[test]
    fn test_find_path_prune() {
        let vault = r"#########
#b.A.@.a#
#########"
            .parse::<Vault>()
            .unwrap();

        assert_eq!(
            vault.find_paths_to_keys(&vault.start_position,
				     &['a'].iter().copied().collect(),
				     3),
            Vec::<(char, Vec<_>)>::new(),
        )
    }

    #[test]
    fn test_find_path_prune_hit() {
        let vault = r"#########
#b.A.@.a#
#########"
            .parse::<Vault>()
            .unwrap();

        assert_eq!(
            vault.find_paths_to_keys(&vault.start_position,
				     &['a'].iter().copied().collect(),
				     5),
            vec![('b', vec![(4, 1), (3, 1), (2, 1), (1, 1)])],
        )
    }
}
