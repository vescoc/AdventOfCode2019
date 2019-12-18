#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use intcode;

mod path;

lazy_static! {
    static ref PROGRAM: Vec<intcode::Memory> = intcode::parse(include_str!("../data.txt"));
    static ref OUTPUT: String = {
        let mut cpu = intcode::CPU::new(PROGRAM.to_vec(), 0, None);
        let mut output: Vec<char> = vec![];
        loop {
            match cpu.run().expect("invalid cpu run state") {
                intcode::Run::Halt => break,
                intcode::Run::NeedInput => unreachable!(),
                intcode::Run::Output(value) => output.push(char::from(value as u8)),
            }
        }

        output.iter().collect()
    };
}

fn solve_1(map: &str) -> i32 {
    map.parse::<path::Path>()
        .unwrap()
        .intersections()
        .map(|(x, y)| x * y)
        .sum()
}

fn encode(s: &str) -> Result<String, ()> {
    let mut r = s
        .chars()
        .fold(vec![], |mut acc, c| {
            let mut v = match acc.pop() {
                Some((l, n)) if l == c => vec![(l, n + 1)],
                Some(v) => vec![v, (c, 1)],
                _ => vec![(c, 1)],
            };

            acc.append(&mut v);
            acc
        })
        .into_iter()
        .try_fold(String::new(), |acc, (l, n)| match (l, n) {
            ('F', n) => Ok(acc + &n.to_string() + ","),
            ('L', 1) => Ok(acc + "L,"),
            ('R', 1) => Ok(acc + "R,"),
            _ => Err(()),
        })?;

    r.pop();

    Ok(r)
}

fn find(s: &str) -> (String, String, String) {
    for i in (0..s.len()).rev() {
        let a = &s[0..i];
        if encode(a).map(|s| s.len() <= 20).unwrap_or(false) {
            let middle = s.split(a).collect::<String>();
            for i in (0..middle.len()).rev() {
                let b = &middle[0..i];
                if encode(b).map(|s| s.len() <= 20).unwrap_or(false) {
                    let last = middle.split(b).collect::<String>();
                    for i in (0..last.len()).rev() {
                        let c = &last[0..i];
                        if encode(c).map(|s| s.len() <= 20).unwrap_or(false)
                            && last.split(c).collect::<String>() == ""
                        {
                            return (a.to_string(), b.to_string(), c.to_string());
                        }
                    }
                }
            }
        }
    }

    unreachable!()
}

fn solve_2(map: &str) -> intcode::Memory {
    let moves = map
        .parse::<path::Path>()
        .unwrap()
        .moves()
        .iter()
        .map(|m| match m {
            path::Move::Forward => "F",
            path::Move::Left => "L",
            path::Move::Right => "R",
        })
        .collect::<String>();

    let (a, b, c) = find(&moves);

    let mut r = moves.replace(&a, "A,").replace(&b, "B,").replace(&c, "C,");

    r.pop();

    let tmp = r
        + "\n"
        + &encode(&a).unwrap()
        + "\n"
        + &encode(&b).unwrap()
        + "\n"
        + &encode(&c).unwrap()
        + "\nn\n";

    let mut input = tmp.chars();

    let mut output = vec![];

    let mut cpu = intcode::CPU::new(
        PROGRAM
            .iter()
            .enumerate()
            .map(|(i, v)| if i == 0 { 2 } else { *v })
            .collect::<Vec<_>>(),
        0,
        None,
    );
    loop {
        match cpu.run().expect("invalid state") {
            intcode::Run::NeedInput => {
                if let Some(c) = input.next() {
                    cpu.set_input(Some(c as u8 as intcode::Memory));
                } else {
                    panic!("EOF");
                }
            }
            intcode::Run::Output(value) => output.push(value),
            intcode::Run::Halt => break,
        }
    }

    output.last().unwrap().to_owned()
}

pub fn part_1() -> i32 {
    solve_1(&OUTPUT)
}

pub fn part_2() -> intcode::Memory {
    solve_2(&OUTPUT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    lazy_static! {
        static ref EXAMPLE: &'static str = r"..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..";
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(&EXAMPLE), 76);
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode("LFFFRFFF").unwrap(), "L,3,R,3");
    }

    #[test]
    fn test_split() {
        assert_eq!("AAA".split("AAA").collect::<String>(), "");
        assert_eq!("AAABBBAAA".split("AAA").collect::<String>(), "BBB");
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
