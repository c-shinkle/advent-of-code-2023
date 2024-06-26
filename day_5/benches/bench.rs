#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use day_5::{input, part_1, part_1::func, part_1::impr};
    use test::{black_box, Bencher};

    #[bench]
    fn part_1_impr_bench(b: &mut Bencher) {
        let mut sum = part_1::Location::default();

        b.iter(|| {
            sum = black_box(impr::part_1(input::INPUT));
        });

        assert_eq!(sum, 107430936);
    }

    #[bench]
    fn part_1_func_bench(b: &mut Bencher) {
        let mut sum = part_1::Location::default();

        b.iter(|| {
            sum = black_box(func::part_1(input::INPUT));
        });

        assert_eq!(sum, 107430936);
    }
}
