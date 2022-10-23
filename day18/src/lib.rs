use lazy_static::lazy_static;

mod vault;
pub use vault::*;

lazy_static! {
    static ref DATA: &'static str = include_str!("../data.txt");
    static ref DATA2: &'static str = include_str!("../data2.txt");
}

fn solve_1(data: &str) -> usize {
    let vault = data
        .parse::<Vault>()
        .unwrap_or_else(|e| panic!("invalid data: {}", e));

    let robots: [Coord; 1] = vault
        .robots
        .iter()
        .copied()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    vault.search(robots).unwrap()
}

fn solve_2(data: &str) -> usize {
    let vault = data
        .parse::<Vault>()
        .unwrap_or_else(|e| panic!("invalid data: {}", e));

    let robots: [Coord; 4] = vault
        .robots
        .iter()
        .copied()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    vault.search(robots).unwrap()
}

pub fn part_1() -> usize {
    solve_1(&DATA)
}

pub fn part_2() -> usize {
    solve_2(&DATA2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_1() {
        assert_eq!(solve_1(include_str!("../example1-1.txt")), 8)
    }

    #[test]
    fn test_example_1_2() {
        assert_eq!(solve_1(include_str!("../example1-2.txt")), 86)
    }

    #[test]
    fn test_example_1_3() {
        assert_eq!(solve_1(include_str!("../example1-3.txt")), 132)
    }

    #[test]
    fn test_example_1_4() {
        assert_eq!(solve_1(include_str!("../example1-4.txt")), 136)
    }

    #[test]
    fn test_example_2_1() {
        assert_eq!(solve_2(include_str!("../example2-1.txt")), 8)
    }
}
