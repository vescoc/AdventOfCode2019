use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

use crate::intcode::{Memory, Step, CPU};
use crate::DATA;

pub fn solve_1(base_memory: &[Memory]) -> (Memory, Vec<usize>) {
    let mut base_cpu = CPU::new(base_memory.to_owned(), 0, None);
    match base_cpu.run() {
        Ok(Step::NeedInput) => {}
        s => panic!("invalid state {:?}", s),
    }

    let cpus = (0usize..5usize)
        .try_fold(HashMap::new(), |mut acc, i| {
            let mut cpu = base_cpu.copy_with_input(Some(i as Memory));

            match cpu.run() {
                Ok(Step::NeedInput) => {
                    acc.insert(i, cpu);
                    Ok(acc)
                }
                Err(e) => Err(e),
                _ => unreachable!(),
            }
        })
        .unwrap_or_else(|e| panic!("invalid data: {:?}", e));

    (0usize..5usize)
        .permutations(5)
        .try_fold(
            (std::i64::MIN, vec![]),
            |(current_max, current_permutation), p| {
                p.iter()
                    .try_fold(0, |acc, i| {
                        let mut cpu = cpus[i].copy_with_input(Some(acc));

                        match cpu.run() {
                            Ok(Step::Output(value)) => Ok(value),
                            Err(e) => Err(e),
                            _ => unreachable!(),
                        }
                    })
                    .and_then(|value| match value.cmp(&current_max) {
                        Ordering::Greater => Ok((value, p)),
                        _ => Ok((current_max, current_permutation)),
                    })
            },
        )
        .unwrap_or_else(|e| panic!("invalid state: {:?}", e))
}

pub fn solve_2(base_memory: &[Memory]) -> (Memory, Vec<usize>) {
    let mut base_cpu = CPU::new(base_memory.to_owned(), 0, None);
    match base_cpu.run() {
        Ok(Step::NeedInput) => {}
        s => panic!("invalid state {:?}", s),
    }

    let cpus = (5usize..10usize)
        .try_fold(HashMap::new(), |mut acc, i| {
            let mut cpu = base_cpu.copy_with_input(Some(i as Memory));

            match cpu.run() {
                Ok(Step::NeedInput) => {
                    acc.insert(i, cpu);
                    Ok(acc)
                }
                Err(e) => Err(e),
                _ => unreachable!(),
            }
        })
        .unwrap_or_else(|e| panic!("invalid data: {:?}", e));

    (5usize..10usize)
        .permutations(5)
        .try_fold(
            (std::i64::MIN, vec![]),
            |(current_max, current_permutation), p| {
                let mut cpus = cpus
                    .iter()
                    .map(|(k, v)| (k, v.copy_with_input(v.input())))
                    .collect::<HashMap<_, _>>();

                let p_last = p.last().unwrap();

                p.iter()
                    .cycle()
                    .try_fold((Some(0), None), |(input, output), i| {
                        let cpu = cpus.get_mut(i).unwrap();
                        cpu.set_input(input);

                        match cpu.run() {
                            Ok(Step::Output(value)) if i == p_last => {
                                Ok((Some(value), Some(value)))
                            }
                            Ok(Step::Output(value)) => Ok((Some(value), output)),

                            Ok(Step::Halt) if i == p_last => Err(Ok(output.unwrap())),
                            Ok(Step::Halt) => Ok((None, output)),
                            Err(e) => Err(Err(e)),
                            _ => unreachable!(),
                        }
                    })
                    .unwrap_err()
                    .and_then(|value| match value.cmp(&current_max) {
                        Ordering::Greater => Ok((value, p)),
                        _ => Ok((current_max, current_permutation)),
                    })
            },
        )
        .unwrap_or_else(|e| panic!("invalid state: {:?}", e))
}

pub fn part_1() -> Memory {
    solve_1(&DATA).0
}

pub fn part_2() -> Memory {
    solve_2(&DATA).0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    use crate::intcode::parse;

    #[test]
    fn test_example_1_1() {
        let base_memory = parse(r#"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"#);

        assert_eq!(solve_1(&base_memory), (43210, vec![4, 3, 2, 1, 0]));
    }

    #[test]
    fn test_example_1_2() {
        let base_memory =
            parse(r#"3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"#);

        assert_eq!(solve_1(&base_memory), (54321, vec![0, 1, 2, 3, 4]));
    }

    #[test]
    fn test_example_1_3() {
        let base_memory = parse(
            r#"3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"#,
        );

        assert_eq!(solve_1(&base_memory), (65210, vec![1, 0, 4, 3, 2]));
    }

    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_example_2_1() {
        let base_memory = parse(
            r#"3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"#,
        );

        assert_eq!(solve_2(&base_memory), (139629729, vec![9, 8, 7, 6, 5]));
    }

    #[test]
    fn test_example_2_2() {
        let base_memory = parse(
            r#"3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"#,
        );

        assert_eq!(solve_2(&base_memory), (18216, vec![9, 7, 8, 5, 6]));
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
