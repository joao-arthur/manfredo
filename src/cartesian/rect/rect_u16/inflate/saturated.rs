use crate::cartesian::{point::point_u16::PointU16, rect::rect_u16::RectU16};

pub fn try_assign_inflate(r: &mut RectU16) -> Option<()> {
    let is_min_x = r.min.x == 0;
    let is_min_y = r.min.y == 0;
    let is_max_x = r.max.x == u16::MAX;
    let is_max_y = r.max.y == u16::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - u16::from(is_min_x) + u16::from(is_max_x);
    let min_y_modifier = 1 - u16::from(is_min_y) + u16::from(is_max_y);
    let max_x_modifier = 1 + u16::from(is_min_x) - u16::from(is_max_x);
    let max_y_modifier = 1 + u16::from(is_min_y) - u16::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
    Some(())
}

pub fn try_inflate(r: &RectU16) -> Option<RectU16> {
    let is_min_x = r.min.x == 0;
    let is_min_y = r.min.y == 0;
    let is_max_x = r.max.x == u16::MAX;
    let is_max_y = r.max.y == u16::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - u16::from(is_min_x) + u16::from(is_max_x);
    let min_y_modifier = 1 - u16::from(is_min_y) + u16::from(is_max_y);
    let max_x_modifier = 1 + u16::from(is_min_x) - u16::from(is_max_x);
    let max_y_modifier = 1 + u16::from(is_min_y) - u16::from(is_max_y);
    let min_x = r.min.x.saturating_sub(min_x_modifier);
    let min_y = r.min.y.saturating_sub(min_y_modifier);
    let max_x = r.max.x.saturating_add(max_x_modifier);
    let max_y = r.max.y.saturating_add(max_y_modifier);
    Some(RectU16 { min: PointU16 { x: min_x, y: min_y }, max: PointU16 { x: max_x, y: max_y } })
}

pub fn assign_inflate(r: &mut RectU16) {
    try_assign_inflate(r).unwrap()
}

pub fn inflate(r: &RectU16) -> RectU16 {
    try_inflate(r).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_u16::RectU16;

    use super::{ assign_inflate, inflate, try_assign_inflate, try_inflate};

    #[test]
    fn try_assign_inflate_min_bounds() {
        let mut r = RectU16::of(7, 2, 17, 13);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(6, 1, 18, 14));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(5, 0, 19, 15));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(4, 0, 20, 17));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(3, 0, 21, 19));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(2, 0, 22, 21));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(1, 0, 23, 23));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(0, 0, 24, 25));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(0, 0, 26, 27));
    }

    #[test]
    fn try_assign_inflate_max_bounds() {
        let mut r = RectU16::of(u16::MAX - 33, u16::MAX - 17, u16::MAX - 5, u16::MAX - 3);
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(u16::MAX - 37, u16::MAX - 22, u16::MAX - 1, u16::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(u16::MAX - 38, u16::MAX - 24, u16::MAX, u16::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(u16::MAX - 40, u16::MAX - 26, u16::MAX, u16::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(u16::MAX - 42, u16::MAX - 28, u16::MAX, u16::MAX));
        assert_eq!(try_assign_inflate(&mut r), Some(()));
        assert_eq!(r, RectU16::of(u16::MAX - 44, u16::MAX - 30, u16::MAX, u16::MAX));
    }

    #[test]
    fn try_assign_inflate_to_bounds() {
        let mut r_odd = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_odd), Some(()));
        assert_eq!(r_odd, RectU16::largest());

        let mut r_even_1 = RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_even_1), Some(()));
        assert_eq!(r_even_1, RectU16::largest());

        let mut r_even_2 = RectU16::of(1, 1, u16::MAX, u16::MAX);
        assert_eq!(try_assign_inflate(&mut r_even_2), Some(()));
        assert_eq!(r_even_2, RectU16::largest());
    }

    #[test]
    fn try_assign_inflate_width_to_bounds() {
        let mut r_min = RectU16::of(1, 10, 20, 20);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectU16::of(0, 9, 21, 21));

        let mut r_max = RectU16::of(10, 10, u16::MAX - 1, 20);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectU16::of(9, 9, u16::MAX, 21));
    }

    #[test]
    fn try_assign_inflate_height_to_bounds() {
        let mut r_min = RectU16::of(10, 1, 20, 20);
        assert_eq!(try_assign_inflate(&mut r_min), Some(()));
        assert_eq!(r_min, RectU16::of(9, 0, 21, 21));

        let mut r_max = RectU16::of(10, 10, 20, u16::MAX - 1);
        assert_eq!(try_assign_inflate(&mut r_max), Some(()));
        assert_eq!(r_max, RectU16::of(9, 9, 21, u16::MAX));
    }

    #[test]
    fn try_assign_inflate_out_of_bounds() {
        let mut r = RectU16::largest();
        assert_eq!(try_assign_inflate(&mut r), None);
        assert_eq!(r, RectU16::largest());

        let mut r_x = RectU16::of(0, 10, u16::MAX, 20);
        assert_eq!(try_assign_inflate(&mut r_x), None);
        assert_eq!(r_x, RectU16::of(0, 10, u16::MAX, 20));

        let mut r_y = RectU16::of(10, 0, 20, u16::MAX);
        assert_eq!(try_assign_inflate(&mut r_y), None);
        assert_eq!(r_y, RectU16::of(10, 0, 20, u16::MAX));
    }

    #[test]
    fn try_inflate_min_bounds() {
        assert_eq!(try_inflate(&RectU16::of(7, 2, 17, 13)), Some(RectU16::of(6, 1, 18, 14)));
        assert_eq!(try_inflate(&RectU16::of(6, 1, 18, 14)), Some(RectU16::of(5, 0, 19, 15)));
        assert_eq!(try_inflate(&RectU16::of(5, 0, 19, 15)), Some(RectU16::of(4, 0, 20, 17)));
        assert_eq!(try_inflate(&RectU16::of(4, 0, 20, 17)), Some(RectU16::of(3, 0, 21, 19)));
        assert_eq!(try_inflate(&RectU16::of(3, 0, 21, 19)), Some(RectU16::of(2, 0, 22, 21)));
        assert_eq!(try_inflate(&RectU16::of(2, 0, 22, 21)), Some(RectU16::of(1, 0, 23, 23)));
        assert_eq!(try_inflate(&RectU16::of(1, 0, 23, 23)), Some(RectU16::of(0, 0, 24, 25)));
        assert_eq!(try_inflate(&RectU16::of(0, 0, 24, 25)), Some(RectU16::of(0, 0, 26, 27)));
    }

    #[test]
    fn try_inflate_max_bounds() {
        assert_eq!(
            try_inflate(&RectU16::of(u16::MAX - 33, u16::MAX - 17, u16::MAX - 5, u16::MAX - 3)),
            Some(RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2))
        );
        assert_eq!(
            try_inflate(&RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2)),
            Some(RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1))
        );
        assert_eq!(
            try_inflate(&RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1)),
            Some(RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX))
        );
        assert_eq!(
            try_inflate(&RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX)),
            Some(RectU16::of(u16::MAX - 37, u16::MAX - 22, u16::MAX - 1, u16::MAX))
        );
        assert_eq!(
            try_inflate(&RectU16::of(u16::MAX - 37, u16::MAX - 22, u16::MAX - 1, u16::MAX)),
            Some(RectU16::of(u16::MAX - 38, u16::MAX - 24, u16::MAX, u16::MAX))
        );
        assert_eq!(
            try_inflate(&RectU16::of(u16::MAX - 38, u16::MAX - 24, u16::MAX, u16::MAX)),
            Some(RectU16::of(u16::MAX - 40, u16::MAX - 26, u16::MAX, u16::MAX))
        );
        assert_eq!(
            try_inflate(&RectU16::of(u16::MAX - 40, u16::MAX - 26, u16::MAX, u16::MAX)),
            Some(RectU16::of(u16::MAX - 42, u16::MAX - 28, u16::MAX, u16::MAX))
        );
        assert_eq!(
            try_inflate(&RectU16::of(u16::MAX - 42, u16::MAX - 28, u16::MAX, u16::MAX)),
            Some(RectU16::of(u16::MAX - 44, u16::MAX - 30, u16::MAX, u16::MAX))
        );
    }

    #[test]
    fn try_inflate_to_bounds() {
        assert_eq!(try_inflate(&RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1)), Some(RectU16::largest()));
        assert_eq!(try_inflate(&RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1)), Some(RectU16::largest()));
        assert_eq!(try_inflate(&RectU16::of(1, 1, u16::MAX, u16::MAX)), Some(RectU16::largest()));
    }

    #[test]
    fn try_inflate_width_to_bounds() {
        assert_eq!(try_inflate(&RectU16::of(1, 10, 20, 20)), Some(RectU16::of(0, 9, 21, 21)));
        assert_eq!(try_inflate(&RectU16::of(10, 10, u16::MAX - 1, 20)), Some(RectU16::of(9, 9, u16::MAX, 21)));
    }

    #[test]
    fn try_inflate_height_to_bounds() {
        assert_eq!(try_inflate(&RectU16::of(10, 1, 20, 20)), Some(RectU16::of(9, 0, 21, 21)));
        assert_eq!(try_inflate(&RectU16::of(10, 10, 20, u16::MAX - 1)), Some(RectU16::of(9, 9, 21, u16::MAX)));
    }

    #[test]
    fn try_inflate_out_of_bounds() {
        assert_eq!(try_inflate(&RectU16::largest()), None);
        assert_eq!(try_inflate(&RectU16::of(0, 10, u16::MAX, 20)), None);
        assert_eq!(try_inflate(&RectU16::of(10, 0, 20, u16::MAX)), None);
    }

    #[test]
    fn assign_inflate_min_bounds() {
        let mut r = RectU16::of(7, 2, 17, 13);
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(6, 1, 18, 14));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(5, 0, 19, 15));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(4, 0, 20, 17));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(3, 0, 21, 19));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(2, 0, 22, 21));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(1, 0, 23, 23));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(0, 0, 24, 25));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(0, 0, 26, 27));
    }

    #[test]
    fn assign_inflate_max_bounds() {
        let mut r = RectU16::of(u16::MAX - 33, u16::MAX - 17, u16::MAX - 5, u16::MAX - 3);
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(u16::MAX - 37, u16::MAX - 22, u16::MAX - 1, u16::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(u16::MAX - 38, u16::MAX - 24, u16::MAX, u16::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(u16::MAX - 40, u16::MAX - 26, u16::MAX, u16::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(u16::MAX - 42, u16::MAX - 28, u16::MAX, u16::MAX));
        assign_inflate(&mut r);
        assert_eq!(r, RectU16::of(u16::MAX - 44, u16::MAX - 30, u16::MAX, u16::MAX));
    }

    #[test]
    fn inflate_min_bounds() {
        assert_eq!(inflate(&RectU16::of(7, 2, 17, 13)), RectU16::of(6, 1, 18, 14));
        assert_eq!(inflate(&RectU16::of(6, 1, 18, 14)), RectU16::of(5, 0, 19, 15));
        assert_eq!(inflate(&RectU16::of(5, 0, 19, 15)), RectU16::of(4, 0, 20, 17));
        assert_eq!(inflate(&RectU16::of(4, 0, 20, 17)), RectU16::of(3, 0, 21, 19));
        assert_eq!(inflate(&RectU16::of(3, 0, 21, 19)), RectU16::of(2, 0, 22, 21));
        assert_eq!(inflate(&RectU16::of(2, 0, 22, 21)), RectU16::of(1, 0, 23, 23));
        assert_eq!(inflate(&RectU16::of(1, 0, 23, 23)), RectU16::of(0, 0, 24, 25));
        assert_eq!(inflate(&RectU16::of(0, 0, 24, 25)), RectU16::of(0, 0, 26, 27));
    }

    #[test]
    fn inflate_max_bounds() {
        assert_eq!(
            inflate(&RectU16::of(u16::MAX - 33, u16::MAX - 17, u16::MAX - 5, u16::MAX - 3)),
            RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2)
        );
        assert_eq!(
            inflate(&RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2)),
            RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1)
        );
        assert_eq!(
            inflate(&RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1)),
            RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX)
        );
        assert_eq!(
            inflate(&RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX)),
            RectU16::of(u16::MAX - 37, u16::MAX - 22, u16::MAX - 1, u16::MAX)
        );
        assert_eq!(
            inflate(&RectU16::of(u16::MAX - 37, u16::MAX - 22, u16::MAX - 1, u16::MAX)),
            RectU16::of(u16::MAX - 38, u16::MAX - 24, u16::MAX, u16::MAX)
        );
        assert_eq!(
            inflate(&RectU16::of(u16::MAX - 38, u16::MAX - 24, u16::MAX, u16::MAX)),
            RectU16::of(u16::MAX - 40, u16::MAX - 26, u16::MAX, u16::MAX)
        );
        assert_eq!(
            inflate(&RectU16::of(u16::MAX - 40, u16::MAX - 26, u16::MAX, u16::MAX)),
            RectU16::of(u16::MAX - 42, u16::MAX - 28, u16::MAX, u16::MAX)
        );
        assert_eq!(
            inflate(&RectU16::of(u16::MAX - 42, u16::MAX - 28, u16::MAX, u16::MAX)),
            RectU16::of(u16::MAX - 44, u16::MAX - 30, u16::MAX, u16::MAX)
        );
    }
}
