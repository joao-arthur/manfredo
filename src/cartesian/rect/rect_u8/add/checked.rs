use crate::cartesian::{
    point::point_u8::PointU8,
    rect::{rect_i8::RectI8, rect_u8::RectU8},
};

pub fn try_assign_add(r: &mut RectU8, delta: &RectI8) -> Option<()> {
    let min_x = u8::try_from(i16::from(r.min.x) + i16::from(delta.min.x)).ok()?;
    let min_y = u8::try_from(i16::from(r.min.y) + i16::from(delta.min.y)).ok()?;
    let max_x = u8::try_from(i16::from(r.max.x) + i16::from(delta.max.x)).ok()?;
    let max_y = u8::try_from(i16::from(r.max.y) + i16::from(delta.max.y)).ok()?;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Some(())
}

pub fn try_add(r: &RectU8, delta: &RectI8) -> Option<RectU8> {
    let min_x = u8::try_from(i16::from(r.min.x) + i16::from(delta.min.x)).ok()?;
    let min_y = u8::try_from(i16::from(r.min.y) + i16::from(delta.min.y)).ok()?;
    let max_x = u8::try_from(i16::from(r.max.x) + i16::from(delta.max.x)).ok()?;
    let max_y = u8::try_from(i16::from(r.max.y) + i16::from(delta.max.y)).ok()?;
    Some(RectU8 { min: PointU8 { x: min_x, y: min_y }, max: PointU8 { x: max_x, y: max_y } })
}

pub fn assign_add(r: &mut RectU8, delta: &RectI8) {
    try_assign_add(r, delta).unwrap()
}

pub fn add(r: &RectU8, delta: &RectI8) -> RectU8 {
    try_add(r, delta).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::{rect_i8::RectI8, rect_u8::RectU8};

    use super::{add, assign_add, try_add, try_assign_add};

    #[test]
    fn test_try_assign_add() {
        let mut r = RectU8::of(0, 0, 12, 15);
        assert_eq!(try_assign_add(&mut r, &RectI8::of(5, 4, 3, 2)), Some(()));
        assert_eq!(r, RectU8::of(5, 4, 15, 17));
        assert_eq!(try_assign_add(&mut r, &RectI8::of(-4, -3, -2, -1)), Some(()));
        assert_eq!(r, RectU8::of(1, 1, 13, 16));
    }

    #[test]
    fn try_assign_add_small_rect_to_bounds() {
        let mut min_r = RectU8::of(2, 5, 12, 15);
        assert_eq!(try_assign_add(&mut min_r, &RectI8::of(-2, -5, 10, 20)), Some(()));
        assert_eq!(min_r, RectU8::of(0, 0, 22, 35));

        let mut max_r = RectU8::of(u8::MAX - 12, u8::MAX - 15, u8::MAX - 2, u8::MAX - 5);
        assert_eq!(try_assign_add(&mut max_r, &RectI8::of(-1, -4, 2, 5)), Some(()));
        assert_eq!(max_r, RectU8::of(u8::MAX - 13, u8::MAX - 19, u8::MAX, u8::MAX));
    }

    #[test]
    fn try_assign_add_big_rect_to_bounds() {
        let mut min_r = RectU8::of(2, 5, u8::MAX - 2, u8::MAX - 5);
        assert_eq!(try_assign_add(&mut min_r, &RectI8::of(-2, -5, 2, 5)), Some(()));
        assert_eq!(min_r, RectU8::largest());

        let mut min_r = RectU8::of(2, 5, u8::MAX, u8::MAX);
        assert_eq!(try_assign_add(&mut min_r, &RectI8::of(-2, -5, 0, 0)), Some(()));
        assert_eq!(min_r, RectU8::largest());

        let mut max_r = RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5);
        assert_eq!(try_assign_add(&mut max_r, &RectI8::of(0, 0, 2, 5)), Some(()));
        assert_eq!(max_r, RectU8::largest());
    }

    #[test]
    fn try_assign_add_small_rect_out_of_bounds() {
        let mut r_min = RectU8::of(10, 5, 20, 30);
        assert_eq!(try_assign_add(&mut r_min, &RectI8::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectU8::of(10, 5, 20, 30));

        let mut r_max = RectU8::of(u8::MAX - 20, u8::MAX - 30, u8::MAX - 5, u8::MAX - 10);
        assert_eq!(try_assign_add(&mut r_max, &RectI8::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectU8::of(u8::MAX - 20, u8::MAX - 30, u8::MAX - 5, u8::MAX - 10));
    }

    #[test]
    fn try_assign_add_big_rect_out_of_bounds() {
        let mut r_min = RectU8::of(10, 5, u8::MAX, u8::MAX);
        assert_eq!(try_assign_add(&mut r_min, &RectI8::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectU8::of(10, 5, u8::MAX, u8::MAX));

        let mut r_max = RectU8::of(0, 0, u8::MAX - 5, u8::MAX - 10);
        assert_eq!(try_assign_add(&mut r_max, &RectI8::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectU8::of(0, 0, u8::MAX - 5, u8::MAX - 10));
    }

    #[test]
    fn try_assign_add_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU8::of(1, 1, 10, 10);
        assert_eq!(try_assign_add(&mut r_min, &RectI8::min()), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI8::of(i8::MIN, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI8::of(0, i8::MIN, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI8::of(0, 0, i8::MIN, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI8::of(0, 0, 0, i8::MIN)), None);
        assert_eq!(r_min, RectU8::of(1, 1, 10, 10));

        let mut r_max = RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX - 1, u8::MAX - 1);
        assert_eq!(try_assign_add(&mut r_max, &RectI8::max()), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI8::of(i8::MAX, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI8::of(0, i8::MAX, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI8::of(0, 0, i8::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI8::of(0, 0, 0, i8::MAX)), None);
        assert_eq!(r_max, RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX - 1, u8::MAX - 1));
    }

    #[test]
    fn try_assign_add_big_rect_limits_out_of_bounds() {
        let mut r = RectU8::largest();
        assert_eq!(try_assign_add(&mut r, &RectI8::largest()), None);
        assert_eq!(try_assign_add(&mut r, &RectI8::min()), None);
        assert_eq!(try_assign_add(&mut r, &RectI8::max()), None);
        assert_eq!(r, RectU8::largest());

        let mut r_min = RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1);
        assert_eq!(try_assign_add(&mut r_min, &RectI8::max()), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI8::of(0, 0, i8::MAX, 0)), None);
        assert_eq!(try_assign_add(&mut r_min, &RectI8::of(0, 0, 0, i8::MAX)), None);
        assert_eq!(r_min, RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1));

        let mut r_max = RectU8::of(1, 1, u8::MAX, u8::MAX);
        assert_eq!(try_assign_add(&mut r_max, &RectI8::min()), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI8::of(i8::MIN, 0, 0, 0)), None);
        assert_eq!(try_assign_add(&mut r_max, &RectI8::of(0, i8::MIN, 0, 0)), None);
        assert_eq!(r_max, RectU8::of(1, 1, u8::MAX, u8::MAX));
    }

    #[test]
    fn test_try_add() {
        let mut r = RectU8::of(0, 0, 12, 15);
        assert_eq!(try_add(&mut r, &RectI8::of(5, 4, 3, 2)), Some(RectU8::of(5, 4, 15, 17)));
        assert_eq!(try_add(&RectU8::of(5, 4, 15, 17), &RectI8::of(-4, -3, -2, -1)), Some(RectU8::of(1, 1, 13, 16)));
    }

    #[test]
    fn try_add_small_rect_to_bounds() {
        assert_eq!(try_add(&RectU8::of(2, 5, 12, 15), &RectI8::of(-2, -5, 10, 20)), Some(RectU8::of(0, 0, 22, 35)));
        assert_eq!(
            try_add(&RectU8::of(u8::MAX - 12, u8::MAX - 15, u8::MAX - 2, u8::MAX - 5), &RectI8::of(-1, -4, 2, 5)),
            Some(RectU8::of(u8::MAX - 13, u8::MAX - 19, u8::MAX, u8::MAX))
        );
    }

    #[test]
    fn try_add_big_rect_to_bounds() {
        assert_eq!(try_add(&RectU8::of(2, 5, u8::MAX - 2, u8::MAX - 5), &RectI8::of(-2, -5, 2, 5)), Some(RectU8::largest()));
        assert_eq!(try_add(&RectU8::of(2, 5, u8::MAX, u8::MAX), &RectI8::of(-2, -5, 0, 0)), Some(RectU8::largest()));
        assert_eq!(try_add(&RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 5), &RectI8::of(0, 0, 2, 5)), Some(RectU8::largest()));
    }

    #[test]
    fn try_add_small_rect_out_of_bounds() {
        let mut r_min = RectU8::of(10, 5, 20, 30);
        assert_eq!(try_add(&mut r_min, &RectI8::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectU8::of(10, 5, 20, 30));

        let mut r_max = RectU8::of(u8::MAX - 20, u8::MAX - 30, u8::MAX - 5, u8::MAX - 10);
        assert_eq!(try_add(&mut r_max, &RectI8::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectU8::of(u8::MAX - 20, u8::MAX - 30, u8::MAX - 5, u8::MAX - 10));
    }

    #[test]
    fn try_add_big_rect_out_of_bounds() {
        let mut r_min = RectU8::of(10, 5, u8::MAX, u8::MAX);
        assert_eq!(try_add(&mut r_min, &RectI8::of(-20, -20, 0, 0)), None);
        assert_eq!(r_min, RectU8::of(10, 5, u8::MAX, u8::MAX));

        let mut r_max = RectU8::of(0, 0, u8::MAX - 5, u8::MAX - 10);
        assert_eq!(try_add(&mut r_max, &RectI8::of(0, 0, 20, 20)), None);
        assert_eq!(r_max, RectU8::of(0, 0, u8::MAX - 5, u8::MAX - 10));
    }

    #[test]
    fn try_add_small_rect_limits_out_of_bounds() {
        let r_min = RectU8::of(1, 1, 10, 10);
        assert_eq!(try_add(&r_min, &RectI8::min()), None);
        assert_eq!(try_add(&r_min, &RectI8::of(i8::MIN, 0, 0, 0)), None);
        assert_eq!(try_add(&r_min, &RectI8::of(0, i8::MIN, 0, 0)), None);
        assert_eq!(try_add(&r_min, &RectI8::of(0, 0, i8::MIN, 0)), None);
        assert_eq!(try_add(&r_min, &RectI8::of(0, 0, 0, i8::MIN)), None);

        let r_max = RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX - 1, u8::MAX - 1);
        assert_eq!(try_add(&r_max, &RectI8::max()), None);
        assert_eq!(try_add(&r_max, &RectI8::of(i8::MAX, 0, 0, 0)), None);
        assert_eq!(try_add(&r_max, &RectI8::of(0, i8::MAX, 0, 0)), None);
        assert_eq!(try_add(&r_max, &RectI8::of(0, 0, i8::MAX, 0)), None);
        assert_eq!(try_add(&r_max, &RectI8::of(0, 0, 0, i8::MAX)), None);
    }

    #[test]
    fn try_add_big_rect_limits_out_of_bounds() {
        let r = RectU8::largest();
        assert_eq!(try_add(&r, &RectI8::largest()), None);
        assert_eq!(try_add(&r, &RectI8::min()), None);
        assert_eq!(try_add(&r, &RectI8::max()), None);

        let r_min = RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1);
        assert_eq!(try_add(&r_min, &RectI8::max()), None);
        assert_eq!(try_add(&r_min, &RectI8::of(0, 0, i8::MAX, 0)), None);
        assert_eq!(try_add(&r_min, &RectI8::of(0, 0, 0, i8::MAX)), None);

        let r_max = RectU8::of(1, 1, u8::MAX, u8::MAX);
        assert_eq!(try_add(&r_max, &RectI8::min()), None);
        assert_eq!(try_add(&r_max, &RectI8::of(i8::MIN, 0, 0, 0)), None);
        assert_eq!(try_add(&r_max, &RectI8::of(0, i8::MIN, 0, 0)), None);
    }

    #[test]
    fn test_assign_add() {
        let mut r = RectU8::of(0, 0, 12, 15);
        assign_add(&mut r, &RectI8::of(5, 4, 3, 2));
        assert_eq!(r, RectU8::of(5, 4, 15, 17));
        assign_add(&mut r, &RectI8::of(-4, -3, -2, -1));
        assert_eq!(r, RectU8::of(1, 1, 13, 16));
    }

    #[test]
    fn test_add() {
        let mut r = RectU8::of(0, 0, 12, 15);
        assert_eq!(add(&mut r, &RectI8::of(5, 4, 3, 2)), RectU8::of(5, 4, 15, 17));
        assert_eq!(add(&RectU8::of(5, 4, 15, 17), &RectI8::of(-4, -3, -2, -1)), RectU8::of(1, 1, 13, 16));
    }
}
