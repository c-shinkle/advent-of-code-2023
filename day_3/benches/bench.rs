#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use day_3::{get_all_part_numbers, input::INPUT};
    use test::{black_box, Bencher};

    #[bench]
    fn first_bench(b: &mut Bencher) {
        let mut sum = u32::default();

        b.iter(|| {
            sum = black_box(get_all_part_numbers(INPUT));
        });

        assert_eq!(sum, 530495);
    }
}
