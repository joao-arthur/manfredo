use crate::cartesian::{point::point_i32::PointI32, rect::rect_i32::RectI32};

pub fn try_assign_inflate(r: &mut RectI32) -> Option<()> {
    let is_min_x = r.min.x == i32::MIN;
    let is_min_y = r.min.y == i32::MIN;
    let is_max_x = r.max.x == i32::MAX;
    let is_max_y = r.max.y == i32::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - i32::from(is_min_x) + i32::from(is_max_x);
    let min_y_modifier = 1 - i32::from(is_min_y) + i32::from(is_max_y);
    let max_x_modifier = 1 + i32::from(is_min_x) - i32::from(is_max_x);
    let max_y_modifier = 1 + i32::from(is_min_y) - i32::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
    Some(())
}

pub fn try_inflate(r: &RectI32) -> Option<RectI32> {
    let is_min_x = r.min.x == i32::MIN;
    let is_min_y = r.min.y == i32::MIN;
    let is_max_x = r.max.x == i32::MAX;
    let is_max_y = r.max.y == i32::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - i32::from(is_min_x) + i32::from(is_max_x);
    let min_y_modifier = 1 - i32::from(is_min_y) + i32::from(is_max_y);
    let max_x_modifier = 1 + i32::from(is_min_x) - i32::from(is_max_x);
    let max_y_modifier = 1 + i32::from(is_min_y) - i32::from(is_max_y);
    let min_x = r.min.x.saturating_sub(min_x_modifier);
    let min_y = r.min.y.saturating_sub(min_y_modifier);
    let max_x = r.max.x.saturating_add(max_x_modifier);
    let max_y = r.max.y.saturating_add(max_y_modifier);
    Some(RectI32 { min: PointI32 { x: min_x, y: min_y }, max: PointI32 { x: max_x, y: max_y } })
}

pub fn assign_inflate(r: &mut RectI32) {
    try_assign_inflate(r).unwrap()
}

pub fn inflate(r: &RectI32) -> RectI32 {
    try_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_i32::RectI32;

    use super::{assign_inflate, inflate, try_assign_inflate, try_inflate};

    #[test]
    fn try_assign_inflate_min_bounds() {
        let mut r = RectI32::of(i32::MIN + 7, i32::MIN + 2, i32::MIN + 17, i32::MIN + 13);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MIN + 6, i32::MIN + 1, i32::MIN + 18, i32::MIN + 14));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MIN + 5, i32::MIN, i32::MIN + 19, i32::MIN + 15));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MIN + 4, i32::MIN, i32::MIN + 20, i32::MIN + 17));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MIN + 3, i32::MIN, i32::MIN + 21, i32::MIN + 19));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 22, i32::MIN + 21));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MIN + 1, i32::MIN, i32::MIN + 23, i32::MIN + 23));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 24, i32::MIN + 25));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 26, i32::MIN + 27));
    }

    #[test]
    fn try_assign_inflate_max_bounds() {
        let mut r = RectI32::of(i32::MAX - 33, i32::MAX - 17, i32::MAX - 5, i32::MAX - 3);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MAX - 34, i32::MAX - 18, i32::MAX - 4, i32::MAX - 2));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MAX - 35, i32::MAX - 19, i32::MAX - 3, i32::MAX - 1));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MAX - 36, i32::MAX - 20, i32::MAX - 2, i32::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MAX - 37, i32::MAX - 22, i32::MAX - 1, i32::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MAX - 38, i32::MAX - 24, i32::MAX, i32::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MAX - 40, i32::MAX - 26, i32::MAX, i32::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MAX - 42, i32::MAX - 28, i32::MAX, i32::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectI32::of(i32::MAX - 44, i32::MAX - 30, i32::MAX, i32::MAX));
    }

    #[test]
    fn try_assign_inflate_to_bounds() {
        let mut r_odd = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_odd), Some(()));
        assert_eq!(r_odd, RectI32::largest());

        let mut r_even_1 = RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_even_1), Some(()));
        assert_eq!(r_even_1, RectI32::largest());

        let mut r_even_2 = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX);
        assert_eq!(try_assign_inflate(&mut r_even_2), Some(()));
        assert_eq!(r_even_2, RectI32::largest());
    }

    #[test]
    fn try_assign_inflate_width_to_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 1, i32::MIN + 10, i32::MIN + 20, i32::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectI32::of(i32::MIN, i32::MIN + 9, i32::MIN + 21, i32::MIN + 21));

        let mut r_max = RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 1, i32::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectI32::of(i32::MIN + 9, i32::MIN + 9, i32::MAX, i32::MIN + 21));
    }

    #[test]
    fn try_assign_inflate_height_to_bounds() {
        let mut r_min = RectI32::of(i32::MIN + 10, i32::MIN + 1, i32::MIN + 20, i32::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectI32::of(i32::MIN + 9, i32::MIN, i32::MIN + 21, i32::MIN + 21));

        let mut r_max = RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MIN + 20, i32::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectI32::of(i32::MIN + 9, i32::MIN + 9, i32::MIN + 21, i32::MAX));
    }

    #[test]
    fn try_assign_inflate_out_of_bounds() {
        let mut r = RectI32::largest();
        assert_eq!(try_assign_inflate(&mut r), None);
        assert_eq!(r, RectI32::largest());

        let mut r_x = RectI32::of(i32::MIN, i32::MIN + 10, i32::MAX, i32::MIN + 20);
        assert_eq!(try_assign_inflate(&mut r_x), None);
        assert_eq!(r_x, RectI32::of(i32::MIN, i32::MIN + 10, i32::MAX, i32::MIN + 20));

        let mut r_y = RectI32::of(i32::MIN + 10, i32::MIN, i32::MIN + 20, i32::MAX);
        assert_eq!(try_assign_inflate(&mut r_y), None);
        assert_eq!(r_y, RectI32::of(i32::MIN + 10, i32::MIN, i32::MIN + 20, i32::MAX));
    }

    #[test]
    fn try_inflate_min_bounds() {
        assert_eq!(
            try_inflate(&RectI32::of(i32::MIN + 7, i32::MIN + 2, i32::MIN + 17, i32::MIN + 13)),
            Some(RectI32::of(i32::MIN + 6, i32::MIN + 1, i32::MIN + 18, i32::MIN + 14))
        );
        assert_eq!(
            try_inflate(&RectI32::of(i32::MIN + 6, i32::MIN + 1, i32::MIN + 18, i32::MIN + 14)),
            Some(RectI32::of(i32::MIN + 5, i32::MIN, i32::MIN + 19, i32::MIN + 15))
        );
        assert_eq!(
            try_inflate(&RectI32::of(i32::MIN + 5, i32::MIN, i32::MIN + 19, i32::MIN + 15)),
            Some(RectI32::of(i32::MIN + 4, i32::MIN, i32::MIN + 20, i32::MIN + 17))
        );
        assert_eq!(
            try_inflate(&RectI32::of(i32::MIN + 4, i32::MIN, i32::MIN + 20, i32::MIN + 17)),
            Some(RectI32::of(i32::MIN + 3, i32::MIN, i32::MIN + 21, i32::MIN + 19))
        );
        assert_eq!(
            try_inflate(&RectI32::of(i32::MIN + 3, i32::MIN, i32::MIN + 21, i32::MIN + 19)),
            Some(RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 22, i32::MIN + 21))
        );
        assert_eq!(
            try_inflate(&RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 22, i32::MIN + 21)),
            Some(RectI32::of(i32::MIN + 1, i32::MIN, i32::MIN + 23, i32::MIN + 23))
        );
        assert_eq!(
            try_inflate(&RectI32::of(i32::MIN + 1, i32::MIN, i32::MIN + 23, i32::MIN + 23)),
            Some(RectI32::of(i32::MIN, i32::MIN, i32::MIN + 24, i32::MIN + 25))
        );
        assert_eq!(
            try_inflate(&RectI32::of(i32::MIN, i32::MIN, i32::MIN + 24, i32::MIN + 25)),
            Some(RectI32::of(i32::MIN, i32::MIN, i32::MIN + 26, i32::MIN + 27))
        );
    }

    #[test]
    fn try_inflate_max_bounds() {
        assert_eq!(
            try_inflate(&RectI32::of(i32::MAX - 33, i32::MAX - 17, i32::MAX - 5, i32::MAX - 3)),
            Some(RectI32::of(i32::MAX - 34, i32::MAX - 18, i32::MAX - 4, i32::MAX - 2))
        );
        assert_eq!(
            try_inflate(&RectI32::of(i32::MAX - 34, i32::MAX - 18, i32::MAX - 4, i32::MAX - 2)),
            Some(RectI32::of(i32::MAX - 35, i32::MAX - 19, i32::MAX - 3, i32::MAX - 1))
        );
        assert_eq!(
            try_inflate(&RectI32::of(i32::MAX - 35, i32::MAX - 19, i32::MAX - 3, i32::MAX - 1)),
            Some(RectI32::of(i32::MAX - 36, i32::MAX - 20, i32::MAX - 2, i32::MAX))
        );
        assert_eq!(
            try_inflate(&RectI32::of(i32::MAX - 36, i32::MAX - 20, i32::MAX - 2, i32::MAX)),
            Some(RectI32::of(i32::MAX - 37, i32::MAX - 22, i32::MAX - 1, i32::MAX))
        );
        assert_eq!(
            try_inflate(&RectI32::of(i32::MAX - 37, i32::MAX - 22, i32::MAX - 1, i32::MAX)),
            Some(RectI32::of(i32::MAX - 38, i32::MAX - 24, i32::MAX, i32::MAX))
        );
        assert_eq!(
            try_inflate(&RectI32::of(i32::MAX - 38, i32::MAX - 24, i32::MAX, i32::MAX)),
            Some(RectI32::of(i32::MAX - 40, i32::MAX - 26, i32::MAX, i32::MAX))
        );
        assert_eq!(
            try_inflate(&RectI32::of(i32::MAX - 40, i32::MAX - 26, i32::MAX, i32::MAX)),
            Some(RectI32::of(i32::MAX - 42, i32::MAX - 28, i32::MAX, i32::MAX))
        );
        assert_eq!(
            try_inflate(&RectI32::of(i32::MAX - 42, i32::MAX - 28, i32::MAX, i32::MAX)),
            Some(RectI32::of(i32::MAX - 44, i32::MAX - 30, i32::MAX, i32::MAX))
        );
    }

    #[test]
    fn try_inflate_to_bounds() {
        assert_eq!(try_inflate(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1)), Some(RectI32::largest()));
        assert_eq!(try_inflate(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 1)), Some(RectI32::largest()));
        assert_eq!(try_inflate(&RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX, i32::MAX)), Some(RectI32::largest()));
    }

    #[test]
    fn try_inflate_width_to_bounds() {
        assert_eq!(
            try_inflate(&RectI32::of(i32::MIN + 1, i32::MIN + 10, i32::MIN + 20, i32::MIN + 20)),
            Some(RectI32::of(i32::MIN, i32::MIN + 9, i32::MIN + 21, i32::MIN + 21))
        );
        assert_eq!(
            try_inflate(&RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 1, i32::MIN + 20)),
            Some(RectI32::of(i32::MIN + 9, i32::MIN + 9, i32::MAX, i32::MIN + 21))
        );
    }

    #[test]
    fn try_inflate_height_to_bounds() {
        assert_eq!(
            try_inflate(&RectI32::of(i32::MIN + 10, i32::MIN + 1, i32::MIN + 20, i32::MIN + 20)),
            Some(RectI32::of(i32::MIN + 9, i32::MIN, i32::MIN + 21, i32::MIN + 21))
        );
        assert_eq!(
            try_inflate(&RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MIN + 20, i32::MAX - 1)),
            Some(RectI32::of(i32::MIN + 9, i32::MIN + 9, i32::MIN + 21, i32::MAX))
        );
    }

    #[test]
    fn try_inflate_out_of_bounds() {
        assert_eq!(try_inflate(&RectI32::largest()), None);
        assert_eq!(try_inflate(&RectI32::of(i32::MIN, i32::MIN + 10, i32::MAX, i32::MIN + 20)), None);
        assert_eq!(try_inflate(&RectI32::of(i32::MIN + 10, i32::MIN, i32::MIN + 20, i32::MAX)), None);
    }

    #[test]
    fn assign_inflate_min_bounds() {
        let mut r = RectI32::of(i32::MIN + 7, i32::MIN + 2, i32::MIN + 17, i32::MIN + 13);
        assign_inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN + 6, i32::MIN + 1, i32::MIN + 18, i32::MIN + 14));
        assign_inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN + 5, i32::MIN, i32::MIN + 19, i32::MIN + 15));
        assign_inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN + 4, i32::MIN, i32::MIN + 20, i32::MIN + 17));
        assign_inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN + 3, i32::MIN, i32::MIN + 21, i32::MIN + 19));
        assign_inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 22, i32::MIN + 21));
        assign_inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN + 1, i32::MIN, i32::MIN + 23, i32::MIN + 23));
        assign_inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 24, i32::MIN + 25));
        assign_inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MIN, i32::MIN, i32::MIN + 26, i32::MIN + 27));
    }

    #[test]
    fn assign_inflate_max_bounds() {
        let mut r = RectI32::of(i32::MAX - 33, i32::MAX - 17, i32::MAX - 5, i32::MAX - 3);
        assign_inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MAX - 34, i32::MAX - 18, i32::MAX - 4, i32::MAX - 2));
        assign_inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MAX - 35, i32::MAX - 19, i32::MAX - 3, i32::MAX - 1));
        assign_inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MAX - 36, i32::MAX - 20, i32::MAX - 2, i32::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MAX - 37, i32::MAX - 22, i32::MAX - 1, i32::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MAX - 38, i32::MAX - 24, i32::MAX, i32::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MAX - 40, i32::MAX - 26, i32::MAX, i32::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MAX - 42, i32::MAX - 28, i32::MAX, i32::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectI32::of(i32::MAX - 44, i32::MAX - 30, i32::MAX, i32::MAX));
    }

    #[test]
    fn inflate_min_bounds() {
        assert_eq!(
            inflate(&RectI32::of(i32::MIN + 7, i32::MIN + 2, i32::MIN + 17, i32::MIN + 13)),
            RectI32::of(i32::MIN + 6, i32::MIN + 1, i32::MIN + 18, i32::MIN + 14)
        );
        assert_eq!(
            inflate(&RectI32::of(i32::MIN + 6, i32::MIN + 1, i32::MIN + 18, i32::MIN + 14)),
            RectI32::of(i32::MIN + 5, i32::MIN, i32::MIN + 19, i32::MIN + 15)
        );
        assert_eq!(
            inflate(&RectI32::of(i32::MIN + 5, i32::MIN, i32::MIN + 19, i32::MIN + 15)),
            RectI32::of(i32::MIN + 4, i32::MIN, i32::MIN + 20, i32::MIN + 17)
        );
        assert_eq!(
            inflate(&RectI32::of(i32::MIN + 4, i32::MIN, i32::MIN + 20, i32::MIN + 17)),
            RectI32::of(i32::MIN + 3, i32::MIN, i32::MIN + 21, i32::MIN + 19)
        );
        assert_eq!(
            inflate(&RectI32::of(i32::MIN + 3, i32::MIN, i32::MIN + 21, i32::MIN + 19)),
            RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 22, i32::MIN + 21)
        );
        assert_eq!(
            inflate(&RectI32::of(i32::MIN + 2, i32::MIN, i32::MIN + 22, i32::MIN + 21)),
            RectI32::of(i32::MIN + 1, i32::MIN, i32::MIN + 23, i32::MIN + 23)
        );
        assert_eq!(
            inflate(&RectI32::of(i32::MIN + 1, i32::MIN, i32::MIN + 23, i32::MIN + 23)),
            RectI32::of(i32::MIN, i32::MIN, i32::MIN + 24, i32::MIN + 25)
        );
        assert_eq!(
            inflate(&RectI32::of(i32::MIN, i32::MIN, i32::MIN + 24, i32::MIN + 25)),
            RectI32::of(i32::MIN, i32::MIN, i32::MIN + 26, i32::MIN + 27)
        );
    }

    #[test]
    fn inflate_max_bounds() {
        assert_eq!(
            inflate(&RectI32::of(i32::MAX - 33, i32::MAX - 17, i32::MAX - 5, i32::MAX - 3)),
            RectI32::of(i32::MAX - 34, i32::MAX - 18, i32::MAX - 4, i32::MAX - 2)
        );
        assert_eq!(
            inflate(&RectI32::of(i32::MAX - 34, i32::MAX - 18, i32::MAX - 4, i32::MAX - 2)),
            RectI32::of(i32::MAX - 35, i32::MAX - 19, i32::MAX - 3, i32::MAX - 1)
        );
        assert_eq!(
            inflate(&RectI32::of(i32::MAX - 35, i32::MAX - 19, i32::MAX - 3, i32::MAX - 1)),
            RectI32::of(i32::MAX - 36, i32::MAX - 20, i32::MAX - 2, i32::MAX)
        );
        assert_eq!(
            inflate(&RectI32::of(i32::MAX - 36, i32::MAX - 20, i32::MAX - 2, i32::MAX)),
            RectI32::of(i32::MAX - 37, i32::MAX - 22, i32::MAX - 1, i32::MAX)
        );
        assert_eq!(
            inflate(&RectI32::of(i32::MAX - 37, i32::MAX - 22, i32::MAX - 1, i32::MAX)),
            RectI32::of(i32::MAX - 38, i32::MAX - 24, i32::MAX, i32::MAX)
        );
        assert_eq!(
            inflate(&RectI32::of(i32::MAX - 38, i32::MAX - 24, i32::MAX, i32::MAX)),
            RectI32::of(i32::MAX - 40, i32::MAX - 26, i32::MAX, i32::MAX)
        );
        assert_eq!(
            inflate(&RectI32::of(i32::MAX - 40, i32::MAX - 26, i32::MAX, i32::MAX)),
            RectI32::of(i32::MAX - 42, i32::MAX - 28, i32::MAX, i32::MAX)
        );
        assert_eq!(
            inflate(&RectI32::of(i32::MAX - 42, i32::MAX - 28, i32::MAX, i32::MAX)),
            RectI32::of(i32::MAX - 44, i32::MAX - 30, i32::MAX, i32::MAX)
        );
    }
}
