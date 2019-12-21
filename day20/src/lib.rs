#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

lazy_static! {
    static ref DATA: &'static str = include_str!("../data.txt");
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

type Point = (usize, usize);
type TeleportID = [char; 2];

enum Tile {
    Empty,
    Teleport(TeleportID),
}

struct Maze {
    map: HashMap<Point, Tile>,
    teleports: HashMap<TeleportID, Vec<Point>>,
}

impl Maze {
    fn search(&self) -> Result<Vec<Point>, String> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((self.teleports[&['A', 'A']][0], vec![]));
        loop {
            if let Some((current_position, current_moves)) = queue.pop_front() {
                visited.insert(current_position);

                for (dx, dy) in DIRECTIONS.iter() {
                    let next_position = (
                        (current_position.0 as i32 + dx) as usize,
                        (current_position.1 as i32 + dy) as usize,
                    );
                    match self.map.get(&next_position) {
                        Some(Tile::Empty) => {
                            if !visited.contains(&next_position) {
                                queue.push_back((next_position, {
                                    let mut moves = current_moves.to_owned();
                                    moves.push(current_position.to_owned());
                                    moves
                                }));
                            }
                        }
                        Some(Tile::Teleport(teleport)) => {
                            if teleport == &['Z', 'Z'] {
                                return Ok({
                                    let mut moves = current_moves;
                                    moves.push(current_position);
                                    moves
                                });
                            } else if let Some(&next_position) = self.teleports[teleport]
                                .iter()
                                .find(|&p| *p != current_position)
                            {
                                if !visited.contains(&next_position) {
                                    queue.push_back((next_position, {
                                        let mut moves = current_moves.to_owned();
                                        moves.push(current_position.to_owned());
                                        moves
                                    }));
                                }
                            }
                        }
                        None => {}
                    }
                }
            } else {
                break Err(String::from("no solution!"));
            }
        }
    }
}

impl FromStr for Maze {
    type Err = String;

    fn from_str(data: &str) -> Result<Self, String> {
        let data = data
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();

        let mut set = vec![];
        for (y, row) in data.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if c == '.' {
                    set.push((x, y));
                }
            }
        }

        let mut map = HashMap::new();
        let mut teleports: HashMap<TeleportID, Vec<Point>> = HashMap::new();
        for (x, y) in set {
            let get = |dx, dy| {
                data.get((y as i32 + dy) as usize)
                    .and_then(|row| row.get((x as i32 + dx) as usize))
            };

            map.insert((x, y), Tile::Empty);
            for (dx, dy) in &DIRECTIONS {
                if let Some(&c0) = get(*dx, *dy) {
                    if c0.is_ascii_uppercase() {
                        if let Some(&c1) = get(*dx * 2, *dy * 2) {
                            let teleport = match (dx, dy) {
                                (-1, 0) | (0, -1) => [c1, c0],
                                _ => [c0, c1],
                            };
                            let p = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                            teleports.entry(teleport).or_default().push((x, y));
                            map.insert(p, Tile::Teleport(teleport));
                        } else {
                            return Err(String::from("invalid teleport"));
                        }
                    }
                }
            }
        }

        Ok(Maze { map, teleports })
    }
}

fn solve_1(data: &str) -> usize {
    data.parse::<Maze>().unwrap().search().unwrap().len() - 1
}

pub fn part_1() -> usize {
    solve_1(&DATA)
}

pub fn part_2() -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_example_1_1() {
        let maze = r"         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       "
            .parse::<Maze>()
            .expect("invalid data");

        println!("{:?}", maze.teleports);

        let path = maze.search().expect("invalid");
        println!("path {:?}", path);

        assert_eq!(path.len() - 1, 23);
    }

    #[test]
    fn test_example_1_2() {
        let maze = r"                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ";

        assert_eq!(solve_1(maze), 58);
    }

    #[test]
    fn test_maze() {
        let maze = r"         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       "
            .parse::<Maze>()
            .expect("invalid data");

        assert_eq!(maze.teleports.len(), 5);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part_1);
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part_2);
    }
}
