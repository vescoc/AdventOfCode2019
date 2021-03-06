#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref DATA: Vec<usize> = include_str!("../data.txt")
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
}

pub fn execute(memory: &mut [usize]) -> Result<usize, usize> {
    let mut index = 0;
    loop {
        match memory[index] {
            1 => {
                memory[memory[index + 3]] = memory[memory[index + 1]] + memory[memory[index + 2]];
                index += 4;
            }
            2 => {
                memory[memory[index + 3]] = memory[memory[index + 1]] * memory[memory[index + 2]];
                index += 4;
            }
            99 => break Ok(memory[0]),
            _ => break Err(index),
        }
    }
}

fn execute_with_input(noun: usize, verb: usize) -> Result<usize, usize> {
    let mut memory = DATA.clone();
    memory[1] = noun;
    memory[2] = verb;
    execute(&mut memory)
}

pub fn part_1() -> usize {
    execute_with_input(12, 2).ok().unwrap()
}

#[allow(clippy::unreadable_literal)]
pub fn part_2() -> usize {
    for noun in 0..=99 {
        for verb in 0..=99 {
            match execute_with_input(noun, verb) {
                Ok(19690720) => return noun * 100 + verb,
                _ => continue,
            }
        }
    }

    panic!("");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example_1_1() {
        let mut memory = vec![1, 0, 0, 0, 99];
        execute(&mut memory).unwrap();
        assert_eq!(memory, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn example_1_2() {
        let mut memory = vec![2, 3, 0, 3, 99];
        execute(&mut memory).unwrap();
        assert_eq!(memory, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn example_1_3() {
        let mut memory = vec![2, 4, 4, 5, 99, 0];
        execute(&mut memory).unwrap();
        assert_eq!(memory, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn example_1_4() {
        let mut memory = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        execute(&mut memory).unwrap();
        assert_eq!(memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
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
