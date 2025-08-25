use crate::cartesian::{point::point_i64::PointI64, rect::rect_i64::RectI64};

pub fn try_checked_inflate_assign(r: &mut RectI64) -> Option<()> {
    let min_x = r.min.x.checked_sub(1)?;
    let min_y = r.min.y.checked_sub(1)?;
    let max_x = r.max.x.checked_add(1)?;
    let max_y = r.max.y.checked_add(1)?;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

pub fn try_checked_inflate(r: &RectI64) -> Option<RectI64> {
    let min_x = r.min.x.checked_sub(1)?;
    let min_y = r.min.y.checked_sub(1)?;
    let max_x = r.max.x.checked_add(1)?;
    let max_y = r.max.y.checked_add(1)?;
    Some(RectI64 { min: PointI64 { x: min_x, y: min_y }, max: PointI64 { x: max_x, y: max_y } })
}

pub fn checked_inflate_assign(r: &mut RectI64) {
    try_checked_inflate_assign(r).unwrap()
}

pub fn checked_inflate(r: &RectI64) -> RectI64 {
    try_checked_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{checked_inflate, checked_inflate_assign, try_checked_inflate, try_checked_inflate_assign};
    use crate::cartesian::rect::rect_i64::RectI64;

    #[test]
    fn try_checked_inflate_assign_min_bounds() {
        let mut r = RectI64::of(i64::MIN + 7, i64::MIN + 3, i64::MIN + 9, i64::MIN + 13);
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN + 6, i64::MIN + 2, i64::MIN + 10, i64::MIN + 14));
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN + 5, i64::MIN + 1, i64::MIN + 11, i64::MIN + 15));
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 12, i64::MIN + 16));
    }

    #[test]
    fn try_checked_inflate_assign_max_bounds() {
        let mut r = RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3);
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2));
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1));
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX));
    }

    #[test]
    fn try_checked_inflate_assign_to_bounds() {
        let mut r = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1);
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectI64::largest());
    }

    #[test]
    fn try_checked_inflate_assign_width_to_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 1, i64::MIN + 10, i64::MIN + 20, i64::MIN + 20);
        assert_eq!(try_checked_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN + 9, i64::MIN + 21, i64::MIN + 21));

        let mut r_max = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 1, i64::MIN + 20);
        assert_eq!(try_checked_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MAX, i64::MIN + 21));
    }

    #[test]
    fn try_checked_inflate_assign_height_to_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 10, i64::MIN + 1, i64::MIN + 20, i64::MIN + 20);
        assert_eq!(try_checked_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectI64::of(i64::MIN + 9, i64::MIN, i64::MIN + 21, i64::MIN + 21));

        let mut r_max = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MIN + 20, i64::MAX - 1);
        assert_eq!(try_checked_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MIN + 21, i64::MAX));
    }

    #[test]
    fn try_checked_inflate_assign_out_of_bounds() {
        let mut r_min_x = RectI64::of(i64::MIN, i64::MIN + 10, i64::MAX, i64::MIN + 20);
        assert_eq!(try_checked_inflate_assign(&mut r_min_x), None);
        assert_eq!(r_min_x, RectI64::of(i64::MIN, i64::MIN + 10, i64::MAX, i64::MIN + 20));

        let mut r_max_x = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX, i64::MIN + 20);
        assert_eq!(try_checked_inflate_assign(&mut r_max_x), None);
        assert_eq!(r_max_x, RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX, i64::MIN + 20));

        let mut r_min_y = RectI64::of(i64::MIN + 10, i64::MIN, i64::MIN + 20, i64::MIN + 20);
        assert_eq!(try_checked_inflate_assign(&mut r_min_y), None);
        assert_eq!(r_min_y, RectI64::of(i64::MIN + 10, i64::MIN, i64::MIN + 20, i64::MIN + 20));

        let mut r_max_y = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MIN + 20, i64::MAX);
        assert_eq!(try_checked_inflate_assign(&mut r_max_y), None);
        assert_eq!(r_max_y, RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MIN + 20, i64::MAX));

        let mut r_min = RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 10);
        assert_eq!(try_checked_inflate_assign(&mut r_min), None);
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 10));

        let mut r_max = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX, i64::MAX);
        assert_eq!(try_checked_inflate_assign(&mut r_max), None);
        assert_eq!(r_max, RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX, i64::MAX));
    }

    #[test]
    fn try_checked_inflate_assign_limits_out_of_bounds() {
        let mut r = RectI64::largest();
        assert_eq!(try_checked_inflate_assign(&mut r), None);
        assert_eq!(r, RectI64::largest());
    }

    #[test]
    fn try_checked_inflate_min_bounds() {
        assert_eq!(
            try_checked_inflate(&RectI64::of(i64::MIN + 7, i64::MIN + 3, i64::MIN + 9, i64::MIN + 13)),
            Some(RectI64::of(i64::MIN + 6, i64::MIN + 2, i64::MIN + 10, i64::MIN + 14))
        );
        assert_eq!(
            try_checked_inflate(&RectI64::of(i64::MIN + 6, i64::MIN + 2, i64::MIN + 10, i64::MIN + 14)),
            Some(RectI64::of(i64::MIN + 5, i64::MIN + 1, i64::MIN + 11, i64::MIN + 15))
        );
        assert_eq!(
            try_checked_inflate(&RectI64::of(i64::MIN + 5, i64::MIN + 1, i64::MIN + 11, i64::MIN + 15)),
            Some(RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 12, i64::MIN + 16))
        );
    }

    #[test]
    fn try_checked_inflate_max_bounds() {
        assert_eq!(
            try_checked_inflate(&RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3)),
            Some(RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2))
        );
        assert_eq!(
            try_checked_inflate(&RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2)),
            Some(RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1))
        );
        assert_eq!(
            try_checked_inflate(&RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1)),
            Some(RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX))
        );
    }

    #[test]
    fn try_checked_inflate_to_bounds() {
        assert_eq!(try_checked_inflate(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1)), Some(RectI64::largest()));
    }

    #[test]
    fn try_checked_inflate_width_to_bounds() {
        assert_eq!(
            try_checked_inflate(&RectI64::of(i64::MIN + 1, i64::MIN + 10, i64::MIN + 20, i64::MIN + 20)),
            Some(RectI64::of(i64::MIN, i64::MIN + 9, i64::MIN + 21, i64::MIN + 21))
        );
        assert_eq!(
            try_checked_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 1, i64::MIN + 20)),
            Some(RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MAX, i64::MIN + 21))
        );
    }

    #[test]
    fn try_checked_inflate_height_to_bounds() {
        assert_eq!(
            try_checked_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 1, i64::MIN + 20, i64::MIN + 20)),
            Some(RectI64::of(i64::MIN + 9, i64::MIN, i64::MIN + 21, i64::MIN + 21))
        );
        assert_eq!(
            try_checked_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MIN + 20, i64::MAX - 1)),
            Some(RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MIN + 21, i64::MAX))
        );
    }

    #[test]
    fn try_checked_inflate_out_of_bounds() {
        assert_eq!(try_checked_inflate(&RectI64::of(i64::MIN, i64::MIN + 10, i64::MAX, i64::MIN + 20)), None);
        assert_eq!(try_checked_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX, i64::MIN + 20)), None);
        assert_eq!(try_checked_inflate(&RectI64::of(i64::MIN + 10, i64::MIN, i64::MIN + 20, i64::MIN + 20)), None);
        assert_eq!(try_checked_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MIN + 20, i64::MAX)), None);
        assert_eq!(try_checked_inflate(&RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 10)), None);
        assert_eq!(try_checked_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX, i64::MAX)), None);
    }

    #[test]
    fn try_checked_inflate_limits_out_of_bounds() {
        assert_eq!(try_checked_inflate(&RectI64::largest()), None);
    }

    #[test]
    fn checked_inflate_assign_min_bounds() {
        let mut r = RectI64::of(i64::MIN + 7, i64::MIN + 3, i64::MIN + 9, i64::MIN + 13);
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 6, i64::MIN + 2, i64::MIN + 10, i64::MIN + 14));
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 5, i64::MIN + 1, i64::MIN + 11, i64::MIN + 15));
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 12, i64::MIN + 16));
    }

    #[test]
    fn checked_inflate_assign_max_bounds() {
        let mut r = RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3);
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2));
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1));
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX));
    }

    #[test]
    fn checked_inflate_min_bounds() {
        assert_eq!(
            checked_inflate(&RectI64::of(i64::MIN + 7, i64::MIN + 3, i64::MIN + 9, i64::MIN + 13)),
            RectI64::of(i64::MIN + 6, i64::MIN + 2, i64::MIN + 10, i64::MIN + 14)
        );
        assert_eq!(
            checked_inflate(&RectI64::of(i64::MIN + 6, i64::MIN + 2, i64::MIN + 10, i64::MIN + 14)),
            RectI64::of(i64::MIN + 5, i64::MIN + 1, i64::MIN + 11, i64::MIN + 15)
        );
        assert_eq!(
            checked_inflate(&RectI64::of(i64::MIN + 5, i64::MIN + 1, i64::MIN + 11, i64::MIN + 15)),
            RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 12, i64::MIN + 16)
        );
    }

    #[test]
    fn checked_inflate_max_bounds() {
        assert_eq!(
            checked_inflate(&RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3)),
            RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2)
        );
        assert_eq!(
            checked_inflate(&RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2)),
            RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1)
        );
        assert_eq!(
            checked_inflate(&RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1)),
            RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX)
        );
    }
}
