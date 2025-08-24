use crate::cartesian::{point::point_u32::PointU32, rect::rect_u32::RectU32};

pub fn try_saturating_inflate_assign(r: &mut RectU32) -> Option<()> {
    let is_min_x = r.min.x == 0;
    let is_min_y = r.min.y == 0;
    let is_max_x = r.max.x == u32::MAX;
    let is_max_y = r.max.y == u32::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - u32::from(is_min_x) + u32::from(is_max_x);
    let min_y_modifier = 1 - u32::from(is_min_y) + u32::from(is_max_y);
    let max_x_modifier = 1 + u32::from(is_min_x) - u32::from(is_max_x);
    let max_y_modifier = 1 + u32::from(is_min_y) - u32::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
    Some(())
}

pub fn try_saturating_inflate(r: &RectU32) -> Option<RectU32> {
    let is_min_x = r.min.x == 0;
    let is_min_y = r.min.y == 0;
    let is_max_x = r.max.x == u32::MAX;
    let is_max_y = r.max.y == u32::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - u32::from(is_min_x) + u32::from(is_max_x);
    let min_y_modifier = 1 - u32::from(is_min_y) + u32::from(is_max_y);
    let max_x_modifier = 1 + u32::from(is_min_x) - u32::from(is_max_x);
    let max_y_modifier = 1 + u32::from(is_min_y) - u32::from(is_max_y);
    let min_x = r.min.x.saturating_sub(min_x_modifier);
    let min_y = r.min.y.saturating_sub(min_y_modifier);
    let max_x = r.max.x.saturating_add(max_x_modifier);
    let max_y = r.max.y.saturating_add(max_y_modifier);
    Some(RectU32 { min: PointU32 { x: min_x, y: min_y }, max: PointU32 { x: max_x, y: max_y } })
}

pub fn saturating_inflate_assign(r: &mut RectU32) {
    try_saturating_inflate_assign(r).unwrap()
}

pub fn saturating_inflate(r: &RectU32) -> RectU32 {
    try_saturating_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{saturating_inflate_assign, saturating_inflate, try_saturating_inflate_assign, try_saturating_inflate};
    use crate::cartesian::rect::rect_u32::RectU32;

    #[test]
    fn try_saturating_inflate_assign_min_bounds() {
        let mut r = RectU32::of(7, 2, 17, 13);
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::of(6, 1, 18, 14));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::of(5, 0, 19, 15));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::of(4, 0, 20, 17));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::of(3, 0, 21, 19));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::of(2, 0, 22, 21));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::of(1, 0, 23, 23));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::of(0, 0, 24, 25));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::of(0, 0, 26, 27));
    }

    #[test]
    fn try_saturating_inflate_assign_max_bounds() {
        let mut r = RectU32::of(u32::MAX - 33, u32::MAX - 17, u32::MAX - 5, u32::MAX - 3);
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::of(u32::MAX - 36, u32::MAX - 20, u32::MAX - 2, u32::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::of(u32::MAX - 37, u32::MAX - 22, u32::MAX - 1, u32::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::of(u32::MAX - 38, u32::MAX - 24, u32::MAX, u32::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::of(u32::MAX - 40, u32::MAX - 26, u32::MAX, u32::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::of(u32::MAX - 42, u32::MAX - 28, u32::MAX, u32::MAX));
        assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
        assert_eq!(r, RectU32::of(u32::MAX - 44, u32::MAX - 30, u32::MAX, u32::MAX));
    }

    #[test]
    fn try_saturating_inflate_assign_to_bounds() {
        let mut r_odd = RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_odd), Some(()));
        assert_eq!(r_odd, RectU32::largest());

        let mut r_even_1 = RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_even_1), Some(()));
        assert_eq!(r_even_1, RectU32::largest());

        let mut r_even_2 = RectU32::of(1, 1, u32::MAX, u32::MAX);
        assert_eq!(try_saturating_inflate_assign(&mut r_even_2), Some(()));
        assert_eq!(r_even_2, RectU32::largest());
    }

    #[test]
    fn try_saturating_inflate_assign_width_to_bounds() {
        let mut r_min = RectU32::of(1, 10, 20, 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectU32::of(0, 9, 21, 21));

        let mut r_max = RectU32::of(10, 10, u32::MAX - 1, 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectU32::of(9, 9, u32::MAX, 21));
    }

    #[test]
    fn try_saturating_inflate_assign_height_to_bounds() {
        let mut r_min = RectU32::of(10, 1, 20, 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
        assert_eq!(r_min, RectU32::of(9, 0, 21, 21));

        let mut r_max = RectU32::of(10, 10, 20, u32::MAX - 1);
        assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
        assert_eq!(r_max, RectU32::of(9, 9, 21, u32::MAX));
    }

    #[test]
    fn try_saturating_inflate_assign_out_of_bounds() {
        let mut r = RectU32::largest();
        assert_eq!(try_saturating_inflate_assign(&mut r), None);
        assert_eq!(r, RectU32::largest());

        let mut r_x = RectU32::of(0, 10, u32::MAX, 20);
        assert_eq!(try_saturating_inflate_assign(&mut r_x), None);
        assert_eq!(r_x, RectU32::of(0, 10, u32::MAX, 20));

        let mut r_y = RectU32::of(10, 0, 20, u32::MAX);
        assert_eq!(try_saturating_inflate_assign(&mut r_y), None);
        assert_eq!(r_y, RectU32::of(10, 0, 20, u32::MAX));
    }

    #[test]
    fn try_saturating_inflate_min_bounds() {
        assert_eq!(try_saturating_inflate(&RectU32::of(7, 2, 17, 13)), Some(RectU32::of(6, 1, 18, 14)));
        assert_eq!(try_saturating_inflate(&RectU32::of(6, 1, 18, 14)), Some(RectU32::of(5, 0, 19, 15)));
        assert_eq!(try_saturating_inflate(&RectU32::of(5, 0, 19, 15)), Some(RectU32::of(4, 0, 20, 17)));
        assert_eq!(try_saturating_inflate(&RectU32::of(4, 0, 20, 17)), Some(RectU32::of(3, 0, 21, 19)));
        assert_eq!(try_saturating_inflate(&RectU32::of(3, 0, 21, 19)), Some(RectU32::of(2, 0, 22, 21)));
        assert_eq!(try_saturating_inflate(&RectU32::of(2, 0, 22, 21)), Some(RectU32::of(1, 0, 23, 23)));
        assert_eq!(try_saturating_inflate(&RectU32::of(1, 0, 23, 23)), Some(RectU32::of(0, 0, 24, 25)));
        assert_eq!(try_saturating_inflate(&RectU32::of(0, 0, 24, 25)), Some(RectU32::of(0, 0, 26, 27)));
    }

    #[test]
    fn try_saturating_inflate_max_bounds() {
        assert_eq!(
            try_saturating_inflate(&RectU32::of(u32::MAX - 33, u32::MAX - 17, u32::MAX - 5, u32::MAX - 3)),
            Some(RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2))
        );
        assert_eq!(
            try_saturating_inflate(&RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2)),
            Some(RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1))
        );
        assert_eq!(
            try_saturating_inflate(&RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1)),
            Some(RectU32::of(u32::MAX - 36, u32::MAX - 20, u32::MAX - 2, u32::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectU32::of(u32::MAX - 36, u32::MAX - 20, u32::MAX - 2, u32::MAX)),
            Some(RectU32::of(u32::MAX - 37, u32::MAX - 22, u32::MAX - 1, u32::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectU32::of(u32::MAX - 37, u32::MAX - 22, u32::MAX - 1, u32::MAX)),
            Some(RectU32::of(u32::MAX - 38, u32::MAX - 24, u32::MAX, u32::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectU32::of(u32::MAX - 38, u32::MAX - 24, u32::MAX, u32::MAX)),
            Some(RectU32::of(u32::MAX - 40, u32::MAX - 26, u32::MAX, u32::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectU32::of(u32::MAX - 40, u32::MAX - 26, u32::MAX, u32::MAX)),
            Some(RectU32::of(u32::MAX - 42, u32::MAX - 28, u32::MAX, u32::MAX))
        );
        assert_eq!(
            try_saturating_inflate(&RectU32::of(u32::MAX - 42, u32::MAX - 28, u32::MAX, u32::MAX)),
            Some(RectU32::of(u32::MAX - 44, u32::MAX - 30, u32::MAX, u32::MAX))
        );
    }

    #[test]
    fn try_saturating_inflate_to_bounds() {
        assert_eq!(try_saturating_inflate(&RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1)), Some(RectU32::largest()));
        assert_eq!(try_saturating_inflate(&RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1)), Some(RectU32::largest()));
        assert_eq!(try_saturating_inflate(&RectU32::of(1, 1, u32::MAX, u32::MAX)), Some(RectU32::largest()));
    }

    #[test]
    fn try_saturating_inflate_width_to_bounds() {
        assert_eq!(try_saturating_inflate(&RectU32::of(1, 10, 20, 20)), Some(RectU32::of(0, 9, 21, 21)));
        assert_eq!(try_saturating_inflate(&RectU32::of(10, 10, u32::MAX - 1, 20)), Some(RectU32::of(9, 9, u32::MAX, 21)));
    }

    #[test]
    fn try_saturating_inflate_height_to_bounds() {
        assert_eq!(try_saturating_inflate(&RectU32::of(10, 1, 20, 20)), Some(RectU32::of(9, 0, 21, 21)));
        assert_eq!(try_saturating_inflate(&RectU32::of(10, 10, 20, u32::MAX - 1)), Some(RectU32::of(9, 9, 21, u32::MAX)));
    }

    #[test]
    fn try_saturating_inflate_out_of_bounds() {
        assert_eq!(try_saturating_inflate(&RectU32::largest()), None);
        assert_eq!(try_saturating_inflate(&RectU32::of(0, 10, u32::MAX, 20)), None);
        assert_eq!(try_saturating_inflate(&RectU32::of(10, 0, 20, u32::MAX)), None);
    }

    #[test]
    fn saturating_inflate_assign_min_bounds() {
        let mut r = RectU32::of(7, 2, 17, 13);
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU32::of(6, 1, 18, 14));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU32::of(5, 0, 19, 15));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU32::of(4, 0, 20, 17));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU32::of(3, 0, 21, 19));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU32::of(2, 0, 22, 21));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU32::of(1, 0, 23, 23));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU32::of(0, 0, 24, 25));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU32::of(0, 0, 26, 27));
    }

    #[test]
    fn saturating_inflate_assign_max_bounds() {
        let mut r = RectU32::of(u32::MAX - 33, u32::MAX - 17, u32::MAX - 5, u32::MAX - 3);
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU32::of(u32::MAX - 36, u32::MAX - 20, u32::MAX - 2, u32::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU32::of(u32::MAX - 37, u32::MAX - 22, u32::MAX - 1, u32::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU32::of(u32::MAX - 38, u32::MAX - 24, u32::MAX, u32::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU32::of(u32::MAX - 40, u32::MAX - 26, u32::MAX, u32::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU32::of(u32::MAX - 42, u32::MAX - 28, u32::MAX, u32::MAX));
        saturating_inflate_assign(&mut r);
        assert_eq!(r, RectU32::of(u32::MAX - 44, u32::MAX - 30, u32::MAX, u32::MAX));
    }

    #[test]
    fn saturating_inflate_min_bounds() {
        assert_eq!(saturating_inflate(&RectU32::of(7, 2, 17, 13)), RectU32::of(6, 1, 18, 14));
        assert_eq!(saturating_inflate(&RectU32::of(6, 1, 18, 14)), RectU32::of(5, 0, 19, 15));
        assert_eq!(saturating_inflate(&RectU32::of(5, 0, 19, 15)), RectU32::of(4, 0, 20, 17));
        assert_eq!(saturating_inflate(&RectU32::of(4, 0, 20, 17)), RectU32::of(3, 0, 21, 19));
        assert_eq!(saturating_inflate(&RectU32::of(3, 0, 21, 19)), RectU32::of(2, 0, 22, 21));
        assert_eq!(saturating_inflate(&RectU32::of(2, 0, 22, 21)), RectU32::of(1, 0, 23, 23));
        assert_eq!(saturating_inflate(&RectU32::of(1, 0, 23, 23)), RectU32::of(0, 0, 24, 25));
        assert_eq!(saturating_inflate(&RectU32::of(0, 0, 24, 25)), RectU32::of(0, 0, 26, 27));
    }

    #[test]
    fn saturating_inflate_max_bounds() {
        assert_eq!(
            saturating_inflate(&RectU32::of(u32::MAX - 33, u32::MAX - 17, u32::MAX - 5, u32::MAX - 3)),
            RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2)
        );
        assert_eq!(
            saturating_inflate(&RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2)),
            RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1)
        );
        assert_eq!(
            saturating_inflate(&RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1)),
            RectU32::of(u32::MAX - 36, u32::MAX - 20, u32::MAX - 2, u32::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectU32::of(u32::MAX - 36, u32::MAX - 20, u32::MAX - 2, u32::MAX)),
            RectU32::of(u32::MAX - 37, u32::MAX - 22, u32::MAX - 1, u32::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectU32::of(u32::MAX - 37, u32::MAX - 22, u32::MAX - 1, u32::MAX)),
            RectU32::of(u32::MAX - 38, u32::MAX - 24, u32::MAX, u32::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectU32::of(u32::MAX - 38, u32::MAX - 24, u32::MAX, u32::MAX)),
            RectU32::of(u32::MAX - 40, u32::MAX - 26, u32::MAX, u32::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectU32::of(u32::MAX - 40, u32::MAX - 26, u32::MAX, u32::MAX)),
            RectU32::of(u32::MAX - 42, u32::MAX - 28, u32::MAX, u32::MAX)
        );
        assert_eq!(
            saturating_inflate(&RectU32::of(u32::MAX - 42, u32::MAX - 28, u32::MAX, u32::MAX)),
            RectU32::of(u32::MAX - 44, u32::MAX - 30, u32::MAX, u32::MAX)
        );
    }
}
