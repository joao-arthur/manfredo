use crate::cartesian::{
    point::{point_i16::PointI16, point_u16::PointU16},
    rect::rect_u16::RectU16,
};

pub fn try_assign_add(r: &mut RectU16, delta: &PointI16) -> Option<()> {
    let min_x = u16::try_from(i32::from(r.min.x) + i32::from(delta.x)).ok()?;
    let min_y = u16::try_from(i32::from(r.min.y) + i32::from(delta.y)).ok()?;
    let max_x = u16::try_from(i32::from(r.max.x) + i32::from(delta.x)).ok()?;
    let max_y = u16::try_from(i32::from(r.max.y) + i32::from(delta.y)).ok()?;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

pub fn try_add(r: &RectU16, delta: &PointI16) -> Option<RectU16> {
    let min_x = u16::try_from(i32::from(r.min.x) + i32::from(delta.x)).ok()?;
    let min_y = u16::try_from(i32::from(r.min.y) + i32::from(delta.y)).ok()?;
    let max_x = u16::try_from(i32::from(r.max.x) + i32::from(delta.x)).ok()?;
    let max_y = u16::try_from(i32::from(r.max.y) + i32::from(delta.y)).ok()?;
    Some(RectU16 { min: PointU16 { x: min_x, y: min_y }, max: PointU16 { x: max_x, y: max_y } })
}

pub fn assign_add(r: &mut RectU16, delta: &PointI16) {
    try_assign_add(r, delta).unwrap()
}

pub fn add(r: &RectU16, delta: &PointI16) -> RectU16 {
    try_add(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{point::point_i16::PointI16, rect::rect_u16::RectU16};

    use super::{add, assign_add, try_add, try_assign_add};

    #[test]
    fn test_try_assign_add() {
        let mut r = RectU16::of(0, 0, 12, 15);
        assert_eq!(try_assign_add(&mut r, &PointI16::of(5, 4)), Some(()));
        assert_eq!(r, RectU16::of(5, 4, 17, 19));
        assert_eq!(try_assign_add(&mut r, &PointI16::of(-4, -2)), Some(()));
        assert_eq!(r, RectU16::of(1, 2, 13, 17));
    }

    #[test]
    fn try_assign_add_small_rect_to_bounds() {
        let mut min_r = RectU16::of(2, 5, 12, 15);
        assert_eq!(try_assign_add(&mut min_r, &PointI16::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectU16::of(0, 0, 10, 10));

        let mut max_r = RectU16::of(u16::MAX - 12, u16::MAX - 15, u16::MAX - 2, u16::MAX - 5);
        assert_eq!(try_assign_add(&mut max_r, &PointI16::of(2, 5)), Some(()));
        assert_eq!(max_r, RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX, u16::MAX));
    }

    #[test]
    fn try_assign_add_big_rect_to_bounds() {
        let mut min_r = RectU16::of(2, 5, u16::MAX, u16::MAX);
        assert_eq!(try_assign_add(&mut min_r, &PointI16::of(-2, -5)), Some(()));
        assert_eq!(min_r, RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5));

        let mut max_r = RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5);
        assert_eq!(try_assign_add(&mut max_r, &PointI16::of(2, 5)), Some(()));
        assert_eq!(max_r, RectU16::of(2, 5, u16::MAX, u16::MAX));
    }

    #[test]
    fn try_assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectU16::of(10, 5, 20, 30);
        assert_eq!(try_assign_add(&mut r_min, &PointI16::of(-20, -20)), None);
        assert_eq!(r_min, RectU16::of(10, 5, 20, 30));

        let mut r_max = RectU16::of(u16::MAX - 20, u16::MAX - 30, u16::MAX - 5, u16::MAX - 10);
        assert_eq!(try_assign_add(&mut r_max, &PointI16::of(20, 20)), None);
        assert_eq!(r_max, RectU16::of(u16::MAX - 20, u16::MAX - 30, u16::MAX - 5, u16::MAX - 10));
    }

    #[test]
    fn try_assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectU16::of(10, 5, u16::MAX, u16::MAX);
        assert_eq!(try_assign_add(&mut r_min, &PointI16::of(-20, -20)), None);
        assert_eq!(r_min, RectU16::of(10, 5, u16::MAX, u16::MAX));

        let mut r_max = RectU16::of(0, 0, u16::MAX - 5, u16::MAX - 10);
        assert_eq!(try_assign_add(&mut r_max, &PointI16::of(20, 20)), None);
        assert_eq!(r_max, RectU16::of(0, 0, u16::MAX - 5, u16::MAX - 10));
    }

    #[test]
    fn try_assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU16::of(1, 1, 10, 10);
        assert_eq!(try_assign_add(&mut r_min, &PointI16::min()), None);
        assert_eq!(try_assign_add(&mut r_min, &PointI16::of(i16::MIN, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &PointI16::of(0, i16::MIN)), None);
        assert_eq!(r_min, RectU16::of(1, 1, 10, 10));

        let mut r_max = RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX - 1, u16::MAX - 1);
        assert_eq!(try_assign_add(&mut r_max, &PointI16::max()), None);
        assert_eq!(try_assign_add(&mut r_max, &PointI16::of(i16::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &PointI16::of(0, i16::MAX)), None);
        assert_eq!(r_max, RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX - 1, u16::MAX - 1));
    }

    #[test]
    fn try_assign_add_big_rect_limits_out_of_bounds() {
        let mut r = RectU16::largest();
        assert_eq!(try_assign_add(&mut r, &PointI16::min()), None);
        assert_eq!(try_assign_add(&mut r, &PointI16::max()), None);
        assert_eq!(r, RectU16::largest());

        let mut r_min = RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1);
        assert_eq!(try_assign_add(&mut r_min, &PointI16::max()), None);
        assert_eq!(try_assign_add(&mut r_min, &PointI16::of(i16::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &PointI16::of(0, i16::MAX)), None);
        assert_eq!(r_min, RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1));

        let mut r_max = RectU16::of(1, 1, u16::MAX, u16::MAX);
        assert_eq!(try_assign_add(&mut r_max, &PointI16::min()), None);
        assert_eq!(try_assign_add(&mut r_max, &PointI16::of(i16::MIN, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &PointI16::of(0, i16::MIN)), None);
        assert_eq!(r_max, RectU16::of(1, 1, u16::MAX, u16::MAX));
    }

    #[test]
    fn test_try_add() {
        assert_eq!(try_add(&RectU16::of(0, 0, 12, 15), &PointI16::of(5, 4)), Some(RectU16::of(5, 4, 17, 19)));
        assert_eq!(try_add(&RectU16::of(5, 4, 17, 19), &PointI16::of(-4, -2)), Some(RectU16::of(1, 2, 13, 17)));
    }

    #[test]
    fn try_add_small_rect_to_bounds() {
        assert_eq!(try_add(&RectU16::of(2, 5, 12, 15), &PointI16::of(-2, -5)), Some(RectU16::of(0, 0, 10, 10)));
        assert_eq!(
            try_add(&RectU16::of(u16::MAX - 12, u16::MAX - 15, u16::MAX - 2, u16::MAX - 5), &PointI16::of(2, 5)),
            Some(RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX, u16::MAX))
        );
    }

    #[test]
    fn try_add_big_rect_to_bounds() {
        assert_eq!(try_add(&RectU16::of(2, 5, u16::MAX, u16::MAX), &PointI16::of(-2, -5)), Some(RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5)));
        assert_eq!(try_add(&RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 5), &PointI16::of(2, 5)), Some(RectU16::of(2, 5, u16::MAX, u16::MAX)));
    }

    #[test]
    fn try_add_small_rect_out_of_bounds() {
        assert_eq!(try_add(&RectU16::of(10, 5, 20, 30), &PointI16::of(-20, -20)), None);
        assert_eq!(try_add(&RectU16::of(u16::MAX - 20, u16::MAX - 30, u16::MAX - 5, u16::MAX - 10), &PointI16::of(20, 20)), None);
    }

    #[test]
    fn try_add_big_rect_out_of_bounds() {
        assert_eq!(try_add(&RectU16::of(10, 5, u16::MAX, u16::MAX), &PointI16::of(-20, -20)), None);
        assert_eq!(try_add(&RectU16::of(0, 0, u16::MAX - 5, u16::MAX - 10), &PointI16::of(20, 20)), None);
    }

    #[test]
    fn try_add_small_rect_limits_out_of_bounds() {
        assert_eq!(try_add(&RectU16::of(1, 1, 10, 10), &PointI16::min()), None);
        assert_eq!(try_add(&RectU16::of(1, 1, 10, 10), &PointI16::of(i16::MIN, 0)), None);
        assert_eq!(try_add(&RectU16::of(1, 1, 10, 10), &PointI16::of(0, i16::MIN)), None);
        assert_eq!(try_add(&RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX - 1, u16::MAX - 1), &PointI16::max()), None);
        assert_eq!(try_add(&RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX - 1, u16::MAX - 1), &PointI16::of(i16::MAX, 0)), None);
        assert_eq!(try_add(&RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX - 1, u16::MAX - 1), &PointI16::of(0, i16::MAX)), None);
    }

    #[test]
    fn try_add_big_rect_limits_out_of_bounds() {
        assert_eq!(try_add(&RectU16::largest(), &PointI16::min()), None);
        assert_eq!(try_add(&RectU16::largest(), &PointI16::max()), None);
        assert_eq!(try_add(&RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1), &PointI16::max()), None);
        assert_eq!(try_add(&RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1), &PointI16::of(i16::MAX, 0)), None);
        assert_eq!(try_add(&RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1), &PointI16::of(0, i16::MAX)), None);
        assert_eq!(try_add(&RectU16::of(1, 1, u16::MAX, u16::MAX), &PointI16::min()), None);
        assert_eq!(try_add(&RectU16::of(1, 1, u16::MAX, u16::MAX), &PointI16::of(i16::MIN, 0)), None);
        assert_eq!(try_add(&RectU16::of(1, 1, u16::MAX, u16::MAX), &PointI16::of(0, i16::MIN)), None);
    }

    #[test]
    fn test_assign_add() {
        let mut r = RectU16::of(0, 0, 12, 15);
        assign_add(&mut r, &PointI16::of(5, 4));
        assert_eq!(r, RectU16::of(5, 4, 17, 19));
        assign_add(&mut r, &PointI16::of(-4, -2));
        assert_eq!(r, RectU16::of(1, 2, 13, 17));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&RectU16::of(0, 0, 12, 15), &PointI16::of(5, 4)), RectU16::of(5, 4, 17, 19));
        assert_eq!(add(&RectU16::of(5, 4, 17, 19), &PointI16::of(-4, -2)), RectU16::of(1, 2, 13, 17));
    }
}
