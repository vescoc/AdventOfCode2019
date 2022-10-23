use lazy_static::lazy_static;

mod vault;
pub use vault::*;

lazy_static! {
    static ref DATA: &'static str = include_str!("../data.txt");
}

fn solve_1(data: &str) -> usize {
    let vault = data
        .parse::<Vault>()
        .unwrap_or_else(|e| panic!("invalid data: {}", e));

    vault.search().unwrap()
}

pub fn part_1() -> usize {
    solve_1(&DATA)
}

pub fn part_2() -> usize {
    unimplemented!()
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
        assert_eq!(solve_1(include_str!("../example1-3.txt")), 132,)
    }

    #[test]
    fn test_example_1_4() {
        assert_eq!(solve_1(include_str!("../example1-4.txt")), 136,)
    }
}
