#[cfg(test)]
mod tests {
    use super::super::{checked_inflate, checked_inflate_assign, try_checked_inflate, try_checked_inflate_assign};
    use crate::matrix::rect::rect_i8::RectI8;

    #[test]
    fn try_checked_inflate_assign_min_bounds() {
        let mut r = RectI8::of(i8::MIN + 7, i8::MIN + 3, i8::MIN + 9, i8::MIN + 13);
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN + 6, i8::MIN + 2, i8::MIN + 10, i8::MIN + 14));
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN + 5, i8::MIN + 1, i8::MIN + 11, i8::MIN + 15));
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 12, i8::MIN + 16));
    }

    #[test]
    fn try_checked_inflate_assign_max_bounds() {
        let mut r = RectI8::of(i8::MAX - 33, i8::MAX - 17, i8::MAX - 5, i8::MAX - 3);
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2));
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1));
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX));
    }

    #[test]
    fn try_checked_inflate_assign_to_bounds() {
        let mut r = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1);
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::largest());
    }

    #[test]
    fn try_checked_inflate_assign_width_to_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 1, i8::MIN + 10, i8::MIN + 20, i8::MIN + 20);
        assert_eq!(try_checked_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectI8::of(i8::MIN, i8::MIN + 9, i8::MIN + 21, i8::MIN + 21));

        let mut r_max = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 1, i8::MIN + 20);
        assert_eq!(try_checked_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectI8::of(i8::MIN + 9, i8::MIN + 9, i8::MAX, i8::MIN + 21));
    }

    #[test]
    fn try_checked_inflate_assign_height_to_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 10, i8::MIN + 1, i8::MIN + 20, i8::MIN + 20);
        assert_eq!(try_checked_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectI8::of(i8::MIN + 9, i8::MIN, i8::MIN + 21, i8::MIN + 21));

        let mut r_max = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MIN + 20, i8::MAX - 1);
        assert_eq!(try_checked_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectI8::of(i8::MIN + 9, i8::MIN + 9, i8::MIN + 21, i8::MAX));
    }

    #[test]
    fn try_checked_inflate_assign_out_of_bounds() {
        let mut r_min_row = RectI8::of(i8::MIN, i8::MIN + 10, i8::MAX, i8::MIN + 20);
        assert_eq!(try_checked_inflate_assign(&mut r_min_row), None);
        assert_eq!(r_min_row, RectI8::of(i8::MIN, i8::MIN + 10, i8::MAX, i8::MIN + 20));

        let mut r_max_row = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX, i8::MIN + 20);
        assert_eq!(try_checked_inflate_assign(&mut r_max_row), None);
        assert_eq!(r_max_row, RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX, i8::MIN + 20));

        let mut r_min_col = RectI8::of(i8::MIN + 10, i8::MIN, i8::MIN + 20, i8::MIN + 20);
        assert_eq!(try_checked_inflate_assign(&mut r_min_col), None);
        assert_eq!(r_min_col, RectI8::of(i8::MIN + 10, i8::MIN, i8::MIN + 20, i8::MIN + 20));

        let mut r_max_col = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MIN + 20, i8::MAX);
        assert_eq!(try_checked_inflate_assign(&mut r_max_col), None);
        assert_eq!(r_max_col, RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MIN + 20, i8::MAX));

        let mut r_min = RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 10);
        assert_eq!(try_checked_inflate_assign(&mut r_min), None);
        assert_eq!(r_min, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 10));

        let mut r_max = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX, i8::MAX);
        assert_eq!(try_checked_inflate_assign(&mut r_max), None);
        assert_eq!(r_max, RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX, i8::MAX));
    }

    #[test]
    fn try_checked_inflate_assign_limits_out_of_bounds() {
        let mut r = RectI8::largest();
        assert_eq!(try_checked_inflate_assign(&mut r), None);
        assert_eq!(r, RectI8::largest());
    }

    #[test]
    fn try_checked_inflate_min_bounds() {
        assert_eq!(try_checked_inflate(&RectI8::of(i8::MIN + 7, i8::MIN + 3, i8::MIN + 9, i8::MIN + 13)), Some(RectI8::of(i8::MIN + 6, i8::MIN + 2, i8::MIN + 10, i8::MIN + 14)));
        assert_eq!(try_checked_inflate(&RectI8::of(i8::MIN + 6, i8::MIN + 2, i8::MIN + 10, i8::MIN + 14)), Some(RectI8::of(i8::MIN + 5, i8::MIN + 1, i8::MIN + 11, i8::MIN + 15)));
        assert_eq!(try_checked_inflate(&RectI8::of(i8::MIN + 5, i8::MIN + 1, i8::MIN + 11, i8::MIN + 15)), Some(RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 12, i8::MIN + 16)));
    }

    #[test]
    fn try_checked_inflate_max_bounds() {
        assert_eq!(try_checked_inflate(&RectI8::of(i8::MAX - 33, i8::MAX - 17, i8::MAX - 5, i8::MAX - 3)), Some(RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2)));
        assert_eq!(try_checked_inflate(&RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2)), Some(RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1)));
        assert_eq!(try_checked_inflate(&RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1)), Some(RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX)));
    }

    #[test]
    fn try_checked_inflate_to_bounds() {
        assert_eq!(try_checked_inflate(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1)), Some(RectI8::largest()));
    }

    #[test]
    fn try_checked_inflate_width_to_bounds() {
        assert_eq!(try_checked_inflate(&RectI8::of(i8::MIN + 1, i8::MIN + 10, i8::MIN + 20, i8::MIN + 20)), Some(RectI8::of(i8::MIN, i8::MIN + 9, i8::MIN + 21, i8::MIN + 21)));
        assert_eq!(try_checked_inflate(&RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 1, i8::MIN + 20)), Some(RectI8::of(i8::MIN + 9, i8::MIN + 9, i8::MAX, i8::MIN + 21)));
    }

    #[test]
    fn try_checked_inflate_height_to_bounds() {
        assert_eq!(try_checked_inflate(&RectI8::of(i8::MIN + 10, i8::MIN + 1, i8::MIN + 20, i8::MIN + 20)), Some(RectI8::of(i8::MIN + 9, i8::MIN, i8::MIN + 21, i8::MIN + 21)));
        assert_eq!(try_checked_inflate(&RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MIN + 20, i8::MAX - 1)), Some(RectI8::of(i8::MIN + 9, i8::MIN + 9, i8::MIN + 21, i8::MAX)));
    }

    #[test]
    fn try_checked_inflate_out_of_bounds() {
        assert_eq!(try_checked_inflate(&RectI8::of(i8::MIN, i8::MIN + 10, i8::MAX, i8::MIN + 20)), None);
        assert_eq!(try_checked_inflate(&RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX, i8::MIN + 20)), None);
        assert_eq!(try_checked_inflate(&RectI8::of(i8::MIN + 10, i8::MIN, i8::MIN + 20, i8::MIN + 20)), None);
        assert_eq!(try_checked_inflate(&RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MIN + 20, i8::MAX)), None);
        assert_eq!(try_checked_inflate(&RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 10)), None);
        assert_eq!(try_checked_inflate(&RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX, i8::MAX)), None);
    }

    #[test]
    fn try_checked_inflate_limits_out_of_bounds() {
        assert_eq!(try_checked_inflate(&RectI8::largest()), None);
    }

    #[test]
    fn checked_inflate_assign_min_bounds() {
        let mut r = RectI8::of(i8::MIN + 7, i8::MIN + 3, i8::MIN + 9, i8::MIN + 13);
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 6, i8::MIN + 2, i8::MIN + 10, i8::MIN + 14));
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 5, i8::MIN + 1, i8::MIN + 11, i8::MIN + 15));
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 12, i8::MIN + 16));
    }

    #[test]
    fn checked_inflate_assign_max_bounds() {
        let mut r = RectI8::of(i8::MAX - 33, i8::MAX - 17, i8::MAX - 5, i8::MAX - 3);
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2));
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1));
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX));
    }

    #[test]
    fn checked_inflate_min_bounds() {
        assert_eq!(checked_inflate(&RectI8::of(i8::MIN + 7, i8::MIN + 3, i8::MIN + 9, i8::MIN + 13)), RectI8::of(i8::MIN + 6, i8::MIN + 2, i8::MIN + 10, i8::MIN + 14));
        assert_eq!(checked_inflate(&RectI8::of(i8::MIN + 6, i8::MIN + 2, i8::MIN + 10, i8::MIN + 14)), RectI8::of(i8::MIN + 5, i8::MIN + 1, i8::MIN + 11, i8::MIN + 15));
        assert_eq!(checked_inflate(&RectI8::of(i8::MIN + 5, i8::MIN + 1, i8::MIN + 11, i8::MIN + 15)), RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 12, i8::MIN + 16));
    }

    #[test]
    fn checked_inflate_max_bounds() {
        assert_eq!(checked_inflate(&RectI8::of(i8::MAX - 33, i8::MAX - 17, i8::MAX - 5, i8::MAX - 3)), RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2));
        assert_eq!(checked_inflate(&RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2)), RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1));
        assert_eq!(checked_inflate(&RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1)), RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX));
    }
}
