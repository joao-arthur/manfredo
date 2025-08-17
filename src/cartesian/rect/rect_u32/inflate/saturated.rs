use crate::cartesian::{point::point_u32::PointU32, rect::rect_u32::RectU32};

pub fn try_assign_inflate_in_bounds(r: &mut RectU32) -> Option<()> {
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

pub fn try_inflate_in_bounds(r: &RectU32) -> Option<RectU32> {
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
    Some(RectU32 {
        min: PointU32 { x: r.min.x.saturating_sub(min_x_modifier), y: r.min.y.saturating_sub(min_y_modifier) },
        max: PointU32 { x: r.max.x.saturating_add(max_x_modifier), y: r.max.y.saturating_add(max_y_modifier) },
    })
}

pub fn assign_inflate_in_bounds(r: &mut RectU32) {
    try_assign_inflate_in_bounds(r).unwrap()
}

pub fn inflate_in_bounds(r: &RectU32) -> RectU32 {
    try_inflate_in_bounds(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{RectU32, assign_inflate_in_bounds, inflate_in_bounds, try_assign_inflate_in_bounds, try_inflate_in_bounds};

    #[test]
    fn try_assign_inflate_in_bounds_min_bounds() {
        let mut r = RectU32::of(7, 2, 4, 13);
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU32::of(6, 1, 5, 14));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU32::of(5, 0, 6, 15));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU32::of(4, 0, 7, 17));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU32::of(3, 0, 8, 19));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU32::of(2, 0, 9, 21));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU32::of(1, 0, 10, 23));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU32::of(0, 0, 11, 25));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU32::of(0, 0, 13, 27));
    }

    #[test]
    fn try_assign_inflate_in_bounds_max_bounds() {
        let mut r = RectU32::of(200, 230, u32::MAX - 5, u32::MAX - 3);
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU32::of(199, 229, u32::MAX - 4, u32::MAX - 2));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU32::of(198, 228, u32::MAX - 3, u32::MAX - 1));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU32::of(197, 227, u32::MAX - 2, u32::MAX));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU32::of(196, 225, u32::MAX - 1, u32::MAX));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU32::of(195, 223, u32::MAX, u32::MAX));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU32::of(193, 221, u32::MAX, u32::MAX));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU32::of(191, 219, u32::MAX, u32::MAX));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU32::of(189, 217, u32::MAX, u32::MAX));
    }

    #[test]
    fn try_assign_inflate_in_bounds_to_bounds() {
        let mut r_odd = RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1);
        assert_eq!(try_assign_inflate_in_bounds(&mut r_odd), Some(()));
        assert_eq!(r_odd, RectU32::of(0, 0, u32::MAX, u32::MAX));

        let mut r_even_1 = RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1);
        assert_eq!(try_assign_inflate_in_bounds(&mut r_even_1), Some(()));
        assert_eq!(r_even_1, RectU32::of(0, 0, u32::MAX, u32::MAX));

        let mut r_even_2 = RectU32::of(1, 1, u32::MAX, u32::MAX);
        assert_eq!(try_assign_inflate_in_bounds(&mut r_even_2), Some(()));
        assert_eq!(r_even_2, RectU32::of(0, 0, u32::MAX, u32::MAX));
    }

    #[test]
    fn try_assign_inflate_in_bounds_width_to_bounds() {
        let mut r_min = RectU32::of(1, 10, 100, 100);
        assert_eq!(try_assign_inflate_in_bounds(&mut r_min), Some(()));
        assert_eq!(r_min, RectU32::of(0, 9, 101, 101));

        let mut r_max = RectU32::of(10, 10, u32::MAX - 1, 100);
        assert_eq!(try_assign_inflate_in_bounds(&mut r_max), Some(()));
        assert_eq!(r_max, RectU32::of(9, 9, u32::MAX, 101));
    }

    #[test]
    fn try_assign_inflate_in_bounds_height_to_bounds() {
        let mut r_min = RectU32::of(10, 1, 100, 100);
        assert_eq!(try_assign_inflate_in_bounds(&mut r_min), Some(()));
        assert_eq!(r_min, RectU32::of(9, 0, 101, 101));

        let mut r_max = RectU32::of(10, 10, 100, u32::MAX - 1);
        assert_eq!(try_assign_inflate_in_bounds(&mut r_max), Some(()));
        assert_eq!(r_max, RectU32::of(9, 9, 101, u32::MAX));
    }

    #[test]
    fn try_assign_inflate_in_bounds_out_of_bounds() {
        let mut r = RectU32::of(0, 0, u32::MAX, u32::MAX);
        assert_eq!(try_assign_inflate_in_bounds(&mut r), None);
        assert_eq!(r, RectU32::of(0, 0, u32::MAX, u32::MAX));

        let mut r_x = RectU32::of(0, 10, u32::MAX, 100);
        assert_eq!(try_assign_inflate_in_bounds(&mut r_x), None);
        assert_eq!(r_x, RectU32::of(0, 10, u32::MAX, 100));

        let mut r_y = RectU32::of(10, 0, 100, u32::MAX);
        assert_eq!(try_assign_inflate_in_bounds(&mut r_y), None);
        assert_eq!(r_y, RectU32::of(10, 0, 100, u32::MAX));
    }

    #[test]
    fn try_inflate_in_bounds_min_bounds() {
        assert_eq!(try_inflate_in_bounds(&RectU32::of(7, 2, 4, 13)), Some(RectU32::of(6, 1, 5, 14)));
        assert_eq!(try_inflate_in_bounds(&RectU32::of(6, 1, 5, 14)), Some(RectU32::of(5, 0, 6, 15)));
        assert_eq!(try_inflate_in_bounds(&RectU32::of(5, 0, 6, 15)), Some(RectU32::of(4, 0, 7, 17)));
        assert_eq!(try_inflate_in_bounds(&RectU32::of(4, 0, 7, 17)), Some(RectU32::of(3, 0, 8, 19)));
        assert_eq!(try_inflate_in_bounds(&RectU32::of(3, 0, 8, 19)), Some(RectU32::of(2, 0, 9, 21)));
        assert_eq!(try_inflate_in_bounds(&RectU32::of(2, 0, 9, 21)), Some(RectU32::of(1, 0, 10, 23)));
        assert_eq!(try_inflate_in_bounds(&RectU32::of(1, 0, 10, 23)), Some(RectU32::of(0, 0, 11, 25)));
        assert_eq!(try_inflate_in_bounds(&RectU32::of(0, 0, 11, 25)), Some(RectU32::of(0, 0, 13, 27)));
    }

    #[test]
    fn try_inflate_in_bounds_max_bounds() {
        assert_eq!(
            try_inflate_in_bounds(&RectU32::of(200, 230, u32::MAX - 5, u32::MAX - 3)),
            Some(RectU32::of(199, 229, u32::MAX - 4, u32::MAX - 2))
        );
        assert_eq!(
            try_inflate_in_bounds(&RectU32::of(199, 229, u32::MAX - 4, u32::MAX - 2)),
            Some(RectU32::of(198, 228, u32::MAX - 3, u32::MAX - 1))
        );
        assert_eq!(try_inflate_in_bounds(&RectU32::of(198, 228, u32::MAX - 3, u32::MAX - 1)), Some(RectU32::of(197, 227, u32::MAX - 2, u32::MAX)));
        assert_eq!(try_inflate_in_bounds(&RectU32::of(197, 227, u32::MAX - 2, u32::MAX)), Some(RectU32::of(196, 225, u32::MAX - 1, u32::MAX)));
        assert_eq!(try_inflate_in_bounds(&RectU32::of(196, 225, u32::MAX - 1, u32::MAX)), Some(RectU32::of(195, 223, u32::MAX, u32::MAX)));
        assert_eq!(try_inflate_in_bounds(&RectU32::of(195, 223, u32::MAX, u32::MAX)), Some(RectU32::of(193, 221, u32::MAX, u32::MAX)));
        assert_eq!(try_inflate_in_bounds(&RectU32::of(193, 221, u32::MAX, u32::MAX)), Some(RectU32::of(191, 219, u32::MAX, u32::MAX)));
        assert_eq!(try_inflate_in_bounds(&RectU32::of(191, 219, u32::MAX, u32::MAX)), Some(RectU32::of(189, 217, u32::MAX, u32::MAX)));
    }

    #[test]
    fn try_inflate_in_bounds_to_bounds() {
        assert_eq!(try_inflate_in_bounds(&RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1)), Some(RectU32::of(0, 0, u32::MAX, u32::MAX)));
        assert_eq!(try_inflate_in_bounds(&RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1)), Some(RectU32::of(0, 0, u32::MAX, u32::MAX)));
        assert_eq!(try_inflate_in_bounds(&RectU32::of(1, 1, u32::MAX, u32::MAX)), Some(RectU32::of(0, 0, u32::MAX, u32::MAX)));
    }

    #[test]
    fn try_inflate_in_bounds_width_to_bounds() {
        assert_eq!(try_inflate_in_bounds(&RectU32::of(1, 10, 100, 100)), Some(RectU32::of(0, 9, 101, 101)));
        assert_eq!(try_inflate_in_bounds(&RectU32::of(10, 10, u32::MAX - 1, 100)), Some(RectU32::of(9, 9, u32::MAX, 101)));
    }

    #[test]
    fn try_inflate_in_bounds_height_to_bounds() {
        assert_eq!(try_inflate_in_bounds(&RectU32::of(10, 1, 100, 100)), Some(RectU32::of(9, 0, 101, 101)));
        assert_eq!(try_inflate_in_bounds(&RectU32::of(10, 10, 100, u32::MAX - 1)), Some(RectU32::of(9, 9, 101, u32::MAX)));
    }

    #[test]
    fn try_inflate_in_bounds_out_of_bounds() {
        assert_eq!(try_inflate_in_bounds(&RectU32::of(0, 0, u32::MAX, u32::MAX)), None);
        assert_eq!(try_inflate_in_bounds(&RectU32::of(0, 10, u32::MAX, 100)), None);
        assert_eq!(try_inflate_in_bounds(&RectU32::of(10, 0, 100, u32::MAX)), None);
    }

    #[test]
    fn assign_inflate_in_bounds_min_bounds() {
        let mut r = RectU32::of(7, 2, 4, 13);
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU32::of(6, 1, 5, 14));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU32::of(5, 0, 6, 15));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU32::of(4, 0, 7, 17));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU32::of(3, 0, 8, 19));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU32::of(2, 0, 9, 21));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU32::of(1, 0, 10, 23));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU32::of(0, 0, 11, 25));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU32::of(0, 0, 13, 27));
    }

    #[test]
    fn assign_inflate_in_bounds_max_bounds() {
        let mut r = RectU32::of(200, 230, u32::MAX - 5, u32::MAX - 3);
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU32::of(199, 229, u32::MAX - 4, u32::MAX - 2));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU32::of(198, 228, u32::MAX - 3, u32::MAX - 1));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU32::of(197, 227, u32::MAX - 2, u32::MAX));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU32::of(196, 225, u32::MAX - 1, u32::MAX));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU32::of(195, 223, u32::MAX, u32::MAX));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU32::of(193, 221, u32::MAX, u32::MAX));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU32::of(191, 219, u32::MAX, u32::MAX));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU32::of(189, 217, u32::MAX, u32::MAX));
    }

    #[test]
    fn inflate_in_bounds_min_bounds() {
        assert_eq!(inflate_in_bounds(&RectU32::of(7, 2, 4, 13)), RectU32::of(6, 1, 5, 14));
        assert_eq!(inflate_in_bounds(&RectU32::of(6, 1, 5, 14)), RectU32::of(5, 0, 6, 15));
        assert_eq!(inflate_in_bounds(&RectU32::of(5, 0, 6, 15)), RectU32::of(4, 0, 7, 17));
        assert_eq!(inflate_in_bounds(&RectU32::of(4, 0, 7, 17)), RectU32::of(3, 0, 8, 19));
        assert_eq!(inflate_in_bounds(&RectU32::of(3, 0, 8, 19)), RectU32::of(2, 0, 9, 21));
        assert_eq!(inflate_in_bounds(&RectU32::of(2, 0, 9, 21)), RectU32::of(1, 0, 10, 23));
        assert_eq!(inflate_in_bounds(&RectU32::of(1, 0, 10, 23)), RectU32::of(0, 0, 11, 25));
        assert_eq!(inflate_in_bounds(&RectU32::of(0, 0, 11, 25)), RectU32::of(0, 0, 13, 27));
    }

    #[test]
    fn inflate_in_bounds_max_bounds() {
        assert_eq!(inflate_in_bounds(&RectU32::of(200, 230, u32::MAX - 5, u32::MAX - 3)), RectU32::of(199, 229, u32::MAX - 4, u32::MAX - 2));
        assert_eq!(inflate_in_bounds(&RectU32::of(199, 229, u32::MAX - 4, u32::MAX - 2)), RectU32::of(198, 228, u32::MAX - 3, u32::MAX - 1));
        assert_eq!(inflate_in_bounds(&RectU32::of(198, 228, u32::MAX - 3, u32::MAX - 1)), RectU32::of(197, 227, u32::MAX - 2, u32::MAX));
        assert_eq!(inflate_in_bounds(&RectU32::of(197, 227, u32::MAX - 2, u32::MAX)), RectU32::of(196, 225, u32::MAX - 1, u32::MAX));
        assert_eq!(inflate_in_bounds(&RectU32::of(196, 225, u32::MAX - 1, u32::MAX)), RectU32::of(195, 223, u32::MAX, u32::MAX));
        assert_eq!(inflate_in_bounds(&RectU32::of(195, 223, u32::MAX, u32::MAX)), RectU32::of(193, 221, u32::MAX, u32::MAX));
        assert_eq!(inflate_in_bounds(&RectU32::of(193, 221, u32::MAX, u32::MAX)), RectU32::of(191, 219, u32::MAX, u32::MAX));
        assert_eq!(inflate_in_bounds(&RectU32::of(191, 219, u32::MAX, u32::MAX)), RectU32::of(189, 217, u32::MAX, u32::MAX));
    }
}
