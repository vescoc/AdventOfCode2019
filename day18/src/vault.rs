use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;
use std::ops;
use std::str::FromStr;

pub type Coord = (usize, usize);

#[derive(PartialEq, Copy, Clone, Hash, Eq)]
pub struct CharsSet(u32);

impl fmt::Debug for CharsSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut v = vec![];
        for i in 0..(b'Z' - b'A' + 1) {
            if self.0 & (1 << i) != 0 {
                v.push((b'A' + i) as char);
            }
        }

        write!(f, "{:?}", v)
    }
}

impl CharsSet {
    pub const EMPTY: CharsSet = CharsSet(0);
    pub const ALLS: CharsSet = CharsSet(0b11111111111111111111111111);

    pub fn insert(&mut self, c: char) {
        self.0 |= 1 << (c.to_ascii_uppercase() as u32 - b'A' as u32);
    }

    pub fn union(&self, other: &Self) -> Self {
        CharsSet(self.0 | other.0)
    }

    pub fn intersection(&self, other: &Self) -> Self {
        CharsSet(self.0 & other.0)
    }

    pub fn difference(&self, other: &Self) -> Self {
        CharsSet(self.0 ^ other.0 & self.0)
    }

    pub fn contains(&self, c: char) -> bool {
        self.0 & (1 << (c.to_ascii_uppercase() as u32 - b'A' as u32)) != 0
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn is_alls(&self) -> bool {
        self == &Self::ALLS
    }

    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl ops::Add<char> for CharsSet {
    type Output = CharsSet;

    fn add(self, c: char) -> Self::Output {
        CharsSet(self.0 | 1 << (c.to_ascii_uppercase() as u32 - b'A' as u32))
    }
}

pub struct Vault {
    grid: HashSet<Coord>,
    doors: HashMap<Coord, char>,
    keys: HashMap<Coord, char>,
    pub robots: HashSet<Coord>,
}

impl FromStr for Vault {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut grid = HashSet::new();
        let mut doors = HashMap::new();
        let mut keys = HashMap::new();
        let mut robots = HashSet::new();

        for (y, row) in input.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                match c {
                    '#' => {}
                    '.' => {
                        grid.insert((x, y));
                    }
                    'a'..='z' => {
                        keys.insert((x, y), c);
                        grid.insert((x, y));
                    }
                    'A'..='Z' => {
                        doors.insert((x, y), c);
                        grid.insert((x, y));
                    }
                    '@' => {
                        robots.insert((x, y));
                        grid.insert((x, y));
                    }
                    _ => {
                        return Err("invalid tile");
                    }
                }
            }
        }

        Ok(Vault {
            grid,
            doors,
            keys,
            robots,
        })
    }
}

impl Vault {
    pub fn search<const N: usize>(&self, start: [Coord; N]) -> Result<usize, &'static str> {
        #[derive(PartialEq, Eq)]
        struct Info<const N: usize>(usize, [Coord; N], CharsSet);

        impl<const N: usize> Ord for Info<N> {
            fn cmp(&self, other: &Self) -> Ordering {
                other.0.cmp(&self.0)
            }
        }

        impl<const N: usize> PartialOrd for Info<N> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let all_keys = self.keys.iter().fold(CharsSet::EMPTY, |mut acc, (_, key)| {
            acc.insert(*key);
            acc
        });

        let mut search_cache = HashMap::new();

        let mut costs = HashMap::new();
        let mut queue = BinaryHeap::new();

        costs.insert((start, CharsSet::EMPTY), 0);
        queue.push(Info(0, start, CharsSet::EMPTY));

        while let Some(Info(current_cost, coords, current_keys)) = queue.pop() {
            let cost = costs[&(coords, current_keys)];
            if current_keys == all_keys {
                return Ok(cost);
            }

            if current_cost > cost {
                continue;
            }

            for (i, coord) in coords.iter().enumerate() {
                let neighbors = search_cache
                    .entry(*coord)
                    .or_insert_with(|| self.dijkstra(*coord));

                for ((key, coord), cost) in Vault::filter_neighbors(neighbors, current_keys) {
                    let total_cost = current_cost + *cost;

                    let keys = {
                        let mut keys = current_keys;
                        keys.insert(*key);
                        keys
                    };

                    let mut coords = coords;
                    coords[i] = *coord;

                    let cost = costs.entry((coords, keys)).or_insert(usize::MAX);

                    if total_cost < *cost {
                        queue.push(Info(total_cost, coords, keys));
                        *cost = total_cost;
                    }
                }
            }
        }

        Err("not found")
    }

    fn filter_neighbors(
        paths: &HashMap<(char, Coord), (usize, CharsSet, CharsSet)>,
        keys: CharsSet,
    ) -> impl Iterator<Item = (&(char, Coord), &usize)> {
        paths
            .iter()
            .filter_map(move |(k, (neighbor_cost, neighbor_doors, neighbor_keys))| {
                if keys.is_superset(neighbor_doors)
                    && neighbor_keys.difference(&keys) == CharsSet::EMPTY
                {
                    Some((k, neighbor_cost))
                } else {
                    None
                }
            })
    }

    fn dijkstra(&self, start: Coord) -> HashMap<(char, Coord), (usize, CharsSet, CharsSet)> {
        fn remove_min(
            q: &mut HashSet<Coord>,
            costs: &HashMap<Coord, (usize, Option<Coord>)>,
        ) -> Option<(Coord, (usize, Option<Coord>))> {
            if let Some(&c) = q.iter().min_by(|c1, c2| costs[c1].0.cmp(&costs[c2].0)) {
                q.remove(&c);
                Some((c, costs[&c]))
            } else {
                None
            }
        }

        fn make_char_set(
            map: &HashMap<Coord, char>,
            visited: &HashMap<Coord, (usize, Option<Coord>)>,
            mut start: Option<Coord>,
        ) -> CharsSet {
            let mut set = CharsSet::EMPTY;
            while let Some(coord) = start {
                if let Some(c) = map.get(&coord) {
                    set.insert(*c);
                }
                start = visited.get(&coord).unwrap().1;
            }
            set
        }

        let mut q = HashSet::new();
        let mut visited = HashMap::new();

        q.insert(start);
        visited.insert(start, (0, None));

        while let Some(((x, y), (cost, _))) = remove_min(&mut q, &visited) {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let target = ((x as isize + dx) as usize, (y as isize + dy) as usize);

                let target_cost = cost + 1;

                if q.contains(&target) {
                    let (old_cost, previous) = visited.get_mut(&target).unwrap();
                    if *old_cost > target_cost {
                        *old_cost = target_cost;
                        previous.replace((x, y));
                    }
                } else if self.grid.contains(&target) {
                    let (old_cost, previous) = visited.entry(target).or_insert_with(|| {
                        q.insert(target);
                        (target_cost, Some((x, y)))
                    });
                    if *old_cost > target_cost {
                        *old_cost = target_cost;
                        previous.replace((x, y));
                    }
                }
            }
        }

        self.keys
            .iter()
            .flat_map(|(&coord, &key)| {
                visited
                    .get(&coord)
                    .map(|(cost, previous)| {
                        (
                            (key, coord),
                            (
                                *cost,
                                make_char_set(&self.doors, &visited, *previous),
                                make_char_set(&self.keys, &visited, *previous),
                            ),
                        )
                    })
                    .into_iter()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! charsset {
        () => { $crate::vault::CharsSet::EMPTY };
        ($($e:expr),*) => {
            {
                let mut set = $crate::vault::CharsSet::EMPTY;
                { $(set.insert($e);)* }
                set
            }
        };
    }

    fn print_paths<K: fmt::Debug, V: fmt::Debug>(paths: &HashMap<K, V>) {
        for (i, path) in paths.iter().enumerate() {
            println!("{i}: {:?}", path);
        }
    }

    #[test]
    fn test_dijkstra_1_1() {
        let vault = include_str!("../example1-1.txt").parse::<Vault>().unwrap();

        let paths = vault.dijkstra(*vault.robots.iter().next().unwrap());

        print_paths(&paths);

        assert_eq!(paths[&('a', (7, 1))], (2, charsset![], charsset![]));
        assert_eq!(paths[&('b', (1, 1))], (4, charsset!['A'], charsset![]));
    }

    #[test]
    fn test_dijkstra_1_2() {
        let vault = include_str!("../example1-2.txt").parse::<Vault>().unwrap();

        let paths = vault.dijkstra(*vault.robots.iter().next().unwrap());

        print_paths(&paths);
    }

    #[test]
    fn test_filter_neighbors() {
        let vault = include_str!("../example1-2.txt").parse::<Vault>().unwrap();

        let paths = vault.dijkstra(*vault.robots.iter().next().unwrap());

        assert_eq!(
            Vault::filter_neighbors(&paths, charsset![]).collect::<Vec<_>>(),
            vec![(&('a', (17, 1)), &2)]
        );

        assert_eq!(Vault::filter_neighbors(&paths, charsset!['a']).count(), 2);
    }
}
