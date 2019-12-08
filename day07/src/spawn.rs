use itertools::Itertools;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

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
                let channel_a = mpsc::channel();
                let channel_b = mpsc::channel();
                let channel_c = mpsc::channel();
                let channel_d = mpsc::channel();
                let channel_e = mpsc::channel();
                let channel_output = mpsc::channel();

                let cpu_a = cpus[&p[0]]
                    .copy_with_input(None)
                    .spawn(channel_a.1, channel_b.0);
                let cpu_b = cpus[&p[1]]
                    .copy_with_input(None)
                    .spawn(channel_b.1, channel_c.0);
                let cpu_c = cpus[&p[2]]
                    .copy_with_input(None)
                    .spawn(channel_c.1, channel_d.0);
                let cpu_d = cpus[&p[3]]
                    .copy_with_input(None)
                    .spawn(channel_d.1, channel_e.0);
                let cpu_e = cpus[&p[4]]
                    .copy_with_input(None)
                    .spawn(channel_e.1, channel_output.0);

                channel_a.0.send(0).expect("send error");

                let candidate = channel_output.1.recv().expect("recv error");

                cpu_a
                    .join()
                    .expect("cpu_a panic")
                    .expect("invalid cpu_a result");
                cpu_b
                    .join()
                    .expect("cpu_b panic")
                    .expect("invalid cpu_b result");
                cpu_c
                    .join()
                    .expect("cpu_c panic")
                    .expect("invalid cpu_c result");
                cpu_d
                    .join()
                    .expect("cpu_d panic")
                    .expect("invalid cpu_d result");
                cpu_e
                    .join()
                    .expect("cpu_e panic")
                    .expect("invalid cpu_e result");

                if false {
                    Err(()) // sluggish...
                } else if candidate > current_max {
                    Ok((candidate, p))
                } else {
                    Ok((current_max, current_permutation))
                }
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
                let (channel_input_tx, channel_input_rx) = mpsc::channel();
                let (channel_output_tx, channel_output_rx) = mpsc::channel();
                let (channel_result_tx, channel_result_rx) = mpsc::channel();

                let (channel_a_tx, channel_a_rx) = mpsc::channel();
                let channel_b = mpsc::channel();
                let channel_c = mpsc::channel();
                let channel_d = mpsc::channel();
                let channel_e = mpsc::channel();

                let cpu_a = cpus[&p[0]]
                    .copy_with_input(None)
                    .spawn(channel_a_rx, channel_b.0);
                let cpu_b = cpus[&p[1]]
                    .copy_with_input(None)
                    .spawn(channel_b.1, channel_c.0);
                let cpu_c = cpus[&p[2]]
                    .copy_with_input(None)
                    .spawn(channel_c.1, channel_d.0);
                let cpu_d = cpus[&p[3]]
                    .copy_with_input(None)
                    .spawn(channel_d.1, channel_e.0);
                let cpu_e = cpus[&p[4]]
                    .copy_with_input(None)
                    .spawn(channel_e.1, channel_output_tx);

                let loop_controller = thread::spawn(move || {
                    let input = channel_input_rx.recv().expect("recv error");

                    channel_a_tx.send(input).expect("send error");

                    let mut candidate = 0;
                    for value in channel_output_rx {
                        candidate = value;

                        let _r = channel_a_tx.send(value);
                    }

                    channel_result_tx.send(candidate).expect("send error");
                });

                channel_input_tx.send(0).expect("send error");

                let candidate = channel_result_rx.recv().expect("recv error");

                cpu_a
                    .join()
                    .expect("cpu_a panic")
                    .expect("invalid cpu_a result");
                cpu_b
                    .join()
                    .expect("cpu_b panic")
                    .expect("invalid cpu_b result");
                cpu_c
                    .join()
                    .expect("cpu_c panic")
                    .expect("invalid cpu_c result");
                cpu_d
                    .join()
                    .expect("cpu_d panic")
                    .expect("invalid cpu_d result");
                cpu_e
                    .join()
                    .expect("cpu_e panic")
                    .expect("invalid cpu_e result");

                loop_controller.join().expect("loop panic");

                if false {
                    Err(()) // sluggish...
                } else if candidate > current_max {
                    Ok((candidate, p))
                } else {
                    Ok((current_max, current_permutation))
                }
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
