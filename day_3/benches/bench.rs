#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use day_3::input::INPUT;
    use day_3::*;
    use test::{black_box, Bencher};

    #[bench]
    fn functional_bench(b: &mut Bencher) {
        let mut sum = u32::default();

        b.iter(|| {
            sum = black_box(get_all_part_numbers_func(INPUT));
        });

        assert_eq!(sum, 530495);
    }

    #[bench]
    fn no_vecs_or_ndarray_or_regex_bench(b: &mut Bencher) {
        let mut sum = u32::default();

        b.iter(|| {
            sum = black_box(no_vecs_or_ndarray_or_regex::<140, 140>(INPUT));
        });

        assert_eq!(sum, 530495);
    }
}
