use std::collections::{HashSet, HashMap, VecDeque};
use std::fmt;
use std::str::FromStr;

type Point = (usize, usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
    key_positions: HashMap<Tile, Point>,
    door_positions: HashMap<Tile, Point>,
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

    fn find_paths_from(
	&self,
	start_position: &Point,
    ) -> Vec<(char, HashSet<char>, HashSet<char>, usize)> {
	let mut remaining_keys = self.keys_set.to_owned();

	let mut result = vec![];

	let mut queue = VecDeque::new();
	let mut visited = HashSet::new();

	let next = |(x, y), visited: &HashSet<Point>| {
	    vec![(1, 0), (-1, 0), (0, 1), (0, -1)]
		.iter()
		.map(|(dx, dy)| ((x as i128 + dx) as usize, (y as i128 + dy) as usize))
		.filter(|p| !visited.contains(p))
		.filter_map(|(x, y)| match self.get(x, y) {
		    Some(Tile::Empty) => Some(((x, y), None)),
		    Some(t @ Tile::Door(_)) => Some(((x, y), Some(t))),
		    Some(t @ Tile::Key(_)) => Some(((x, y), Some(t))),
		    _ => None,			 
		})
		.collect::<Vec<_>>()
		.into_iter()
	};

	let mut keys = HashSet::new();
	let mut doors = HashSet::new();

	match self.get(start_position.0, start_position.1) {
	    Some(Tile::Key(k)) => { keys.insert(k); }
	    Some(Tile::Door(d)) => { doors.insert(d); }
	    _ => {}
	}

	queue.push_back((*start_position, keys, doors, 0));
        loop {
            if remaining_keys.is_empty() {
                break result;
            } else if let Some((position, keys, doors, moves)) = queue.pop_front() {
                visited.insert(position);
                next(position, &visited).for_each(|(new_position, tile)| {
                    match tile {
			Some(Tile::Key(k)) => {
			    let mut keys = keys.to_owned();
			    keys.insert(k);
			    
			    result.push((k, keys.to_owned(), doors.to_owned(), moves + 1));

			    remaining_keys.remove(&k);

			    queue.push_back((new_position, keys, doors.to_owned(), moves + 1));
			}
			Some(Tile::Door(d)) => {
			    let mut doors = doors.to_owned();
			    doors.insert(d);

			    queue.push_back((new_position, keys.to_owned(), doors.to_owned(), moves + 1));
			}
                        _ => queue.push_back((new_position, keys.to_owned(), doors.to_owned(), moves + 1)),
                    }
                });
            } else {
                break result;
            }
        }
    }

    pub fn search(&self) -> Result<(usize, Vec<char>), String> {
	unimplemented!()
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
        writeln!(fmt, "keys: {:?}", self.keys)?;
	writeln!(fmt, "key positions: {:?}", self.key_positions)?;
	writeln!(fmt, "door position: {:?}", self.door_positions)
    }
}

impl FromStr for Vault {
    type Err = String;

    fn from_str(data: &str) -> Result<Self, String> {
        let (tiles, start_position, mut keys, door_positions, key_positions) = data
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.trim()
                    .chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => Ok((Tile::Wall, (x, y), None)),
                        '.' => Ok((Tile::Empty, (x, y), None)),
                        'a'..='z' => Ok((Tile::Key(c), (x, y), None)),
                        'A'..='Z' => Ok((Tile::Door(c.to_ascii_lowercase()), (x, y), None)),
                        '@' => Ok((Tile::Empty, (x, y), Some((x, y)))),
                        _ => Err(format!("invalid char {} at ({}, {})", c, x, y)),
                    })
                    .try_fold(
                        (vec![], None, vec![], vec![], vec![]),
                        |(mut v, position, mut keys, mut door_positions, mut key_positions), r| match r {
                            Ok((t, p, sp)) => {
				match t {
				    Tile::Key(k) => {
					keys.push(k);
					key_positions.push((t, p));
				    }
				    Tile::Door(_) => {
					door_positions.push((t, p));
				    }
				    _ => {},
				}
				v.push(t);
				Ok((v, position.or(sp), keys, door_positions, key_positions))
			    }
                            Err(e) => Err(e),
                        },
                    )
            })
            .try_fold(
                (vec![], None, vec![], vec![], vec![]),
                |(mut tiles, position, mut keys, mut door_positions, mut key_positions), r| match r {
                    Ok((ts, p, mut k, mut dp, mut kp)) => {
			tiles.push(ts);
			keys.append(&mut k);
			door_positions.append(&mut dp);
			key_positions.append(&mut kp);
			Ok((tiles,
			    if position.is_some() { position } else { p },
			    keys,
			    door_positions,
			    key_positions))
		    }
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
	    door_positions: door_positions.into_iter().collect(),
	    key_positions: key_positions.into_iter().collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_paths_from() {
        let vault = r"#########
#b.A.@.a#
#########"
            .parse::<Vault>()
            .unwrap();

        assert_eq!(
            vault.find_paths_from(&vault.start_position),
            vec![('a', vec!['a'].into_iter().collect(), HashSet::new(), 2), ('b', vec!['b'].into_iter().collect(), vec!['a'].into_iter().collect(), 4)],
        )
    }
}
