use crate::cartesian::{point::point_u64::PointU64, rect::rect_u64::RectU64};

pub fn try_assign_inflate(r: &mut RectU64) -> Option<()> {
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

pub fn try_inflate(r: &RectU64) -> Option<RectU64> {
    let min_x = r.min.x.checked_sub(1)?;
    let min_y = r.min.y.checked_sub(1)?;
    let max_x = r.max.x.checked_add(1)?;
    let max_y = r.max.y.checked_add(1)?;
    Some(RectU64 { min: PointU64 { x: min_x, y: min_y }, max: PointU64 { x: max_x, y: max_y } })
}

pub fn assign_inflate(r: &mut RectU64) {
    try_assign_inflate(r).unwrap()
}

pub fn inflate(r: &RectU64) -> RectU64 {
    try_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{assign_inflate, inflate, try_assign_inflate, try_inflate};
    use crate::cartesian::rect::rect_u64::RectU64;

    #[test]
    fn try_assign_inflate_min_bounds() {
        let mut r = RectU64::of(7, 3, 9, 13);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::of(6, 2, 10, 14));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::of(5, 1, 11, 15));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::of(4, 0, 12, 16));
    }

    #[test]
    fn try_assign_inflate_max_bounds() {
        let mut r = RectU64::of(u64::MAX - 33, u64::MAX - 17, u64::MAX - 5, u64::MAX - 3);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::of(u64::MAX - 34, u64::MAX - 18, u64::MAX - 4, u64::MAX - 2));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::of(u64::MAX - 35, u64::MAX - 19, u64::MAX - 3, u64::MAX - 1));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::of(u64::MAX - 36, u64::MAX - 20, u64::MAX - 2, u64::MAX));
    }

    #[test]
    fn try_assign_inflate_to_bounds() {
        let mut r = RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU64::largest());
    }

    #[test]
    fn try_assign_inflate_width_to_bounds() {
        let mut r_min = RectU64::of(1, 10, 20, 20);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectU64::of(0, 9, 21, 21));

        let mut r_max = RectU64::of(10, 10, u64::MAX - 1, 20);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectU64::of(9, 9, u64::MAX, 21));
    }

    #[test]
    fn try_assign_inflate_height_to_bounds() {
        let mut r_min = RectU64::of(10, 1, 20, 20);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectU64::of(9, 0, 21, 21));

        let mut r_max = RectU64::of(10, 10, 20, u64::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectU64::of(9, 9, 21, u64::MAX));
    }

    #[test]
    fn try_assign_inflate_out_of_bounds() {
        let mut r_min_x = RectU64::of(0, 10, u64::MAX, 20);
        assert_eq!(try_assign_inflate(&mut r_min_x), None);
        assert_eq!(r_min_x, RectU64::of(0, 10, u64::MAX, 20));

        let mut r_max_x = RectU64::of(10, 10, u64::MAX, 20);
        assert_eq!(try_assign_inflate(&mut r_max_x), None);
        assert_eq!(r_max_x, RectU64::of(10, 10, u64::MAX, 20));

        let mut r_min_y = RectU64::of(10, 0, 20, 20);
        assert_eq!(try_assign_inflate(&mut r_min_y), None);
        assert_eq!(r_min_y, RectU64::of(10, 0, 20, 20));

        let mut r_max_y = RectU64::of(10, 10, 20, u64::MAX);
        assert_eq!(try_assign_inflate(&mut r_max_y), None);
        assert_eq!(r_max_y, RectU64::of(10, 10, 20, u64::MAX));

        let mut r_min = RectU64::of(0, 0, 10, 10);
        assert_eq!(try_assign_inflate(&mut r_min), None);
        assert_eq!(r_min, RectU64::of(0, 0, 10, 10));

        let mut r_max = RectU64::of(10, 10, u64::MAX, u64::MAX);
        assert_eq!(try_assign_inflate(&mut r_max), None);
        assert_eq!(r_max, RectU64::of(10, 10, u64::MAX, u64::MAX));
    }

    #[test]
    fn try_assign_inflate_limits_out_of_bounds() {
        let mut r = RectU64::largest();
        assert_eq!(try_assign_inflate(&mut r), None);
        assert_eq!(r, RectU64::largest());
    }

    #[test]
    fn try_inflate_min_bounds() {
        assert_eq!(try_inflate(&RectU64::of(7, 3, 9, 13)), Some(RectU64::of(6, 2, 10, 14)));
        assert_eq!(try_inflate(&RectU64::of(6, 2, 10, 14)), Some(RectU64::of(5, 1, 11, 15)));
        assert_eq!(try_inflate(&RectU64::of(5, 1, 11, 15)), Some(RectU64::of(4, 0, 12, 16)));
    }

    #[test]
    fn try_inflate_max_bounds() {
        assert_eq!(
            try_inflate(&RectU64::of(u64::MAX - 33, u64::MAX - 17, u64::MAX - 5, u64::MAX - 3)),
            Some(RectU64::of(u64::MAX - 34, u64::MAX - 18, u64::MAX - 4, u64::MAX - 2))
        );
        assert_eq!(
            try_inflate(&RectU64::of(u64::MAX - 34, u64::MAX - 18, u64::MAX - 4, u64::MAX - 2)),
            Some(RectU64::of(u64::MAX - 35, u64::MAX - 19, u64::MAX - 3, u64::MAX - 1))
        );
        assert_eq!(
            try_inflate(&RectU64::of(u64::MAX - 35, u64::MAX - 19, u64::MAX - 3, u64::MAX - 1)),
            Some(RectU64::of(u64::MAX - 36, u64::MAX - 20, u64::MAX - 2, u64::MAX))
        );
    }

    #[test]
    fn try_inflate_to_bounds() {
        assert_eq!(try_inflate(&RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 1)), Some(RectU64::largest()));
    }

    #[test]
    fn try_inflate_width_to_bounds() {
        assert_eq!(try_inflate(&RectU64::of(1, 10, 20, 20)), Some(RectU64::of(0, 9, 21, 21)));
        assert_eq!(try_inflate(&RectU64::of(10, 10, u64::MAX - 1, 20)), Some(RectU64::of(9, 9, u64::MAX, 21)));
    }

    #[test]
    fn try_inflate_height_to_bounds() {
        assert_eq!(try_inflate(&RectU64::of(10, 1, 20, 20)), Some(RectU64::of(9, 0, 21, 21)));
        assert_eq!(try_inflate(&RectU64::of(10, 10, 20, u64::MAX - 1)), Some(RectU64::of(9, 9, 21, u64::MAX)));
    }

    #[test]
    fn try_inflate_out_of_bounds() {
        assert_eq!(try_inflate(&RectU64::of(0, 10, u64::MAX, 20)), None);
        assert_eq!(try_inflate(&RectU64::of(10, 10, u64::MAX, 20)), None);
        assert_eq!(try_inflate(&RectU64::of(10, 0, 20, 20)), None);
        assert_eq!(try_inflate(&RectU64::of(10, 10, 20, u64::MAX)), None);
        assert_eq!(try_inflate(&RectU64::of(0, 0, 10, 10)), None);
        assert_eq!(try_inflate(&RectU64::of(10, 10, u64::MAX, u64::MAX)), None);
    }

    #[test]
    fn try_inflate_limits_out_of_bounds() {
        assert_eq!(try_inflate(&RectU64::largest()), None);
    }

    #[test]
    fn assign_inflate_min_bounds() {
        let mut r = RectU64::of(7, 3, 9, 13);
        assign_inflate(&mut r);
        assert_eq!(r, RectU64::of(6, 2, 10, 14));
        assign_inflate(&mut r);
        assert_eq!(r, RectU64::of(5, 1, 11, 15));
        assign_inflate(&mut r);
        assert_eq!(r, RectU64::of(4, 0, 12, 16));
    }

    #[test]
    fn assign_inflate_max_bounds() {
        let mut r = RectU64::of(u64::MAX - 33, u64::MAX - 17, u64::MAX - 5, u64::MAX - 3);
        assign_inflate(&mut r);
        assert_eq!(r, RectU64::of(u64::MAX - 34, u64::MAX - 18, u64::MAX - 4, u64::MAX - 2));
        assign_inflate(&mut r);
        assert_eq!(r, RectU64::of(u64::MAX - 35, u64::MAX - 19, u64::MAX - 3, u64::MAX - 1));
        assign_inflate(&mut r);
        assert_eq!(r, RectU64::of(u64::MAX - 36, u64::MAX - 20, u64::MAX - 2, u64::MAX));
    }

    #[test]
    fn inflate_min_bounds() {
        assert_eq!(inflate(&RectU64::of(7, 3, 9, 13)), RectU64::of(6, 2, 10, 14));
        assert_eq!(inflate(&RectU64::of(6, 2, 10, 14)), RectU64::of(5, 1, 11, 15));
        assert_eq!(inflate(&RectU64::of(5, 1, 11, 15)), RectU64::of(4, 0, 12, 16));
    }

    #[test]
    fn inflate_max_bounds() {
        assert_eq!(
            inflate(&RectU64::of(u64::MAX - 33, u64::MAX - 17, u64::MAX - 5, u64::MAX - 3)),
            RectU64::of(u64::MAX - 34, u64::MAX - 18, u64::MAX - 4, u64::MAX - 2)
        );
        assert_eq!(
            inflate(&RectU64::of(u64::MAX - 34, u64::MAX - 18, u64::MAX - 4, u64::MAX - 2)),
            RectU64::of(u64::MAX - 35, u64::MAX - 19, u64::MAX - 3, u64::MAX - 1)
        );
        assert_eq!(
            inflate(&RectU64::of(u64::MAX - 35, u64::MAX - 19, u64::MAX - 3, u64::MAX - 1)),
            RectU64::of(u64::MAX - 36, u64::MAX - 20, u64::MAX - 2, u64::MAX)
        );
    }
}
