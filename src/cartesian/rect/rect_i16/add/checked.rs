use crate::cartesian::{point::point_i16::PointI16, rect::rect_i16::RectI16};

pub fn try_assign_add(r: &mut RectI16, delta: &RectI16) -> Option<()> {
    let min_x = r.min.x.checked_add(delta.min.x)?;
    let min_y = r.min.y.checked_add(delta.min.y)?;
    let max_x = r.max.x.checked_add(delta.max.x)?;
    let max_y = r.max.y.checked_add(delta.max.y)?;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

pub fn try_add(r: &RectI16, delta: &RectI16) -> Option<RectI16> {
    let min_x = r.min.x.checked_add(delta.min.x)?;
    let min_y = r.min.y.checked_add(delta.min.y)?;
    let max_x = r.max.x.checked_add(delta.max.x)?;
    let max_y = r.max.y.checked_add(delta.max.y)?;
    Some(RectI16 { min: PointI16 { x: min_x, y: min_y }, max: PointI16 { x: max_x, y: max_y } })
}

pub fn assign_add(r: &mut RectI16, delta: &RectI16) {
    try_assign_add(r, delta).unwrap()
}

pub fn add(r: &RectI16, delta: &RectI16) -> RectI16 {
    try_add(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_i16::RectI16;

    use super::{add, assign_add, try_add, try_assign_add};

    #[test]
    fn test_try_assign_add() {
        let mut r = RectI16::of(0, 0, 12, 15);
        assert_eq!(try_assign_add(&mut r, &RectI16::of(5, 4, 3, 2)), Some(()));
        assert_eq!(r, RectI16::of(5, 4, 15, 17));
        assert_eq!(try_assign_add(&mut r, &RectI16::of(-14, -13, -12, -11)), Some(()));
        assert_eq!(r, RectI16::of(-9, -9, 3, 6));
    }

    #[test]
    fn try_assign_add_small_rect_to_bounds() {
        let mut min_r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MIN + 12, i16::MIN + 15);
        assert_eq!(try_assign_add(&mut min_r, &RectI16::of(-2, -5, 10, 20)), Some(()));
        assert_eq!(min_r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 22, i16::MIN + 35));

        let mut max_r = RectI16::of(i16::MAX - 12, i16::MAX - 15, i16::MAX - 2, i16::MAX - 5);
        assert_eq!(try_assign_add(&mut max_r, &RectI16::of(-1, -4, 2, 5)), Some(()));
        assert_eq!(max_r, RectI16::of(i16::MAX - 13, i16::MAX - 19, i16::MAX, i16::MAX));
    }

    #[test]
    fn try_assign_add_big_rect_to_bounds() {
        let mut min_r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX - 2, i16::MAX - 5);
        assert_eq!(try_assign_add(&mut min_r, &RectI16::of(-2, -5, 2, 5)), Some(()));
        assert_eq!(min_r, RectI16::largest());

        let mut min_r = RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX);
        assert_eq!(try_assign_add(&mut min_r, &RectI16::of(-2, -5, 0, 0)), Some(()));
        assert_eq!(min_r, RectI16::largest());

        let mut max_r = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5);
        assert_eq!(try_assign_add(&mut max_r, &RectI16::of(0, 0, 2, 5)), Some(()));
        assert_eq!(max_r, RectI16::largest());
    }

    #[test]
    fn try_assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MIN + 20, i16::MIN + 30);
        assert_eq!(try_assign_add(&mut r_min, &RectI16::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MIN + 20, i16::MIN + 30));

        let mut r_max = RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX - 5, i16::MAX - 10);
        assert_eq!(try_assign_add(&mut r_max, &RectI16::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX - 5, i16::MAX - 10));
    }

    #[test]
    fn try_assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MAX, i16::MAX);
        assert_eq!(try_assign_add(&mut r_min, &RectI16::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MAX, i16::MAX));

        let mut r_max = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 5, i16::MAX - 10);
        assert_eq!(try_assign_add(&mut r_max, &RectI16::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 5, i16::MAX - 10));
    }

    #[test]
    fn try_assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10);
        assert_eq!(try_assign_add(&mut r_min, &RectI16::min()), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI16::of(i16::MIN, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI16::of(0, i16::MIN, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI16::of(0, 0, i16::MIN, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI16::of(0, 0, 0, i16::MIN)), None);
        assert_eq!(r_min, RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10));

        let mut r_max = RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1);
        assert_eq!(try_assign_add(&mut r_max, &RectI16::max()), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI16::of(i16::MAX, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI16::of(0, i16::MAX, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI16::of(0, 0, i16::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI16::of(0, 0, 0, i16::MAX)), None);
        assert_eq!(r_max, RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1));
    }

    #[test]
    fn try_assign_add_big_rect_limits_out_of_bounds() {
        let mut r = RectI16::largest();
        assert_eq!(try_assign_add(&mut r, &RectI16::largest()), None);
        assert_eq!(try_assign_add(&mut r, &RectI16::min()), None);
        assert_eq!(try_assign_add(&mut r, &RectI16::max()), None);
        assert_eq!(r, RectI16::largest());

        let mut r_min = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1);
        assert_eq!(try_assign_add(&mut r_min, &RectI16::max()), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI16::of(0, 0, i16::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI16::of(0, 0, 0, i16::MAX)), None);
        assert_eq!(r_min, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1));

        let mut r_max = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX);
        assert_eq!(try_assign_add(&mut r_max, &RectI16::min()), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI16::of(i16::MIN, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI16::of(0, i16::MIN, 0, 0)), None);
        assert_eq!(r_max, RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX));
    }

    #[test]
    fn test_try_add() {
        assert_eq!(try_add(&RectI16::of(0, 0, 12, 15), &RectI16::of(5, 4, 3, 2)), Some(RectI16::of(5, 4, 15, 17)));
        assert_eq!(try_add(&RectI16::of(5, 4, 15, 17), &RectI16::of(-14, -13, -12, -11)), Some(RectI16::of(-9, -9, 3, 6)));
    }

    #[test]
    fn try_add_small_rect_to_bounds() {
        assert_eq!(
            try_add(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MIN + 12, i16::MIN + 15), &RectI16::of(-2, -5, 10, 20)),
            Some(RectI16::of(i16::MIN, i16::MIN, i16::MIN + 22, i16::MIN + 35))
        );
        assert_eq!(
            try_add(&RectI16::of(i16::MAX - 12, i16::MAX - 15, i16::MAX - 2, i16::MAX - 5), &RectI16::of(-1, -4, 2, 5)),
            Some(RectI16::of(i16::MAX - 13, i16::MAX - 19, i16::MAX, i16::MAX))
        );
    }

    #[test]
    fn try_add_big_rect_to_bounds() {
        assert_eq!(
            try_add(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX - 2, i16::MAX - 5), &RectI16::of(-2, -5, 2, 5)),
            Some(RectI16::largest())
        );
        assert_eq!(try_add(&RectI16::of(i16::MIN + 2, i16::MIN + 5, i16::MAX, i16::MAX), &RectI16::of(-2, -5, 0, 0)), Some(RectI16::largest()));
        assert_eq!(try_add(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 5), &RectI16::of(0, 0, 2, 5)), Some(RectI16::largest()));
    }

    #[test]
    fn try_add_small_rect_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MIN + 20, i16::MIN + 30);
        assert_eq!(try_add(&mut r_min, &RectI16::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MIN + 20, i16::MIN + 30));

        let mut r_max = RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX - 5, i16::MAX - 10);
        assert_eq!(try_add(&mut r_max, &RectI16::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectI16::of(i16::MAX - 20, i16::MAX - 30, i16::MAX - 5, i16::MAX - 10));
    }

    #[test]
    fn try_add_big_rect_out_of_bounds() {
        let mut r_min = RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MAX, i16::MAX);
        assert_eq!(try_add(&mut r_min, &RectI16::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectI16::of(i16::MIN + 10, i16::MIN + 5, i16::MAX, i16::MAX));

        let mut r_max = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 5, i16::MAX - 10);
        assert_eq!(try_add(&mut r_max, &RectI16::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 5, i16::MAX - 10));
    }

    #[test]
    fn try_add_small_rect_limits_out_of_bounds() {
        let r_min = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MIN + 10, i16::MIN + 10);
        assert_eq!(try_add(&r_min, &RectI16::min()), None);
        assert_eq!(try_add(&r_min, &RectI16::of(i16::MIN, 0, 0, 0)), None);
        assert_eq!(try_add(&r_min, &RectI16::of(0, i16::MIN, 0, 0)), None);
        assert_eq!(try_add(&r_min, &RectI16::of(0, 0, i16::MIN, 0)), None);
        assert_eq!(try_add(&r_min, &RectI16::of(0, 0, 0, i16::MIN)), None);

        let r_max = RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX - 1, i16::MAX - 1);
        assert_eq!(try_add(&r_max, &RectI16::max()), None);
        assert_eq!(try_add(&r_max, &RectI16::of(i16::MAX, 0, 0, 0)), None);
        assert_eq!(try_add(&r_max, &RectI16::of(0, i16::MAX, 0, 0)), None);
        assert_eq!(try_add(&r_max, &RectI16::of(0, 0, i16::MAX, 0)), None);
        assert_eq!(try_add(&r_max, &RectI16::of(0, 0, 0, i16::MAX)), None);
    }

    #[test]
    fn try_add_big_rect_limits_out_of_bounds() {
        let r = RectI16::largest();
        assert_eq!(try_add(&r, &RectI16::largest()), None);
        assert_eq!(try_add(&r, &RectI16::min()), None);
        assert_eq!(try_add(&r, &RectI16::max()), None);

        let r_min = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1);
        assert_eq!(try_add(&r_min, &RectI16::max()), None);
        assert_eq!(try_add(&r_min, &RectI16::of(0, 0, i16::MAX, 0)), None);
        assert_eq!(try_add(&r_min, &RectI16::of(0, 0, 0, i16::MAX)), None);

        let r_max = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX);
        assert_eq!(try_add(&r_max, &RectI16::min()), None);
        assert_eq!(try_add(&r_max, &RectI16::of(i16::MIN, 0, 0, 0)), None);
        assert_eq!(try_add(&r_max, &RectI16::of(0, i16::MIN, 0, 0)), None);
    }

    #[test]
    fn test_assign_add() {
        let mut r = RectI16::of(0, 0, 12, 15);
        assign_add(&mut r, &RectI16::of(5, 4, 3, 2));
        assert_eq!(r, RectI16::of(5, 4, 15, 17));
        assign_add(&mut r, &RectI16::of(-14, -13, -12, -11));
        assert_eq!(r, RectI16::of(-9, -9, 3, 6));
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&RectI16::of(0, 0, 12, 15), &RectI16::of(5, 4, 3, 2)), RectI16::of(5, 4, 15, 17));
        assert_eq!(add(&RectI16::of(5, 4, 15, 17), &RectI16::of(-14, -13, -12, -11)), RectI16::of(-9, -9, 3, 6));
    }
}
