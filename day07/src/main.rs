use std::time::Instant;

use day07::{simple, spawn};

fn main() {
    let now = Instant::now();

    if true {
        println!("part 1: {}", simple::part_1());
        println!("part 2: {}", simple::part_2());
    } else {
        println!("part 1: {}", spawn::part_1());
        println!("part 2: {}", spawn::part_2());
    }

    let elapsed = now.elapsed();

    println!(
        "elapsed: {}ms ({}ns)",
        elapsed.as_millis(),
        elapsed.as_nanos()
    );
}
