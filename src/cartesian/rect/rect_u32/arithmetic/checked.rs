use crate::cartesian::{
    point::{point_i32::PointI32, point_u32::PointU32},
    rect::rect_u32::RectU32,
};

pub fn try_assign_add(r: &mut RectU32, delta: &PointI32) -> Option<()> {
    let min_x = u32::try_from(i64::from(r.min.x) + i64::from(delta.x)).ok()?;
    let min_y = u32::try_from(i64::from(r.min.y) + i64::from(delta.y)).ok()?;
    let max_x = u32::try_from(i64::from(r.max.x) + i64::from(delta.x)).ok()?;
    let max_y = u32::try_from(i64::from(r.max.y) + i64::from(delta.y)).ok()?;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

pub fn try_add(r: &RectU32, delta: &PointI32) -> Option<RectU32> {
    let min_x = u32::try_from(i64::from(r.min.x) + i64::from(delta.x)).ok()?;
    let min_y = u32::try_from(i64::from(r.min.y) + i64::from(delta.y)).ok()?;
    let max_x = u32::try_from(i64::from(r.max.x) + i64::from(delta.x)).ok()?;
    let max_y = u32::try_from(i64::from(r.max.y) + i64::from(delta.y)).ok()?;
    Some(RectU32 { min: PointU32 { x: min_x, y: min_y }, max: PointU32 { x: max_x, y: max_y } })
}

pub fn assign_add(r: &mut RectU32, delta: &PointI32) {
    try_assign_add(r, delta).unwrap()
}

pub fn add(r: &RectU32, delta: &PointI32) -> RectU32 {
    try_add(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{point::point_i32::PointI32, rect::rect_u32::RectU32};

    use super::{add, assign_add, try_add, try_assign_add};

    #[test]
    fn test_try_assign_add() {
        let mut r = RectU32::of(0, 0, 12, 15);
        assert_eq!(try_assign_add(&mut r, &PointI32::of(5, 4)), Some(()));
        assert_eq!(r, RectU32::of(5, 4, 17, 19));
        assert_eq!(try_assign_add(&mut r, &PointI32::of(-4, -2)), Some(()));
        assert_eq!(r, RectU32::of(1, 2, 13, 17));
    }

    #[test]
    fn try_assign_add_small_rect_to_bounds() {
        let mut min_r = RectU32::of(2, 5, 12, 15);
        assert_eq!(try_assign_add(&mut min_r, &PointI32::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectU32::of(0, 0, 10, 10));

        let mut max_r = RectU32::of(u32::MAX - 12, u32::MAX - 15, u32::MAX - 2, u32::MAX - 5);
        assert_eq!(try_assign_add(&mut max_r, &PointI32::of(2, 5)), Some(()));
        assert_eq!(max_r, RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX, u32::MAX));
    }

    #[test]
    fn try_assign_add_big_rect_to_bounds() {
        let mut min_r = RectU32::of(2, 5, u32::MAX, u32::MAX);
        assert_eq!(try_assign_add(&mut min_r, &PointI32::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5));

        let mut max_r = RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5);
        assert_eq!(try_assign_add(&mut max_r, &PointI32::of(2, 5)), Some(()));
        assert_eq!(max_r, RectU32::of(2, 5, u32::MAX, u32::MAX));
    }

    #[test]
    fn try_assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectU32::of(10, 5, 20, 30);
        assert_eq!(try_assign_add(&mut r_min, &PointI32::of(-20, -20)), None);
        assert_eq!(r_min, RectU32::of(10, 5, 20, 30));

        let mut r_max = RectU32::of(u32::MAX - 20, u32::MAX - 30, u32::MAX - 5, u32::MAX - 10);
        assert_eq!(try_assign_add(&mut r_max, &PointI32::of(20, 20)), None);
        assert_eq!(r_max, RectU32::of(u32::MAX - 20, u32::MAX - 30, u32::MAX - 5, u32::MAX - 10));
    }

    #[test]
    fn try_assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectU32::of(10, 5, u32::MAX, u32::MAX);
        assert_eq!(try_assign_add(&mut r_min, &PointI32::of(-20, -20)), None);
        assert_eq!(r_min, RectU32::of(10, 5, u32::MAX, u32::MAX));

        let mut r_max = RectU32::of(0, 0, u32::MAX - 5, u32::MAX - 10);
        assert_eq!(try_assign_add(&mut r_max, &PointI32::of(20, 20)), None);
        assert_eq!(r_max, RectU32::of(0, 0, u32::MAX - 5, u32::MAX - 10));
    }

    #[test]
    fn try_assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU32::of(1, 1, 10, 10);
        assert_eq!(try_assign_add(&mut r_min, &PointI32::min()), None);
        assert_eq!(try_assign_add(&mut r_min, &PointI32::of(i32::MIN, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &PointI32::of(0, i32::MIN)), None);
        assert_eq!(r_min, RectU32::of(1, 1, 10, 10));

        let mut r_max = RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX - 1, u32::MAX - 1);
        assert_eq!(try_assign_add(&mut r_max, &PointI32::max()), None);
        assert_eq!(try_assign_add(&mut r_max, &PointI32::of(i32::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &PointI32::of(0, i32::MAX)), None);
        assert_eq!(r_max, RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX - 1, u32::MAX - 1));
    }

    #[test]
    fn try_assign_add_big_rect_limits_out_of_bounds() {
        let mut r_min = RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1);
        assert_eq!(try_assign_add(&mut r_min, &PointI32::max()), None);
        assert_eq!(try_assign_add(&mut r_min, &PointI32::of(i32::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &PointI32::of(0, i32::MAX)), None);
        assert_eq!(r_min, RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1));

        let mut r_max = RectU32::of(1, 1, u32::MAX, u32::MAX);
        assert_eq!(try_assign_add(&mut r_max, &PointI32::min()), None);
        assert_eq!(try_assign_add(&mut r_max, &PointI32::of(i32::MIN, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &PointI32::of(0, i32::MIN)), None);
        assert_eq!(r_max, RectU32::of(1, 1, u32::MAX, u32::MAX));
    }

    #[test]
    fn test_try_add() {
        assert_eq!(try_add(&RectU32::of(0, 0, 12, 15), &PointI32::of(5, 4)), Some(RectU32::of(5, 4, 17, 19)));
        assert_eq!(try_add(&RectU32::of(5, 4, 17, 19), &PointI32::of(-4, -2)), Some(RectU32::of(1, 2, 13, 17)));
    }

    #[test]
    fn try_add_small_rect_to_bounds() {
        assert_eq!(try_add(&RectU32::of(2, 5, 12, 15), &PointI32::of(-2, -5)), Some(RectU32::of(0, 0, 10, 10)));
        assert_eq!(
            try_add(&RectU32::of(u32::MAX - 12, u32::MAX - 15, u32::MAX - 2, u32::MAX - 5), &PointI32::of(2, 5)),
            Some(RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX, u32::MAX))
        );
    }

    #[test]
    fn try_add_big_rect_to_bounds() {
        assert_eq!(try_add(&RectU32::of(2, 5, u32::MAX, u32::MAX), &PointI32::of(-2, -5)), Some(RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5)));
        assert_eq!(try_add(&RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 5), &PointI32::of(2, 5)), Some(RectU32::of(2, 5, u32::MAX, u32::MAX)));
    }

    #[test]
    fn try_add_small_rect_out_of_bounds() {
        assert_eq!(try_add(&RectU32::of(10, 5, 20, 30), &PointI32::of(-20, -20)), None);
        assert_eq!(try_add(&RectU32::of(u32::MAX - 20, u32::MAX - 30, u32::MAX - 5, u32::MAX - 10), &PointI32::of(20, 20)), None);
    }

    #[test]
    fn try_add_big_rect_out_of_bounds() {
        assert_eq!(try_add(&RectU32::of(10, 5, u32::MAX, u32::MAX), &PointI32::of(-20, -20)), None);
        assert_eq!(try_add(&RectU32::of(0, 0, u32::MAX - 5, u32::MAX - 10), &PointI32::of(20, 20)), None);
    }

    #[test]
    fn try_add_small_rect_limits_out_of_bounds() {
        assert_eq!(try_add(&RectU32::of(1, 1, 10, 10), &PointI32::min()), None);
        assert_eq!(try_add(&RectU32::of(1, 1, 10, 10), &PointI32::of(i32::MIN, 0)), None);
        assert_eq!(try_add(&RectU32::of(1, 1, 10, 10), &PointI32::of(0, i32::MIN)), None);
        assert_eq!(try_add(&RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX - 1, u32::MAX - 1), &PointI32::max()), None);
        assert_eq!(try_add(&RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX - 1, u32::MAX - 1), &PointI32::of(i32::MAX, 0)), None);
        assert_eq!(try_add(&RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX - 1, u32::MAX - 1), &PointI32::of(0, i32::MAX)), None);
    }

    #[test]
    fn try_add_big_rect_limits_out_of_bounds() {
        assert_eq!(try_add(&RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1), &PointI32::max()), None);
        assert_eq!(try_add(&RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1), &PointI32::of(i32::MAX, 0)), None);
        assert_eq!(try_add(&RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1), &PointI32::of(0, i32::MAX)), None);
        assert_eq!(try_add(&RectU32::of(1, 1, u32::MAX, u32::MAX), &PointI32::min()), None);
        assert_eq!(try_add(&RectU32::of(1, 1, u32::MAX, u32::MAX), &PointI32::of(i32::MIN, 0)), None);
        assert_eq!(try_add(&RectU32::of(1, 1, u32::MAX, u32::MAX), &PointI32::of(0, i32::MIN)), None);
    }

    #[test]
    fn test_assign_add() {
        let mut r = RectU32::of(0, 0, 12, 15);
        assign_add(&mut r, &PointI32::of(5, 4));
        assert_eq!(r, RectU32::of(5, 4, 17, 19));
        assign_add(&mut r, &PointI32::of(-4, -2));
        assert_eq!(r, RectU32::of(1, 2, 13, 17));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&RectU32::of(0, 0, 12, 15), &PointI32::of(5, 4)), RectU32::of(5, 4, 17, 19));
        assert_eq!(add(&RectU32::of(5, 4, 17, 19), &PointI32::of(-4, -2)), RectU32::of(1, 2, 13, 17));
    }
}
