#[cfg(test)]
mod tests {
    use super::super::{saturating_inflate, saturating_inflate_assign, try_saturating_inflate, try_saturating_inflate_assign};
    use crate::cartesian::{
        point::point_f32::{MAX, MIN},
        rect::rect_f32::RectF32,
    };

    #[test]
    fn try_saturating_inflate_assign_min_bounds() {
        let mut r = RectF32::of(MIN + 7.0, MIN + 2.0, MIN + 17.0, MIN + 13.0);
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectF32::of(MIN + 6.0, MIN + 1.0, MIN + 18.0, MIN + 14.0));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectF32::of(MIN + 5.0, MIN, MIN + 19.0, MIN + 15.0));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectF32::of(MIN + 4.0, MIN, MIN + 20.0, MIN + 17.0));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectF32::of(MIN + 3.0, MIN, MIN + 21.0, MIN + 19.0));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectF32::of(MIN + 2.0, MIN, MIN + 22.0, MIN + 21.0));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectF32::of(MIN + 1.0, MIN, MIN + 23.0, MIN + 23.0));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectF32::of(MIN, MIN, MIN + 24.0, MIN + 25.0));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectF32::of(MIN, MIN, MIN + 26.0, MIN + 27.0));
    }

    #[test]
    fn try_saturating_inflate_assign_max_bounds() {
        let mut r = RectF32::of(MAX - 33.0, MAX - 17.0, MAX - 5.0, MAX - 3.0);
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectF32::of(MAX - 34.0, MAX - 18.0, MAX - 4.0, MAX - 2.0));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectF32::of(MAX - 35.0, MAX - 19.0, MAX - 3.0, MAX - 1.0));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectF32::of(MAX - 36.0, MAX - 20.0, MAX - 2.0, MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectF32::of(MAX - 37.0, MAX - 22.0, MAX - 1.0, MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectF32::of(MAX - 38.0, MAX - 24.0, MAX, MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectF32::of(MAX - 40.0, MAX - 26.0, MAX, MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectF32::of(MAX - 42.0, MAX - 28.0, MAX, MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectF32::of(MAX - 44.0, MAX - 30.0, MAX, MAX));
    }

    #[test]
    fn try_saturating_inflate_assign_to_bounds() {
        let mut r_odd = RectF32::of(MIN + 1.0, MIN + 1.0, MAX - 1.0, MAX - 1.0);
        assert_eq!(try_saturating_inflate_assign(&mut r_odd), Some(()));
        assert_eq!(r_odd, RectF32::largest());

        let mut r_even_1 = RectF32::of(MIN, MIN, MAX - 1.0, MAX - 1.0);
        assert_eq!(try_saturating_inflate_assign(&mut r_even_1), Some(()));
        assert_eq!(r_even_1, RectF32::largest());

        let mut r_even_2 = RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX);
        assert_eq!(try_saturating_inflate_assign(&mut r_even_2), Some(()));
        assert_eq!(r_even_2, RectF32::largest());
    }

    #[test]
    fn try_saturating_inflate_assign_width_to_bounds() {
        let mut r_min = RectF32::of(MIN + 1.0, MIN + 10.0, MIN + 20.0, MIN + 20.0);
        assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectF32::of(MIN, MIN + 9.0, MIN + 21.0, MIN + 21.0));

        let mut r_max = RectF32::of(MIN + 10.0, MIN + 10.0, MAX - 1.0, MIN + 20.0);
        assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectF32::of(MIN + 9.0, MIN + 9.0, MAX, MIN + 21.0));
    }

    #[test]
    fn try_saturating_inflate_assign_height_to_bounds() {
        let mut r_min = RectF32::of(MIN + 10.0, MIN + 1.0, MIN + 20.0, MIN + 20.0);
        assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectF32::of(MIN + 9.0, MIN, MIN + 21.0, MIN + 21.0));

        let mut r_max = RectF32::of(MIN + 10.0, MIN + 10.0, MIN + 20.0, MAX - 1.0);
        assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectF32::of(MIN + 9.0, MIN + 9.0, MIN + 21.0, MAX));
    }

    #[test]
    fn try_saturating_inflate_assign_out_of_bounds() {
        let mut r = RectF32::largest();
        assert_eq!(try_saturating_inflate_assign(&mut r), None);
        assert_eq!(r, RectF32::largest());

        let mut r_x = RectF32::of(MIN, MIN + 10.0, MAX, MIN + 20.0);
        assert_eq!(try_saturating_inflate_assign(&mut r_x), None);
        assert_eq!(r_x, RectF32::of(MIN, MIN + 10.0, MAX, MIN + 20.0));

        let mut r_y = RectF32::of(MIN + 10.0, MIN, MIN + 20.0, MAX);
        assert_eq!(try_saturating_inflate_assign(&mut r_y), None);
        assert_eq!(r_y, RectF32::of(MIN + 10.0, MIN, MIN + 20.0, MAX));
    }

    #[test]
    fn try_saturating_inflate_min_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectF32::of(MIN + 7.0, MIN + 2.0, MIN + 17.0, MIN + 13.0)),
            Some(RectF32::of(MIN + 6.0, MIN + 1.0, MIN + 18.0, MIN + 14.0))
        );
        assert_eq!(
            try_saturating_inflate(&RectF32::of(MIN + 6.0, MIN + 1.0, MIN + 18.0, MIN + 14.0)),
            Some(RectF32::of(MIN + 5.0, MIN, MIN + 19.0, MIN + 15.0))
        );
        assert_eq!(
            try_saturating_inflate(&RectF32::of(MIN + 5.0, MIN, MIN + 19.0, MIN + 15.0)),
            Some(RectF32::of(MIN + 4.0, MIN, MIN + 20.0, MIN + 17.0))
        );
        assert_eq!(
            try_saturating_inflate(&RectF32::of(MIN + 4.0, MIN, MIN + 20.0, MIN + 17.0)),
            Some(RectF32::of(MIN + 3.0, MIN, MIN + 21.0, MIN + 19.0))
        );
        assert_eq!(
            try_saturating_inflate(&RectF32::of(MIN + 3.0, MIN, MIN + 21.0, MIN + 19.0)),
            Some(RectF32::of(MIN + 2.0, MIN, MIN + 22.0, MIN + 21.0))
        );
        assert_eq!(
            try_saturating_inflate(&RectF32::of(MIN + 2.0, MIN, MIN + 22.0, MIN + 21.0)),
            Some(RectF32::of(MIN + 1.0, MIN, MIN + 23.0, MIN + 23.0))
        );
        assert_eq!(try_saturating_inflate(&RectF32::of(MIN + 1.0, MIN, MIN + 23.0, MIN + 23.0)), Some(RectF32::of(MIN, MIN, MIN + 24.0, MIN + 25.0)));
        assert_eq!(try_saturating_inflate(&RectF32::of(MIN, MIN, MIN + 24.0, MIN + 25.0)), Some(RectF32::of(MIN, MIN, MIN + 26.0, MIN + 27.0)));
    }

    #[test]
    fn try_saturating_inflate_max_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectF32::of(MAX - 33.0, MAX - 17.0, MAX - 5.0, MAX - 3.0)),
            Some(RectF32::of(MAX - 34.0, MAX - 18.0, MAX - 4.0, MAX - 2.0))
        );
        assert_eq!(
            try_saturating_inflate(&RectF32::of(MAX - 34.0, MAX - 18.0, MAX - 4.0, MAX - 2.0)),
            Some(RectF32::of(MAX - 35.0, MAX - 19.0, MAX - 3.0, MAX - 1.0))
        );
        assert_eq!(
            try_saturating_inflate(&RectF32::of(MAX - 35.0, MAX - 19.0, MAX - 3.0, MAX - 1.0)),
            Some(RectF32::of(MAX - 36.0, MAX - 20.0, MAX - 2.0, MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectF32::of(MAX - 36.0, MAX - 20.0, MAX - 2.0, MAX)),
            Some(RectF32::of(MAX - 37.0, MAX - 22.0, MAX - 1.0, MAX))
        );
        assert_eq!(try_saturating_inflate(&RectF32::of(MAX - 37.0, MAX - 22.0, MAX - 1.0, MAX)), Some(RectF32::of(MAX - 38.0, MAX - 24.0, MAX, MAX)));
        assert_eq!(try_saturating_inflate(&RectF32::of(MAX - 38.0, MAX - 24.0, MAX, MAX)), Some(RectF32::of(MAX - 40.0, MAX - 26.0, MAX, MAX)));
        assert_eq!(try_saturating_inflate(&RectF32::of(MAX - 40.0, MAX - 26.0, MAX, MAX)), Some(RectF32::of(MAX - 42.0, MAX - 28.0, MAX, MAX)));
        assert_eq!(try_saturating_inflate(&RectF32::of(MAX - 42.0, MAX - 28.0, MAX, MAX)), Some(RectF32::of(MAX - 44.0, MAX - 30.0, MAX, MAX)));
    }

    #[test]
    fn try_saturating_inflate_to_bounds() {
        assert_eq!(try_saturating_inflate(&RectF32::of(MIN + 1.0, MIN + 1.0, MAX - 1.0, MAX - 1.0)), Some(RectF32::largest()));
        assert_eq!(try_saturating_inflate(&RectF32::of(MIN, MIN, MAX - 1.0, MAX - 1.0)), Some(RectF32::largest()));
        assert_eq!(try_saturating_inflate(&RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX)), Some(RectF32::largest()));
    }

    #[test]
    fn try_saturating_inflate_width_to_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectF32::of(MIN + 1.0, MIN + 10.0, MIN + 20.0, MIN + 20.0)),
            Some(RectF32::of(MIN, MIN + 9.0, MIN + 21.0, MIN + 21.0))
        );
        assert_eq!(
            try_saturating_inflate(&RectF32::of(MIN + 10.0, MIN + 10.0, MAX - 1.0, MIN + 20.0)),
            Some(RectF32::of(MIN + 9.0, MIN + 9.0, MAX, MIN + 21.0))
        );
    }

    #[test]
    fn try_saturating_inflate_height_to_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectF32::of(MIN + 10.0, MIN + 1.0, MIN + 20.0, MIN + 20.0)),
            Some(RectF32::of(MIN + 9.0, MIN, MIN + 21.0, MIN + 21.0))
        );
        assert_eq!(
            try_saturating_inflate(&RectF32::of(MIN + 10.0, MIN + 10.0, MIN + 20.0, MAX - 1.0)),
            Some(RectF32::of(MIN + 9.0, MIN + 9.0, MIN + 21.0, MAX))
        );
    }

    #[test]
    fn try_saturating_inflate_out_of_bounds() {
        assert_eq!(try_saturating_inflate(&RectF32::largest()), None);
        assert_eq!(try_saturating_inflate(&RectF32::of(MIN, MIN + 10.0, MAX, MIN + 20.0)), None);
        assert_eq!(try_saturating_inflate(&RectF32::of(MIN + 10.0, MIN, MIN + 20.0, MAX)), None);
    }

    #[test]
    fn saturating_inflate_assign_min_bounds() {
        let mut r = RectF32::of(MIN + 7.0, MIN + 2.0, MIN + 17.0, MIN + 13.0);
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectF32::of(MIN + 6.0, MIN + 1.0, MIN + 18.0, MIN + 14.0));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectF32::of(MIN + 5.0, MIN, MIN + 19.0, MIN + 15.0));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectF32::of(MIN + 4.0, MIN, MIN + 20.0, MIN + 17.0));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectF32::of(MIN + 3.0, MIN, MIN + 21.0, MIN + 19.0));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectF32::of(MIN + 2.0, MIN, MIN + 22.0, MIN + 21.0));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectF32::of(MIN + 1.0, MIN, MIN + 23.0, MIN + 23.0));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectF32::of(MIN, MIN, MIN + 24.0, MIN + 25.0));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectF32::of(MIN, MIN, MIN + 26.0, MIN + 27.0));
    }

    #[test]
    fn saturating_inflate_assign_max_bounds() {
        let mut r = RectF32::of(MAX - 33.0, MAX - 17.0, MAX - 5.0, MAX - 3.0);
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectF32::of(MAX - 34.0, MAX - 18.0, MAX - 4.0, MAX - 2.0));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectF32::of(MAX - 35.0, MAX - 19.0, MAX - 3.0, MAX - 1.0));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectF32::of(MAX - 36.0, MAX - 20.0, MAX - 2.0, MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectF32::of(MAX - 37.0, MAX - 22.0, MAX - 1.0, MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectF32::of(MAX - 38.0, MAX - 24.0, MAX, MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectF32::of(MAX - 40.0, MAX - 26.0, MAX, MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectF32::of(MAX - 42.0, MAX - 28.0, MAX, MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectF32::of(MAX - 44.0, MAX - 30.0, MAX, MAX));
    }

    #[test]
    fn saturating_inflate_min_bounds() {
        assert_eq!(
            saturating_inflate(&RectF32::of(MIN + 7.0, MIN + 2.0, MIN + 17.0, MIN + 13.0)),
            RectF32::of(MIN + 6.0, MIN + 1.0, MIN + 18.0, MIN + 14.0)
        );
        assert_eq!(
            saturating_inflate(&RectF32::of(MIN + 6.0, MIN + 1.0, MIN + 18.0, MIN + 14.0)),
            RectF32::of(MIN + 5.0, MIN, MIN + 19.0, MIN + 15.0)
        );
        assert_eq!(saturating_inflate(&RectF32::of(MIN + 5.0, MIN, MIN + 19.0, MIN + 15.0)), RectF32::of(MIN + 4.0, MIN, MIN + 20.0, MIN + 17.0));
        assert_eq!(saturating_inflate(&RectF32::of(MIN + 4.0, MIN, MIN + 20.0, MIN + 17.0)), RectF32::of(MIN + 3.0, MIN, MIN + 21.0, MIN + 19.0));
        assert_eq!(saturating_inflate(&RectF32::of(MIN + 3.0, MIN, MIN + 21.0, MIN + 19.0)), RectF32::of(MIN + 2.0, MIN, MIN + 22.0, MIN + 21.0));
        assert_eq!(saturating_inflate(&RectF32::of(MIN + 2.0, MIN, MIN + 22.0, MIN + 21.0)), RectF32::of(MIN + 1.0, MIN, MIN + 23.0, MIN + 23.0));
        assert_eq!(saturating_inflate(&RectF32::of(MIN + 1.0, MIN, MIN + 23.0, MIN + 23.0)), RectF32::of(MIN, MIN, MIN + 24.0, MIN + 25.0));
        assert_eq!(saturating_inflate(&RectF32::of(MIN, MIN, MIN + 24.0, MIN + 25.0)), RectF32::of(MIN, MIN, MIN + 26.0, MIN + 27.0));
    }

    #[test]
    fn saturating_inflate_max_bounds() {
        assert_eq!(
            saturating_inflate(&RectF32::of(MAX - 33.0, MAX - 17.0, MAX - 5.0, MAX - 3.0)),
            RectF32::of(MAX - 34.0, MAX - 18.0, MAX - 4.0, MAX - 2.0)
        );
        assert_eq!(
            saturating_inflate(&RectF32::of(MAX - 34.0, MAX - 18.0, MAX - 4.0, MAX - 2.0)),
            RectF32::of(MAX - 35.0, MAX - 19.0, MAX - 3.0, MAX - 1.0)
        );
        assert_eq!(
            saturating_inflate(&RectF32::of(MAX - 35.0, MAX - 19.0, MAX - 3.0, MAX - 1.0)),
            RectF32::of(MAX - 36.0, MAX - 20.0, MAX - 2.0, MAX)
        );
        assert_eq!(saturating_inflate(&RectF32::of(MAX - 36.0, MAX - 20.0, MAX - 2.0, MAX)), RectF32::of(MAX - 37.0, MAX - 22.0, MAX - 1.0, MAX));
        assert_eq!(saturating_inflate(&RectF32::of(MAX - 37.0, MAX - 22.0, MAX - 1.0, MAX)), RectF32::of(MAX - 38.0, MAX - 24.0, MAX, MAX));
        assert_eq!(saturating_inflate(&RectF32::of(MAX - 38.0, MAX - 24.0, MAX, MAX)), RectF32::of(MAX - 40.0, MAX - 26.0, MAX, MAX));
        assert_eq!(saturating_inflate(&RectF32::of(MAX - 40.0, MAX - 26.0, MAX, MAX)), RectF32::of(MAX - 42.0, MAX - 28.0, MAX, MAX));
        assert_eq!(saturating_inflate(&RectF32::of(MAX - 42.0, MAX - 28.0, MAX, MAX)), RectF32::of(MAX - 44.0, MAX - 30.0, MAX, MAX));
    }
}
