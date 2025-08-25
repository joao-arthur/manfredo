#[cfg(test)]
mod tests {
    use super::super::{saturating_inflate, saturating_inflate_assign, try_saturating_inflate, try_saturating_inflate_assign};
    use crate::cartesian::rect::rect_i16::RectI16;

    #[test]
    fn try_saturating_inflate_assign_min_bounds() {
        let mut r = RectI16::of(i16::MIN + 7, i16::MIN + 2, i16::MIN + 17, i16::MIN + 13);
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN + 6, i16::MIN + 1, i16::MIN + 18, i16::MIN + 14));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN + 5, i16::MIN, i16::MIN + 19, i16::MIN + 15));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 20, i16::MIN + 17));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN + 3, i16::MIN, i16::MIN + 21, i16::MIN + 19));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 22, i16::MIN + 21));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN + 1, i16::MIN, i16::MIN + 23, i16::MIN + 23));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 24, i16::MIN + 25));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 26, i16::MIN + 27));
    }

    #[test]
    fn try_saturating_inflate_assign_max_bounds() {
        let mut r = RectI16::of(i16::MAX - 33, i16::MAX - 17, i16::MAX - 5, i16::MAX - 3);
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 37, i16::MAX - 22, i16::MAX - 1, i16::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 38, i16::MAX - 24, i16::MAX, i16::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 40, i16::MAX - 26, i16::MAX, i16::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 42, i16::MAX - 28, i16::MAX, i16::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI16::of(i16::MAX - 44, i16::MAX - 30, i16::MAX, i16::MAX));
    }

    #[test]
    fn try_saturating_inflate_assign_to_bounds() {
        let mut r_odd = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_odd), Some(()));
        assert_eq!(r_odd, RectI16::largest());

        let mut r_even_1 = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_even_1), Some(()));
        assert_eq!(r_even_1, RectI16::largest());

        let mut r_even_2 = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX);
        assert_eq!(try_saturating_inflate_assign(&mut r_even_2), Some(()));
        assert_eq!(r_even_2, RectI16::largest());
    }

    #[test]
    fn try_saturating_inflate_assign_width_to_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 1, i16::MIN + 10, i16::MIN + 20, i16::MIN + 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectI16::of(i16::MIN, i16::MIN + 9, i16::MIN + 21, i16::MIN + 21));

        let mut r_max = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 1, i16::MIN + 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectI16::of(i16::MIN + 9, i16::MIN + 9, i16::MAX, i16::MIN + 21));
    }

    #[test]
    fn try_saturating_inflate_assign_height_to_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 1, i16::MIN + 20, i16::MIN + 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectI16::of(i16::MIN + 9, i16::MIN, i16::MIN + 21, i16::MIN + 21));

        let mut r_max = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MIN + 20, i16::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectI16::of(i16::MIN + 9, i16::MIN + 9, i16::MIN + 21, i16::MAX));
    }

    #[test]
    fn try_saturating_inflate_assign_out_of_bounds() {
        let mut r = RectI16::largest();
        assert_eq!(try_saturating_inflate_assign(&mut r), None);
        assert_eq!(r, RectI16::largest());

        let mut r_x = RectI16::of(i16::MIN, i16::MIN + 10, i16::MAX, i16::MIN + 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_x), None);
        assert_eq!(r_x, RectI16::of(i16::MIN, i16::MIN + 10, i16::MAX, i16::MIN + 20));

        let mut r_y = RectI16::of(i16::MIN + 10, i16::MIN, i16::MIN + 20, i16::MAX);
        assert_eq!(try_saturating_inflate_assign(&mut r_y), None);
        assert_eq!(r_y, RectI16::of(i16::MIN + 10, i16::MIN, i16::MIN + 20, i16::MAX));
    }

    #[test]
    fn try_saturating_inflate_min_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN + 7, i16::MIN + 2, i16::MIN + 17, i16::MIN + 13)),
            Some(RectI16::of(i16::MIN + 6, i16::MIN + 1, i16::MIN + 18, i16::MIN + 14))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN + 6, i16::MIN + 1, i16::MIN + 18, i16::MIN + 14)),
            Some(RectI16::of(i16::MIN + 5, i16::MIN, i16::MIN + 19, i16::MIN + 15))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN + 5, i16::MIN, i16::MIN + 19, i16::MIN + 15)),
            Some(RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 20, i16::MIN + 17))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 20, i16::MIN + 17)),
            Some(RectI16::of(i16::MIN + 3, i16::MIN, i16::MIN + 21, i16::MIN + 19))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN + 3, i16::MIN, i16::MIN + 21, i16::MIN + 19)),
            Some(RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 22, i16::MIN + 21))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 22, i16::MIN + 21)),
            Some(RectI16::of(i16::MIN + 1, i16::MIN, i16::MIN + 23, i16::MIN + 23))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN + 1, i16::MIN, i16::MIN + 23, i16::MIN + 23)),
            Some(RectI16::of(i16::MIN, i16::MIN, i16::MIN + 24, i16::MIN + 25))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN, i16::MIN, i16::MIN + 24, i16::MIN + 25)),
            Some(RectI16::of(i16::MIN, i16::MIN, i16::MIN + 26, i16::MIN + 27))
        );
    }

    #[test]
    fn try_saturating_inflate_max_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MAX - 33, i16::MAX - 17, i16::MAX - 5, i16::MAX - 3)),
            Some(RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2)),
            Some(RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1)),
            Some(RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX)),
            Some(RectI16::of(i16::MAX - 37, i16::MAX - 22, i16::MAX - 1, i16::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MAX - 37, i16::MAX - 22, i16::MAX - 1, i16::MAX)),
            Some(RectI16::of(i16::MAX - 38, i16::MAX - 24, i16::MAX, i16::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MAX - 38, i16::MAX - 24, i16::MAX, i16::MAX)),
            Some(RectI16::of(i16::MAX - 40, i16::MAX - 26, i16::MAX, i16::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MAX - 40, i16::MAX - 26, i16::MAX, i16::MAX)),
            Some(RectI16::of(i16::MAX - 42, i16::MAX - 28, i16::MAX, i16::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MAX - 42, i16::MAX - 28, i16::MAX, i16::MAX)),
            Some(RectI16::of(i16::MAX - 44, i16::MAX - 30, i16::MAX, i16::MAX))
        );
    }

    #[test]
    fn try_saturating_inflate_to_bounds() {
        assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1)), Some(RectI16::largest()));
        assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1)), Some(RectI16::largest()));
        assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX)), Some(RectI16::largest()));
    }

    #[test]
    fn try_saturating_inflate_width_to_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN + 1, i16::MIN + 10, i16::MIN + 20, i16::MIN + 20)),
            Some(RectI16::of(i16::MIN, i16::MIN + 9, i16::MIN + 21, i16::MIN + 21))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 1, i16::MIN + 20)),
            Some(RectI16::of(i16::MIN + 9, i16::MIN + 9, i16::MAX, i16::MIN + 21))
        );
    }

    #[test]
    fn try_saturating_inflate_height_to_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN + 10, i16::MIN + 1, i16::MIN + 20, i16::MIN + 20)),
            Some(RectI16::of(i16::MIN + 9, i16::MIN, i16::MIN + 21, i16::MIN + 21))
        );
        assert_eq!(
            try_saturating_inflate(&RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MIN + 20, i16::MAX - 1)),
            Some(RectI16::of(i16::MIN + 9, i16::MIN + 9, i16::MIN + 21, i16::MAX))
        );
    }

    #[test]
    fn try_saturating_inflate_out_of_bounds() {
        assert_eq!(try_saturating_inflate(&RectI16::largest()), None);
        assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN, i16::MIN + 10, i16::MAX, i16::MIN + 20)), None);
        assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN + 10, i16::MIN, i16::MIN + 20, i16::MAX)), None);
    }

    #[test]
    fn saturating_inflate_assign_min_bounds() {
        let mut r = RectI16::of(i16::MIN + 7, i16::MIN + 2, i16::MIN + 17, i16::MIN + 13);
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 6, i16::MIN + 1, i16::MIN + 18, i16::MIN + 14));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 5, i16::MIN, i16::MIN + 19, i16::MIN + 15));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 20, i16::MIN + 17));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 3, i16::MIN, i16::MIN + 21, i16::MIN + 19));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 22, i16::MIN + 21));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 1, i16::MIN, i16::MIN + 23, i16::MIN + 23));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 24, i16::MIN + 25));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 26, i16::MIN + 27));
    }

    #[test]
    fn saturating_inflate_assign_max_bounds() {
        let mut r = RectI16::of(i16::MAX - 33, i16::MAX - 17, i16::MAX - 5, i16::MAX - 3);
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 37, i16::MAX - 22, i16::MAX - 1, i16::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 38, i16::MAX - 24, i16::MAX, i16::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 40, i16::MAX - 26, i16::MAX, i16::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 42, i16::MAX - 28, i16::MAX, i16::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectI16::of(i16::MAX - 44, i16::MAX - 30, i16::MAX, i16::MAX));
    }

    #[test]
    fn saturating_inflate_min_bounds() {
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MIN + 7, i16::MIN + 2, i16::MIN + 17, i16::MIN + 13)),
            RectI16::of(i16::MIN + 6, i16::MIN + 1, i16::MIN + 18, i16::MIN + 14)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MIN + 6, i16::MIN + 1, i16::MIN + 18, i16::MIN + 14)),
            RectI16::of(i16::MIN + 5, i16::MIN, i16::MIN + 19, i16::MIN + 15)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MIN + 5, i16::MIN, i16::MIN + 19, i16::MIN + 15)),
            RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 20, i16::MIN + 17)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 20, i16::MIN + 17)),
            RectI16::of(i16::MIN + 3, i16::MIN, i16::MIN + 21, i16::MIN + 19)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MIN + 3, i16::MIN, i16::MIN + 21, i16::MIN + 19)),
            RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 22, i16::MIN + 21)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 22, i16::MIN + 21)),
            RectI16::of(i16::MIN + 1, i16::MIN, i16::MIN + 23, i16::MIN + 23)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MIN + 1, i16::MIN, i16::MIN + 23, i16::MIN + 23)),
            RectI16::of(i16::MIN, i16::MIN, i16::MIN + 24, i16::MIN + 25)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MIN, i16::MIN, i16::MIN + 24, i16::MIN + 25)),
            RectI16::of(i16::MIN, i16::MIN, i16::MIN + 26, i16::MIN + 27)
        );
    }

    #[test]
    fn saturating_inflate_max_bounds() {
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MAX - 33, i16::MAX - 17, i16::MAX - 5, i16::MAX - 3)),
            RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2)),
            RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1)),
            RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX)),
            RectI16::of(i16::MAX - 37, i16::MAX - 22, i16::MAX - 1, i16::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MAX - 37, i16::MAX - 22, i16::MAX - 1, i16::MAX)),
            RectI16::of(i16::MAX - 38, i16::MAX - 24, i16::MAX, i16::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MAX - 38, i16::MAX - 24, i16::MAX, i16::MAX)),
            RectI16::of(i16::MAX - 40, i16::MAX - 26, i16::MAX, i16::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MAX - 40, i16::MAX - 26, i16::MAX, i16::MAX)),
            RectI16::of(i16::MAX - 42, i16::MAX - 28, i16::MAX, i16::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectI16::of(i16::MAX - 42, i16::MAX - 28, i16::MAX, i16::MAX)),
            RectI16::of(i16::MAX - 44, i16::MAX - 30, i16::MAX, i16::MAX)
        );
    }
}
