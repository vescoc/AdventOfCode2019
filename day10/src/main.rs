use std::time::Instant;

use day10::generic;

fn main() {
    let now = Instant::now();

    println!("part 1: {}", generic::part_1());
    println!("part 2: {}", generic::part_2());

    let elapsed = now.elapsed();

    println!(
        "elapsed: {}ms ({}ns)",
        elapsed.as_millis(),
        elapsed.as_nanos()
    );
}
