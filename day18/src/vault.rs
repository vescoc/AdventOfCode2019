use std::collections::{HashSet, HashMap, VecDeque};
use std::cell::RefCell;
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

#[derive(PartialEq, Hash, Eq, Debug)]
struct PathKey(Point, Vec<char>);

impl PathKey {
    fn new(position: &Point, keys: &HashSet<char>) -> Self {
	let mut keys = keys.iter().copied().collect::<Vec<char>>();
	keys.sort();

	Self(*position, keys)
    }
}

#[derive(PartialEq, Hash, Eq, Debug)]
struct GraphKey(Vec<Point>, Vec<char>);

impl GraphKey {
    fn new(positions: &[Point], keys: &HashSet<char>) -> Self {
	let mut keys = keys.iter().copied().collect::<Vec<char>>();
	keys.sort();

	Self(positions.to_vec(), keys)
    }
}

pub struct Vault {
    tiles: Vec<Vec<Tile>>,
    start_position: Point,
    keys: Vec<char>,
    keys_set: HashSet<char>,
    key_positions: HashMap<char, Point>,
    door_positions: HashMap<char, Point>,
    path_cache: RefCell<HashMap<PathKey, HashMap<char, usize>>>,
    graph_cache: RefCell<HashMap<GraphKey, HashMap<char, usize>>>,
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

    fn bfs(&self, start_position: &Point, keys: &HashSet<char>) -> HashMap<char, usize> {
	let path_key = PathKey::new(start_position, keys);
	if let Some(hit) = self.path_cache.borrow().get(&path_key) {
	    println!("paths_cache hit {:?}", path_key);
	    return hit.to_owned();
	}

	let mut result = vec![];
	let mut visited = HashSet::new();
	let mut queue = VecDeque::new();

	queue.push_back((*start_position, 0));
	while let Some(((x, y), moves)) = queue.pop_front() {
	    visited.insert((x, y));
	    [(1, 0), (-1, 0), (0, 1), (0, -1)]
		.iter()
		.filter_map(|(dx, dy)| {
		    let (x, y) = ((x as isize + dx) as usize, ((y as isize + dy) as usize));
		    match self.get(x, y) {
			_ if visited.contains(&(x, y)) => None,
			Some(Tile::Empty) => Some(((x, y), Tile::Empty)),
			Some(Tile::Door(d)) if keys.contains(&d) => Some(((x, y), Tile::Empty)),
			Some(Tile::Key(k)) if !keys.contains(&k) => Some(((x, y), Tile::Key(k))),
			_ => None,
		    }
		})
		.for_each(|((x, y), t)| {
		    match t {
			Tile::Key(k) => {
			    result.push((k, moves + 1));
			}
			_ => {
			    queue.push_back(((x, y), moves + 1));
			}
		    }
		});
	}

	{
	    let mut cache = self.path_cache.borrow_mut();
	    println!("paths_cache fill {:?} {}", path_key, cache.len() + 1);
	    cache.insert(path_key, result.iter().copied().collect());
	}

	result.into_iter().collect()
    }

    fn search_subgraph(&self, start_positions: &[Point], keys: &HashSet<char>) -> HashMap<Vec<char>, usize> {
	let graph_key = GraphKey::new(start_positions, keys);
	if let Some(hit) = self.graph_cache.borrow().get(&graph_key) {
	    println!("graph_cache hit {:?}", graph_key);
	    return hit;
	}

	start_positions
	    .iter()
	    .map(|position| {
		self.bfs(position, keys)
		    .into_iter()
		    .map(|(key, steps)| {
			self.search_subgraph(
			    self.key_positions[key],
			    {
				let keys = keys.to_owned();
				keys.insert(key);
				keys
			    },
			)
			    .into_iter()
			    .map(|(v, sub_steps)| 
	
	{
	    let mut cache = self.graph_cache.borrow_mut();
	    println!("graph_cache fill {:?} {}", graph_key, cache.len() + 1);
	    cache.insert(graph_key, result.iter().copied().collect());
	}

	result.into_iter().collect()
    }

    pub fn search(&self) -> Result<(usize, Vec<char>), String> {
	todo!()
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
					key_positions.push((k, p));
				    }
				    Tile::Door(d) => {
					door_positions.push((d, p));
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
	    path_cache: RefCell::new(HashMap::new()),
	    graph_cache: RefCell::new(HashMap::new()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
	static ref VAULT: &'static str = r"#########
#b.A.@.a#
#########";
    }
    
    #[test]
    fn test_bfs() {
        let vault = VAULT.parse::<Vault>()
            .unwrap();

	assert_eq!(
	    vault.bfs(&vault.start_position, &HashSet::new()),
	    vec![('a', 2)].into_iter().collect(),
	);
    }
    
    #[test]
    fn test_bfs_b() {
        let vault = VAULT.parse::<Vault>()
            .unwrap();

	assert_eq!(
	    vault.bfs(&vault.start_position, &vec!['a'].into_iter().collect()),
	    vec![('b', 4)].into_iter().collect(),
	);
    }
    
    #[test]
    fn test_bfs_b_cache() {
        let vault = VAULT.parse::<Vault>()
            .unwrap();

	let keys = vec!['a'].into_iter().collect();
	vault.bfs(&vault.start_position, &keys);
	
	assert_eq!(
	    vault.bfs(&vault.start_position, &keys),
	    vec![('b', 4)].into_iter().collect(),
	);
    }
}
