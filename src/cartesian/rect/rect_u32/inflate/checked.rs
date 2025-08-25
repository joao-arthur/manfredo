use crate::cartesian::{point::point_u32::PointU32, rect::rect_u32::RectU32};

pub fn try_checked_inflate_assign(r: &mut RectU32) -> Option<()> {
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

pub fn try_checked_inflate(r: &RectU32) -> Option<RectU32> {
    let min_x = r.min.x.checked_sub(1)?;
    let min_y = r.min.y.checked_sub(1)?;
    let max_x = r.max.x.checked_add(1)?;
    let max_y = r.max.y.checked_add(1)?;
    Some(RectU32 { min: PointU32 { x: min_x, y: min_y }, max: PointU32 { x: max_x, y: max_y } })
}

pub fn checked_inflate_assign(r: &mut RectU32) {
    try_checked_inflate_assign(r).unwrap()
}

pub fn checked_inflate(r: &RectU32) -> RectU32 {
    try_checked_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{checked_inflate, checked_inflate_assign, try_checked_inflate, try_checked_inflate_assign};
    use crate::cartesian::rect::rect_u32::RectU32;

    #[test]
    fn try_checked_inflate_assign_min_bounds() {
        let mut r = RectU32::of(7, 3, 9, 13);
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::of(6, 2, 10, 14));
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::of(5, 1, 11, 15));
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::of(4, 0, 12, 16));
    }

    #[test]
    fn try_checked_inflate_assign_max_bounds() {
        let mut r = RectU32::of(u32::MAX - 33, u32::MAX - 17, u32::MAX - 5, u32::MAX - 3);
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2));
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1));
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::of(u32::MAX - 36, u32::MAX - 20, u32::MAX - 2, u32::MAX));
    }

    #[test]
    fn try_checked_inflate_assign_to_bounds() {
        let mut r = RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1);
        assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::largest());
    }

    #[test]
    fn try_checked_inflate_assign_width_to_bounds() {
        let mut r_min = RectU32::of(1, 10, 20, 20);
        assert_eq!(try_checked_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectU32::of(0, 9, 21, 21));

        let mut r_max = RectU32::of(10, 10, u32::MAX - 1, 20);
        assert_eq!(try_checked_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectU32::of(9, 9, u32::MAX, 21));
    }

    #[test]
    fn try_checked_inflate_assign_height_to_bounds() {
        let mut r_min = RectU32::of(10, 1, 20, 20);
        assert_eq!(try_checked_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectU32::of(9, 0, 21, 21));

        let mut r_max = RectU32::of(10, 10, 20, u32::MAX - 1);
        assert_eq!(try_checked_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectU32::of(9, 9, 21, u32::MAX));
    }

    #[test]
    fn try_checked_inflate_assign_out_of_bounds() {
        let mut r_min_x = RectU32::of(0, 10, u32::MAX, 20);
        assert_eq!(try_checked_inflate_assign(&mut r_min_x), None);
        assert_eq!(r_min_x, RectU32::of(0, 10, u32::MAX, 20));

        let mut r_max_x = RectU32::of(10, 10, u32::MAX, 20);
        assert_eq!(try_checked_inflate_assign(&mut r_max_x), None);
        assert_eq!(r_max_x, RectU32::of(10, 10, u32::MAX, 20));

        let mut r_min_y = RectU32::of(10, 0, 20, 20);
        assert_eq!(try_checked_inflate_assign(&mut r_min_y), None);
        assert_eq!(r_min_y, RectU32::of(10, 0, 20, 20));

        let mut r_max_y = RectU32::of(10, 10, 20, u32::MAX);
        assert_eq!(try_checked_inflate_assign(&mut r_max_y), None);
        assert_eq!(r_max_y, RectU32::of(10, 10, 20, u32::MAX));

        let mut r_min = RectU32::of(0, 0, 10, 10);
        assert_eq!(try_checked_inflate_assign(&mut r_min), None);
        assert_eq!(r_min, RectU32::of(0, 0, 10, 10));

        let mut r_max = RectU32::of(10, 10, u32::MAX, u32::MAX);
        assert_eq!(try_checked_inflate_assign(&mut r_max), None);
        assert_eq!(r_max, RectU32::of(10, 10, u32::MAX, u32::MAX));
    }

    #[test]
    fn try_checked_inflate_assign_limits_out_of_bounds() {
        let mut r = RectU32::largest();
        assert_eq!(try_checked_inflate_assign(&mut r), None);
        assert_eq!(r, RectU32::largest());
    }

    #[test]
    fn try_checked_inflate_min_bounds() {
        assert_eq!(try_checked_inflate(&RectU32::of(7, 3, 9, 13)), Some(RectU32::of(6, 2, 10, 14)));
        assert_eq!(try_checked_inflate(&RectU32::of(6, 2, 10, 14)), Some(RectU32::of(5, 1, 11, 15)));
        assert_eq!(try_checked_inflate(&RectU32::of(5, 1, 11, 15)), Some(RectU32::of(4, 0, 12, 16)));
    }

    #[test]
    fn try_checked_inflate_max_bounds() {
        assert_eq!(
            try_checked_inflate(&RectU32::of(u32::MAX - 33, u32::MAX - 17, u32::MAX - 5, u32::MAX - 3)),
            Some(RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2))
        );
        assert_eq!(
            try_checked_inflate(&RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2)),
            Some(RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1))
        );
        assert_eq!(
            try_checked_inflate(&RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1)),
            Some(RectU32::of(u32::MAX - 36, u32::MAX - 20, u32::MAX - 2, u32::MAX))
        );
    }

    #[test]
    fn try_checked_inflate_to_bounds() {
        assert_eq!(try_checked_inflate(&RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1)), Some(RectU32::largest()));
    }

    #[test]
    fn try_checked_inflate_width_to_bounds() {
        assert_eq!(try_checked_inflate(&RectU32::of(1, 10, 20, 20)), Some(RectU32::of(0, 9, 21, 21)));
        assert_eq!(try_checked_inflate(&RectU32::of(10, 10, u32::MAX - 1, 20)), Some(RectU32::of(9, 9, u32::MAX, 21)));
    }

    #[test]
    fn try_checked_inflate_height_to_bounds() {
        assert_eq!(try_checked_inflate(&RectU32::of(10, 1, 20, 20)), Some(RectU32::of(9, 0, 21, 21)));
        assert_eq!(try_checked_inflate(&RectU32::of(10, 10, 20, u32::MAX - 1)), Some(RectU32::of(9, 9, 21, u32::MAX)));
    }

    #[test]
    fn try_checked_inflate_out_of_bounds() {
        assert_eq!(try_checked_inflate(&RectU32::of(0, 10, u32::MAX, 20)), None);
        assert_eq!(try_checked_inflate(&RectU32::of(10, 10, u32::MAX, 20)), None);
        assert_eq!(try_checked_inflate(&RectU32::of(10, 0, 20, 20)), None);
        assert_eq!(try_checked_inflate(&RectU32::of(10, 10, 20, u32::MAX)), None);
        assert_eq!(try_checked_inflate(&RectU32::of(0, 0, 10, 10)), None);
        assert_eq!(try_checked_inflate(&RectU32::of(10, 10, u32::MAX, u32::MAX)), None);
    }

    #[test]
    fn try_checked_inflate_limits_out_of_bounds() {
        assert_eq!(try_checked_inflate(&RectU32::largest()), None);
    }

    #[test]
    fn checked_inflate_assign_min_bounds() {
        let mut r = RectU32::of(7, 3, 9, 13);
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectU32::of(6, 2, 10, 14));
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectU32::of(5, 1, 11, 15));
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectU32::of(4, 0, 12, 16));
    }

    #[test]
    fn checked_inflate_assign_max_bounds() {
        let mut r = RectU32::of(u32::MAX - 33, u32::MAX - 17, u32::MAX - 5, u32::MAX - 3);
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2));
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1));
        checked_inflate_assign(&mut r);
        assert_eq!(r, RectU32::of(u32::MAX - 36, u32::MAX - 20, u32::MAX - 2, u32::MAX));
    }

    #[test]
    fn checked_inflate_min_bounds() {
        assert_eq!(checked_inflate(&RectU32::of(7, 3, 9, 13)), RectU32::of(6, 2, 10, 14));
        assert_eq!(checked_inflate(&RectU32::of(6, 2, 10, 14)), RectU32::of(5, 1, 11, 15));
        assert_eq!(checked_inflate(&RectU32::of(5, 1, 11, 15)), RectU32::of(4, 0, 12, 16));
    }

    #[test]
    fn checked_inflate_max_bounds() {
        assert_eq!(
            checked_inflate(&RectU32::of(u32::MAX - 33, u32::MAX - 17, u32::MAX - 5, u32::MAX - 3)),
            RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2)
        );
        assert_eq!(
            checked_inflate(&RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2)),
            RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1)
        );
        assert_eq!(
            checked_inflate(&RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1)),
            RectU32::of(u32::MAX - 36, u32::MAX - 20, u32::MAX - 2, u32::MAX)
        );
    }
}
