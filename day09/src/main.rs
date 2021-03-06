use std::time::Instant;

use day09::simple;

fn main() {
    let now = Instant::now();

    println!("part 1: {}", simple::part_1());
    println!("part 2: {}", simple::part_2());

    let elapsed = now.elapsed();

    println!(
        "elapsed: {}ms ({}ns)",
        elapsed.as_millis(),
        elapsed.as_nanos()
    );
}
