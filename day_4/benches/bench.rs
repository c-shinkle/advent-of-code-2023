#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use day_4::*;
    use test::{black_box, Bencher};

    #[bench]
    fn part_1_impr_bench(b: &mut Bencher) {
        let mut sum = u32::default();

        b.iter(|| {
            sum = black_box(part_1_impr(input::INPUT));
        });

        assert_eq!(sum, 21485);
    }
}
