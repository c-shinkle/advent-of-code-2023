#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use day_3::{gear_ratio::*, input::INPUT, sum_parts::*};
    use test::{black_box, Bencher};

    #[bench]
    fn sum_parts_functional_bench(b: &mut Bencher) {
        let mut sum = u32::default();

        b.iter(|| {
            sum = black_box(get_all_part_numbers_func(INPUT));
        });

        assert_eq!(sum, 530495);
    }

    #[bench]
    fn sum_parts_no_vecs_or_ndarray_or_regex_bench(b: &mut Bencher) {
        let mut sum = u32::default();

        b.iter(|| {
            sum = black_box(no_vecs_or_ndarray_or_regex::<140, 140>(INPUT));
        });

        assert_eq!(sum, 530495);
    }

    #[bench]
    fn gear_ratios_functional_bench(b: &mut Bencher) {
        let mut sum = u32::default();

        b.iter(|| {
            sum = black_box(first_impl_gear_ratios(INPUT));
        });

        assert_eq!(sum, 80253814);
    }

    #[bench]
    fn gear_ratio_hashmap_locations_no_hashset_bench(b: &mut Bencher) {
        let mut sum = u32::default();

        b.iter(|| {
            sum = black_box(hashmap_locations_no_hashset_gear_ratios(INPUT));
        });

        assert_eq!(sum, 80253814);
    }
}
