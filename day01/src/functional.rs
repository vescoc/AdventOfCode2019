pub use crate::simple::calculate_fuel;

pub fn calculate_total_fuel(mass: u32) -> Option<u32> {
    (0..)
        .try_fold((mass, 0), |(mass, total), _| match calculate_fuel(mass) {
            Some(c) if c > 0 => Ok((c, total + c)),
            _ => Err(total),
        })
        .err()
}

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
mod tests {
    use super::*;
    use test::Bencher;

    use crate::{part, DATA};

    #[test]
    fn example_2() {
        assert_eq!(calculate_total_fuel(12), Some(2));
        assert_eq!(calculate_total_fuel(1969), Some(966));
        assert_eq!(calculate_total_fuel(100756), Some(50346));
    }

    #[bench]
    fn bench_example_2(b: &mut Bencher) {
        b.iter(|| calculate_total_fuel(100756))
    }

    #[bench]
    fn bench_parts(b: &mut Bencher) {
        b.iter(|| part(&DATA, calculate_fuel));
        b.iter(|| part(&DATA, calculate_total_fuel));
    }
}
