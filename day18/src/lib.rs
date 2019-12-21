#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

mod vault;

lazy_static! {
    static ref DATA: &'static str = include_str!("../data.txt");
}

fn solve_1(data: &str) -> (usize, Vec<char>) {
    let vault = data
        .parse::<vault::Vault>()
        .unwrap_or_else(|e| panic!("invalid data: {}", e));

    println!("{}", vault);

    if true {
	unimplemented!()
    } else {
	let (moves, keys) = vault.search().unwrap();

	(moves - 1, keys)
    }
}

pub fn part_1() -> usize {
    solve_1(&DATA).0
}

pub fn part_2() -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_example_1_1() {
        assert_eq!(
            solve_1(
                r"#########
#b.A.@.a#
#########"
            )
            .0,
            8
        )
    }

    #[test]
    fn test_example_1_2() {
        assert_eq!(
            solve_1(
                r"########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"
            )
            .0,
            86
        )
    }

    #[test]
    fn test_example_1_3() {
        assert_eq!(
            solve_1(
                r"########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################"
            ),
            (132, vec!['b', 'a', 'c', 'd', 'f', 'e', 'g'])
        )
    }

    #[test]
    fn test_example_1_4() {
        assert_eq!(
            solve_1(
                r"#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################"
            ),
            (
                136,
                vec![
                    'a', 'f', 'b', 'j', 'g', 'n', 'h', 'd', 'l', 'o', 'e', 'p', 'c', 'i', 'k', 'm'
                ]
            )
        )
    }

    // #[bench]
    // fn bench_part_1(b: &mut Bencher) {
    //     b.iter(part_1);
    // }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part_2);
    }
}
