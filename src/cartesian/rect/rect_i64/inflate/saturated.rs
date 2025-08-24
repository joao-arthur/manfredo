use crate::cartesian::{point::point_i64::PointI64, rect::rect_i64::RectI64};

pub fn try_assign_inflate(r: &mut RectI64) -> Option<()> {
    let is_min_x = r.min.x == i64::MIN;
    let is_min_y = r.min.y == i64::MIN;
    let is_max_x = r.max.x == i64::MAX;
    let is_max_y = r.max.y == i64::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - i64::from(is_min_x) + i64::from(is_max_x);
    let min_y_modifier = 1 - i64::from(is_min_y) + i64::from(is_max_y);
    let max_x_modifier = 1 + i64::from(is_min_x) - i64::from(is_max_x);
    let max_y_modifier = 1 + i64::from(is_min_y) - i64::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
    Some(())
}

pub fn try_inflate(r: &RectI64) -> Option<RectI64> {
    let is_min_x = r.min.x == i64::MIN;
    let is_min_y = r.min.y == i64::MIN;
    let is_max_x = r.max.x == i64::MAX;
    let is_max_y = r.max.y == i64::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - i64::from(is_min_x) + i64::from(is_max_x);
    let min_y_modifier = 1 - i64::from(is_min_y) + i64::from(is_max_y);
    let max_x_modifier = 1 + i64::from(is_min_x) - i64::from(is_max_x);
    let max_y_modifier = 1 + i64::from(is_min_y) - i64::from(is_max_y);
    let min_x = r.min.x.saturating_sub(min_x_modifier);
    let min_y = r.min.y.saturating_sub(min_y_modifier);
    let max_x = r.max.x.saturating_add(max_x_modifier);
    let max_y = r.max.y.saturating_add(max_y_modifier);
    Some(RectI64 { min: PointI64 { x: min_x, y: min_y }, max: PointI64 { x: max_x, y: max_y } })
}

pub fn assign_inflate(r: &mut RectI64) {
    try_assign_inflate(r).unwrap()
}

pub fn inflate(r: &RectI64) -> RectI64 {
    try_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{assign_inflate, inflate, try_assign_inflate, try_inflate};
    use crate::cartesian::rect::rect_i64::RectI64;

    #[test]
    fn try_assign_inflate_min_bounds() {
        let mut r = RectI64::of(i64::MIN + 7, i64::MIN + 2, i64::MIN + 17, i64::MIN + 13);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN + 6, i64::MIN + 1, i64::MIN + 18, i64::MIN + 14));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN + 5, i64::MIN, i64::MIN + 19, i64::MIN + 15));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 20, i64::MIN + 17));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN + 3, i64::MIN, i64::MIN + 21, i64::MIN + 19));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN + 2, i64::MIN, i64::MIN + 22, i64::MIN + 21));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN + 1, i64::MIN, i64::MIN + 23, i64::MIN + 23));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 24, i64::MIN + 25));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 26, i64::MIN + 27));
    }

    #[test]
    fn try_assign_inflate_max_bounds() {
        let mut r = RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 37, i64::MAX - 22, i64::MAX - 1, i64::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 38, i64::MAX - 24, i64::MAX, i64::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 40, i64::MAX - 26, i64::MAX, i64::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 42, i64::MAX - 28, i64::MAX, i64::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI64::of(i64::MAX - 44, i64::MAX - 30, i64::MAX, i64::MAX));
    }

    #[test]
    fn try_assign_inflate_to_bounds() {
        let mut r_odd = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_odd), Some(()));
        assert_eq!(r_odd, RectI64::largest());

        let mut r_even_1 = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_even_1), Some(()));
        assert_eq!(r_even_1, RectI64::largest());

        let mut r_even_2 = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX);
        assert_eq!(try_assign_inflate(&mut r_even_2), Some(()));
        assert_eq!(r_even_2, RectI64::largest());
    }

    #[test]
    fn try_assign_inflate_width_to_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 1, i64::MIN + 10, i64::MIN + 20, i64::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectI64::of(i64::MIN, i64::MIN + 9, i64::MIN + 21, i64::MIN + 21));

        let mut r_max = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 1, i64::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MAX, i64::MIN + 21));
    }

    #[test]
    fn try_assign_inflate_height_to_bounds() {
        let mut r_min = RectI64::of(i64::MIN + 10, i64::MIN + 1, i64::MIN + 20, i64::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectI64::of(i64::MIN + 9, i64::MIN, i64::MIN + 21, i64::MIN + 21));

        let mut r_max = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MIN + 20, i64::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MIN + 21, i64::MAX));
    }

    #[test]
    fn try_assign_inflate_out_of_bounds() {
        let mut r = RectI64::largest();
        assert_eq!(try_assign_inflate(&mut r), None);
        assert_eq!(r, RectI64::largest());

        let mut r_x = RectI64::of(i64::MIN, i64::MIN + 10, i64::MAX, i64::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_x), None);
        assert_eq!(r_x, RectI64::of(i64::MIN, i64::MIN + 10, i64::MAX, i64::MIN + 20));

        let mut r_y = RectI64::of(i64::MIN + 10, i64::MIN, i64::MIN + 20, i64::MAX);
        assert_eq!(try_assign_inflate(&mut r_y), None);
        assert_eq!(r_y, RectI64::of(i64::MIN + 10, i64::MIN, i64::MIN + 20, i64::MAX));
    }

    #[test]
    fn try_inflate_min_bounds() {
        assert_eq!(
            try_inflate(&RectI64::of(i64::MIN + 7, i64::MIN + 2, i64::MIN + 17, i64::MIN + 13)),
            Some(RectI64::of(i64::MIN + 6, i64::MIN + 1, i64::MIN + 18, i64::MIN + 14))
        );
        assert_eq!(
            try_inflate(&RectI64::of(i64::MIN + 6, i64::MIN + 1, i64::MIN + 18, i64::MIN + 14)),
            Some(RectI64::of(i64::MIN + 5, i64::MIN, i64::MIN + 19, i64::MIN + 15))
        );
        assert_eq!(
            try_inflate(&RectI64::of(i64::MIN + 5, i64::MIN, i64::MIN + 19, i64::MIN + 15)),
            Some(RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 20, i64::MIN + 17))
        );
        assert_eq!(
            try_inflate(&RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 20, i64::MIN + 17)),
            Some(RectI64::of(i64::MIN + 3, i64::MIN, i64::MIN + 21, i64::MIN + 19))
        );
        assert_eq!(
            try_inflate(&RectI64::of(i64::MIN + 3, i64::MIN, i64::MIN + 21, i64::MIN + 19)),
            Some(RectI64::of(i64::MIN + 2, i64::MIN, i64::MIN + 22, i64::MIN + 21))
        );
        assert_eq!(
            try_inflate(&RectI64::of(i64::MIN + 2, i64::MIN, i64::MIN + 22, i64::MIN + 21)),
            Some(RectI64::of(i64::MIN + 1, i64::MIN, i64::MIN + 23, i64::MIN + 23))
        );
        assert_eq!(
            try_inflate(&RectI64::of(i64::MIN + 1, i64::MIN, i64::MIN + 23, i64::MIN + 23)),
            Some(RectI64::of(i64::MIN, i64::MIN, i64::MIN + 24, i64::MIN + 25))
        );
        assert_eq!(
            try_inflate(&RectI64::of(i64::MIN, i64::MIN, i64::MIN + 24, i64::MIN + 25)),
            Some(RectI64::of(i64::MIN, i64::MIN, i64::MIN + 26, i64::MIN + 27))
        );
    }

    #[test]
    fn try_inflate_max_bounds() {
        assert_eq!(
            try_inflate(&RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3)),
            Some(RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2))
        );
        assert_eq!(
            try_inflate(&RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2)),
            Some(RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1))
        );
        assert_eq!(
            try_inflate(&RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1)),
            Some(RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX))
        );
        assert_eq!(
            try_inflate(&RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX)),
            Some(RectI64::of(i64::MAX - 37, i64::MAX - 22, i64::MAX - 1, i64::MAX))
        );
        assert_eq!(
            try_inflate(&RectI64::of(i64::MAX - 37, i64::MAX - 22, i64::MAX - 1, i64::MAX)),
            Some(RectI64::of(i64::MAX - 38, i64::MAX - 24, i64::MAX, i64::MAX))
        );
        assert_eq!(
            try_inflate(&RectI64::of(i64::MAX - 38, i64::MAX - 24, i64::MAX, i64::MAX)),
            Some(RectI64::of(i64::MAX - 40, i64::MAX - 26, i64::MAX, i64::MAX))
        );
        assert_eq!(
            try_inflate(&RectI64::of(i64::MAX - 40, i64::MAX - 26, i64::MAX, i64::MAX)),
            Some(RectI64::of(i64::MAX - 42, i64::MAX - 28, i64::MAX, i64::MAX))
        );
        assert_eq!(
            try_inflate(&RectI64::of(i64::MAX - 42, i64::MAX - 28, i64::MAX, i64::MAX)),
            Some(RectI64::of(i64::MAX - 44, i64::MAX - 30, i64::MAX, i64::MAX))
        );
    }

    #[test]
    fn try_inflate_to_bounds() {
        assert_eq!(try_inflate(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1)), Some(RectI64::largest()));
        assert_eq!(try_inflate(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1)), Some(RectI64::largest()));
        assert_eq!(try_inflate(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX)), Some(RectI64::largest()));
    }

    #[test]
    fn try_inflate_width_to_bounds() {
        assert_eq!(
            try_inflate(&RectI64::of(i64::MIN + 1, i64::MIN + 10, i64::MIN + 20, i64::MIN + 20)),
            Some(RectI64::of(i64::MIN, i64::MIN + 9, i64::MIN + 21, i64::MIN + 21))
        );
        assert_eq!(
            try_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 1, i64::MIN + 20)),
            Some(RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MAX, i64::MIN + 21))
        );
    }

    #[test]
    fn try_inflate_height_to_bounds() {
        assert_eq!(
            try_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 1, i64::MIN + 20, i64::MIN + 20)),
            Some(RectI64::of(i64::MIN + 9, i64::MIN, i64::MIN + 21, i64::MIN + 21))
        );
        assert_eq!(
            try_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MIN + 20, i64::MAX - 1)),
            Some(RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MIN + 21, i64::MAX))
        );
    }

    #[test]
    fn try_inflate_out_of_bounds() {
        assert_eq!(try_inflate(&RectI64::largest()), None);
        assert_eq!(try_inflate(&RectI64::of(i64::MIN, i64::MIN + 10, i64::MAX, i64::MIN + 20)), None);
        assert_eq!(try_inflate(&RectI64::of(i64::MIN + 10, i64::MIN, i64::MIN + 20, i64::MAX)), None);
    }

    #[test]
    fn assign_inflate_min_bounds() {
        let mut r = RectI64::of(i64::MIN + 7, i64::MIN + 2, i64::MIN + 17, i64::MIN + 13);
        assign_inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 6, i64::MIN + 1, i64::MIN + 18, i64::MIN + 14));
        assign_inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 5, i64::MIN, i64::MIN + 19, i64::MIN + 15));
        assign_inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 20, i64::MIN + 17));
        assign_inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 3, i64::MIN, i64::MIN + 21, i64::MIN + 19));
        assign_inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 2, i64::MIN, i64::MIN + 22, i64::MIN + 21));
        assign_inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 1, i64::MIN, i64::MIN + 23, i64::MIN + 23));
        assign_inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 24, i64::MIN + 25));
        assign_inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 26, i64::MIN + 27));
    }

    #[test]
    fn assign_inflate_max_bounds() {
        let mut r = RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3);
        assign_inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2));
        assign_inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1));
        assign_inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MAX - 37, i64::MAX - 22, i64::MAX - 1, i64::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MAX - 38, i64::MAX - 24, i64::MAX, i64::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MAX - 40, i64::MAX - 26, i64::MAX, i64::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MAX - 42, i64::MAX - 28, i64::MAX, i64::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MAX - 44, i64::MAX - 30, i64::MAX, i64::MAX));
    }

    #[test]
    fn inflate_min_bounds() {
        assert_eq!(
            inflate(&RectI64::of(i64::MIN + 7, i64::MIN + 2, i64::MIN + 17, i64::MIN + 13)),
            RectI64::of(i64::MIN + 6, i64::MIN + 1, i64::MIN + 18, i64::MIN + 14)
        );
        assert_eq!(
            inflate(&RectI64::of(i64::MIN + 6, i64::MIN + 1, i64::MIN + 18, i64::MIN + 14)),
            RectI64::of(i64::MIN + 5, i64::MIN, i64::MIN + 19, i64::MIN + 15)
        );
        assert_eq!(
            inflate(&RectI64::of(i64::MIN + 5, i64::MIN, i64::MIN + 19, i64::MIN + 15)),
            RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 20, i64::MIN + 17)
        );
        assert_eq!(
            inflate(&RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 20, i64::MIN + 17)),
            RectI64::of(i64::MIN + 3, i64::MIN, i64::MIN + 21, i64::MIN + 19)
        );
        assert_eq!(
            inflate(&RectI64::of(i64::MIN + 3, i64::MIN, i64::MIN + 21, i64::MIN + 19)),
            RectI64::of(i64::MIN + 2, i64::MIN, i64::MIN + 22, i64::MIN + 21)
        );
        assert_eq!(
            inflate(&RectI64::of(i64::MIN + 2, i64::MIN, i64::MIN + 22, i64::MIN + 21)),
            RectI64::of(i64::MIN + 1, i64::MIN, i64::MIN + 23, i64::MIN + 23)
        );
        assert_eq!(
            inflate(&RectI64::of(i64::MIN + 1, i64::MIN, i64::MIN + 23, i64::MIN + 23)),
            RectI64::of(i64::MIN, i64::MIN, i64::MIN + 24, i64::MIN + 25)
        );
        assert_eq!(
            inflate(&RectI64::of(i64::MIN, i64::MIN, i64::MIN + 24, i64::MIN + 25)),
            RectI64::of(i64::MIN, i64::MIN, i64::MIN + 26, i64::MIN + 27)
        );
    }

    #[test]
    fn inflate_max_bounds() {
        assert_eq!(
            inflate(&RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3)),
            RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2)
        );
        assert_eq!(
            inflate(&RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2)),
            RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1)
        );
        assert_eq!(
            inflate(&RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1)),
            RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX)
        );
        assert_eq!(
            inflate(&RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX)),
            RectI64::of(i64::MAX - 37, i64::MAX - 22, i64::MAX - 1, i64::MAX)
        );
        assert_eq!(
            inflate(&RectI64::of(i64::MAX - 37, i64::MAX - 22, i64::MAX - 1, i64::MAX)),
            RectI64::of(i64::MAX - 38, i64::MAX - 24, i64::MAX, i64::MAX)
        );
        assert_eq!(
            inflate(&RectI64::of(i64::MAX - 38, i64::MAX - 24, i64::MAX, i64::MAX)),
            RectI64::of(i64::MAX - 40, i64::MAX - 26, i64::MAX, i64::MAX)
        );
        assert_eq!(
            inflate(&RectI64::of(i64::MAX - 40, i64::MAX - 26, i64::MAX, i64::MAX)),
            RectI64::of(i64::MAX - 42, i64::MAX - 28, i64::MAX, i64::MAX)
        );
        assert_eq!(
            inflate(&RectI64::of(i64::MAX - 42, i64::MAX - 28, i64::MAX, i64::MAX)),
            RectI64::of(i64::MAX - 44, i64::MAX - 30, i64::MAX, i64::MAX)
        );
    }
}
