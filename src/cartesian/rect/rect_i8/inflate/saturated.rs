#[cfg(test)]
mod tests {
    use super::super::{saturating_inflate, saturating_inflate_assign, try_saturating_inflate, try_saturating_inflate_assign};
    use crate::cartesian::rect::rect_i8::RectI8;

    #[test]
    fn try_saturating_inflate_assign_min_bounds() {
        let mut r = RectI8::of(i8::MIN + 7, i8::MIN + 2, i8::MIN + 17, i8::MIN + 13);
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN + 6, i8::MIN + 1, i8::MIN + 18, i8::MIN + 14));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN + 5, i8::MIN, i8::MIN + 19, i8::MIN + 15));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 20, i8::MIN + 17));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN + 3, i8::MIN, i8::MIN + 21, i8::MIN + 19));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN + 2, i8::MIN, i8::MIN + 22, i8::MIN + 21));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN + 1, i8::MIN, i8::MIN + 23, i8::MIN + 23));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 24, i8::MIN + 25));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 26, i8::MIN + 27));
    }

    #[test]
    fn try_saturating_inflate_assign_max_bounds() {
        let mut r = RectI8::of(i8::MAX - 33, i8::MAX - 17, i8::MAX - 5, i8::MAX - 3);
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 37, i8::MAX - 22, i8::MAX - 1, i8::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 38, i8::MAX - 24, i8::MAX, i8::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 40, i8::MAX - 26, i8::MAX, i8::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 42, i8::MAX - 28, i8::MAX, i8::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 44, i8::MAX - 30, i8::MAX, i8::MAX));
    }

    #[test]
    fn try_saturating_inflate_assign_to_bounds() {
        let mut r_odd = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_odd), Some(()));
        assert_eq!(r_odd, RectI8::largest());

        let mut r_even_1 = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_even_1), Some(()));
        assert_eq!(r_even_1, RectI8::largest());

        let mut r_even_2 = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX);
        assert_eq!(try_saturating_inflate_assign(&mut r_even_2), Some(()));
        assert_eq!(r_even_2, RectI8::largest());
    }

    #[test]
    fn try_saturating_inflate_assign_width_to_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 1, i8::MIN + 10, i8::MIN + 20, i8::MIN + 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectI8::of(i8::MIN, i8::MIN + 9, i8::MIN + 21, i8::MIN + 21));

        let mut r_max = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 1, i8::MIN + 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectI8::of(i8::MIN + 9, i8::MIN + 9, i8::MAX, i8::MIN + 21));
    }

    #[test]
    fn try_saturating_inflate_assign_height_to_bounds() {
        let mut r_min = RectI8::of(i8::MIN + 10, i8::MIN + 1, i8::MIN + 20, i8::MIN + 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectI8::of(i8::MIN + 9, i8::MIN, i8::MIN + 21, i8::MIN + 21));

        let mut r_max = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MIN + 20, i8::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectI8::of(i8::MIN + 9, i8::MIN + 9, i8::MIN + 21, i8::MAX));
    }

    #[test]
    fn try_saturating_inflate_assign_out_of_bounds() {
        let mut r = RectI8::largest();
        assert_eq!(try_saturating_inflate_assign(&mut r), None);
        assert_eq!(r, RectI8::largest());

        let mut r_x = RectI8::of(i8::MIN, i8::MIN + 10, i8::MAX, i8::MIN + 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_x), None);
        assert_eq!(r_x, RectI8::of(i8::MIN, i8::MIN + 10, i8::MAX, i8::MIN + 20));

        let mut r_y = RectI8::of(i8::MIN + 10, i8::MIN, i8::MIN + 20, i8::MAX);
        assert_eq!(try_saturating_inflate_assign(&mut r_y), None);
        assert_eq!(r_y, RectI8::of(i8::MIN + 10, i8::MIN, i8::MIN + 20, i8::MAX));
    }

    #[test]
    fn try_saturating_inflate_min_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectI8::of(i8::MIN + 7, i8::MIN + 2, i8::MIN + 17, i8::MIN + 13)),
            Some(RectI8::of(i8::MIN + 6, i8::MIN + 1, i8::MIN + 18, i8::MIN + 14))
        );
        assert_eq!(
            try_saturating_inflate(&RectI8::of(i8::MIN + 6, i8::MIN + 1, i8::MIN + 18, i8::MIN + 14)),
            Some(RectI8::of(i8::MIN + 5, i8::MIN, i8::MIN + 19, i8::MIN + 15))
        );
        assert_eq!(
            try_saturating_inflate(&RectI8::of(i8::MIN + 5, i8::MIN, i8::MIN + 19, i8::MIN + 15)),
            Some(RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 20, i8::MIN + 17))
        );
        assert_eq!(
            try_saturating_inflate(&RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 20, i8::MIN + 17)),
            Some(RectI8::of(i8::MIN + 3, i8::MIN, i8::MIN + 21, i8::MIN + 19))
        );
        assert_eq!(
            try_saturating_inflate(&RectI8::of(i8::MIN + 3, i8::MIN, i8::MIN + 21, i8::MIN + 19)),
            Some(RectI8::of(i8::MIN + 2, i8::MIN, i8::MIN + 22, i8::MIN + 21))
        );
        assert_eq!(
            try_saturating_inflate(&RectI8::of(i8::MIN + 2, i8::MIN, i8::MIN + 22, i8::MIN + 21)),
            Some(RectI8::of(i8::MIN + 1, i8::MIN, i8::MIN + 23, i8::MIN + 23))
        );
        assert_eq!(
            try_saturating_inflate(&RectI8::of(i8::MIN + 1, i8::MIN, i8::MIN + 23, i8::MIN + 23)),
            Some(RectI8::of(i8::MIN, i8::MIN, i8::MIN + 24, i8::MIN + 25))
        );
        assert_eq!(
            try_saturating_inflate(&RectI8::of(i8::MIN, i8::MIN, i8::MIN + 24, i8::MIN + 25)),
            Some(RectI8::of(i8::MIN, i8::MIN, i8::MIN + 26, i8::MIN + 27))
        );
    }

    #[test]
    fn try_saturating_inflate_max_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectI8::of(i8::MAX - 33, i8::MAX - 17, i8::MAX - 5, i8::MAX - 3)),
            Some(RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2))
        );
        assert_eq!(
            try_saturating_inflate(&RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2)),
            Some(RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1))
        );
        assert_eq!(
            try_saturating_inflate(&RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1)),
            Some(RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX)),
            Some(RectI8::of(i8::MAX - 37, i8::MAX - 22, i8::MAX - 1, i8::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI8::of(i8::MAX - 37, i8::MAX - 22, i8::MAX - 1, i8::MAX)),
            Some(RectI8::of(i8::MAX - 38, i8::MAX - 24, i8::MAX, i8::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI8::of(i8::MAX - 38, i8::MAX - 24, i8::MAX, i8::MAX)),
            Some(RectI8::of(i8::MAX - 40, i8::MAX - 26, i8::MAX, i8::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI8::of(i8::MAX - 40, i8::MAX - 26, i8::MAX, i8::MAX)),
            Some(RectI8::of(i8::MAX - 42, i8::MAX - 28, i8::MAX, i8::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI8::of(i8::MAX - 42, i8::MAX - 28, i8::MAX, i8::MAX)),
            Some(RectI8::of(i8::MAX - 44, i8::MAX - 30, i8::MAX, i8::MAX))
        );
    }

    #[test]
    fn try_saturating_inflate_to_bounds() {
        assert_eq!(try_saturating_inflate(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1)), Some(RectI8::largest()));
        assert_eq!(try_saturating_inflate(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1)), Some(RectI8::largest()));
        assert_eq!(try_saturating_inflate(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX)), Some(RectI8::largest()));
    }

    #[test]
    fn try_saturating_inflate_width_to_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectI8::of(i8::MIN + 1, i8::MIN + 10, i8::MIN + 20, i8::MIN + 20)),
            Some(RectI8::of(i8::MIN, i8::MIN + 9, i8::MIN + 21, i8::MIN + 21))
        );
        assert_eq!(
            try_saturating_inflate(&RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 1, i8::MIN + 20)),
            Some(RectI8::of(i8::MIN + 9, i8::MIN + 9, i8::MAX, i8::MIN + 21))
        );
    }

    #[test]
    fn try_saturating_inflate_height_to_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectI8::of(i8::MIN + 10, i8::MIN + 1, i8::MIN + 20, i8::MIN + 20)),
            Some(RectI8::of(i8::MIN + 9, i8::MIN, i8::MIN + 21, i8::MIN + 21))
        );
        assert_eq!(
            try_saturating_inflate(&RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MIN + 20, i8::MAX - 1)),
            Some(RectI8::of(i8::MIN + 9, i8::MIN + 9, i8::MIN + 21, i8::MAX))
        );
    }

    #[test]
    fn try_saturating_inflate_out_of_bounds() {
        assert_eq!(try_saturating_inflate(&RectI8::largest()), None);
        assert_eq!(try_saturating_inflate(&RectI8::of(i8::MIN, i8::MIN + 10, i8::MAX, i8::MIN + 20)), None);
        assert_eq!(try_saturating_inflate(&RectI8::of(i8::MIN + 10, i8::MIN, i8::MIN + 20, i8::MAX)), None);
    }

    #[test]
    fn saturating_inflate_assign_min_bounds() {
        let mut r = RectI8::of(i8::MIN + 7, i8::MIN + 2, i8::MIN + 17, i8::MIN + 13);
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 6, i8::MIN + 1, i8::MIN + 18, i8::MIN + 14));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 5, i8::MIN, i8::MIN + 19, i8::MIN + 15));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 20, i8::MIN + 17));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 3, i8::MIN, i8::MIN + 21, i8::MIN + 19));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 2, i8::MIN, i8::MIN + 22, i8::MIN + 21));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN + 1, i8::MIN, i8::MIN + 23, i8::MIN + 23));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 24, i8::MIN + 25));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 26, i8::MIN + 27));
    }

    #[test]
    fn saturating_inflate_assign_max_bounds() {
        let mut r = RectI8::of(i8::MAX - 33, i8::MAX - 17, i8::MAX - 5, i8::MAX - 3);
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI8::of(i8::MAX - 37, i8::MAX - 22, i8::MAX - 1, i8::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI8::of(i8::MAX - 38, i8::MAX - 24, i8::MAX, i8::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI8::of(i8::MAX - 40, i8::MAX - 26, i8::MAX, i8::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI8::of(i8::MAX - 42, i8::MAX - 28, i8::MAX, i8::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI8::of(i8::MAX - 44, i8::MAX - 30, i8::MAX, i8::MAX));
    }

    #[test]
    fn saturating_inflate_min_bounds() {
        assert_eq!(
            saturating_inflate(&RectI8::of(i8::MIN + 7, i8::MIN + 2, i8::MIN + 17, i8::MIN + 13)),
            RectI8::of(i8::MIN + 6, i8::MIN + 1, i8::MIN + 18, i8::MIN + 14)
        );
        assert_eq!(
            saturating_inflate(&RectI8::of(i8::MIN + 6, i8::MIN + 1, i8::MIN + 18, i8::MIN + 14)),
            RectI8::of(i8::MIN + 5, i8::MIN, i8::MIN + 19, i8::MIN + 15)
        );
        assert_eq!(
            saturating_inflate(&RectI8::of(i8::MIN + 5, i8::MIN, i8::MIN + 19, i8::MIN + 15)),
            RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 20, i8::MIN + 17)
        );
        assert_eq!(
            saturating_inflate(&RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 20, i8::MIN + 17)),
            RectI8::of(i8::MIN + 3, i8::MIN, i8::MIN + 21, i8::MIN + 19)
        );
        assert_eq!(
            saturating_inflate(&RectI8::of(i8::MIN + 3, i8::MIN, i8::MIN + 21, i8::MIN + 19)),
            RectI8::of(i8::MIN + 2, i8::MIN, i8::MIN + 22, i8::MIN + 21)
        );
        assert_eq!(
            saturating_inflate(&RectI8::of(i8::MIN + 2, i8::MIN, i8::MIN + 22, i8::MIN + 21)),
            RectI8::of(i8::MIN + 1, i8::MIN, i8::MIN + 23, i8::MIN + 23)
        );
        assert_eq!(
            saturating_inflate(&RectI8::of(i8::MIN + 1, i8::MIN, i8::MIN + 23, i8::MIN + 23)),
            RectI8::of(i8::MIN, i8::MIN, i8::MIN + 24, i8::MIN + 25)
        );
        assert_eq!(
            saturating_inflate(&RectI8::of(i8::MIN, i8::MIN, i8::MIN + 24, i8::MIN + 25)),
            RectI8::of(i8::MIN, i8::MIN, i8::MIN + 26, i8::MIN + 27)
        );
    }

    #[test]
    fn saturating_inflate_max_bounds() {
        assert_eq!(
            saturating_inflate(&RectI8::of(i8::MAX - 33, i8::MAX - 17, i8::MAX - 5, i8::MAX - 3)),
            RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2)
        );
        assert_eq!(
            saturating_inflate(&RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2)),
            RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1)
        );
        assert_eq!(
            saturating_inflate(&RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1)),
            RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX)),
            RectI8::of(i8::MAX - 37, i8::MAX - 22, i8::MAX - 1, i8::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI8::of(i8::MAX - 37, i8::MAX - 22, i8::MAX - 1, i8::MAX)),
            RectI8::of(i8::MAX - 38, i8::MAX - 24, i8::MAX, i8::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI8::of(i8::MAX - 38, i8::MAX - 24, i8::MAX, i8::MAX)),
            RectI8::of(i8::MAX - 40, i8::MAX - 26, i8::MAX, i8::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI8::of(i8::MAX - 40, i8::MAX - 26, i8::MAX, i8::MAX)),
            RectI8::of(i8::MAX - 42, i8::MAX - 28, i8::MAX, i8::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI8::of(i8::MAX - 42, i8::MAX - 28, i8::MAX, i8::MAX)),
            RectI8::of(i8::MAX - 44, i8::MAX - 30, i8::MAX, i8::MAX)
        );
    }
}
