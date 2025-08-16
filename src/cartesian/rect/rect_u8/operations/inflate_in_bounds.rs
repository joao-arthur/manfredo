use crate::cartesian::{
    point::point_u8,
    rect::rect_u8::{RectU8, delta_x, delta_y},
};

pub fn try_assign_inflate_in_bounds(r: &mut RectU8) -> Option<()> {
    let is_min_x = r.min.x == 0;
    let is_min_y = r.min.y == 0;
    let is_max_x = r.max.x == u8::MAX;
    let is_max_y = r.max.y == u8::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - u8::from(is_min_x) + u8::from(is_max_x);
    let min_y_modifier = 1 - u8::from(is_min_y) + u8::from(is_max_y);
    let max_x_modifier = 1 + u8::from(is_min_x) - u8::from(is_max_x);
    let max_y_modifier = 1 + u8::from(is_min_y) - u8::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
    Some(())
}

pub fn try_inflate_in_bounds(r: &RectU8) -> Option<RectU8> {
    let is_min_x = r.min.x == 0;
    let is_min_y = r.min.y == 0;
    let is_max_x = r.max.x == u8::MAX;
    let is_max_y = r.max.y == u8::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return None;
    }
    let min_x_modifier = 1 - u8::from(is_min_x) + u8::from(is_max_x);
    let min_y_modifier = 1 - u8::from(is_min_y) + u8::from(is_max_y);
    let max_x_modifier = 1 + u8::from(is_min_x) - u8::from(is_max_x);
    let max_y_modifier = 1 + u8::from(is_min_y) - u8::from(is_max_y);
    Some(RectU8 {
        min: point_u8::PointU8 { x: r.min.x.saturating_sub(min_x_modifier), y: r.min.y.saturating_sub(min_y_modifier) },
        max: point_u8::PointU8 { x: r.max.x.saturating_add(max_x_modifier), y: r.max.y.saturating_add(max_y_modifier) },
    })
}

pub fn assign_inflate_in_bounds(r: &mut RectU8) {
    try_assign_inflate_in_bounds(r).unwrap()
}

pub fn inflate_in_bounds(r: &RectU8) -> RectU8 {
    try_inflate_in_bounds(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{RectU8, assign_inflate_in_bounds, inflate_in_bounds, try_assign_inflate_in_bounds, try_inflate_in_bounds};

    #[test]
    fn try_assign_inflate_in_bounds_min_bounds() {
        let mut r = RectU8::of(7, 2, 4, 13);
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU8::of(6, 1, 5, 14));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU8::of(5, 0, 6, 15));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU8::of(4, 0, 7, 17));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU8::of(3, 0, 8, 19));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU8::of(2, 0, 9, 21));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU8::of(1, 0, 10, 23));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU8::of(0, 0, 11, 25));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU8::of(0, 0, 13, 27));
    }

    #[test]
    fn try_assign_inflate_in_bounds_max_bounds() {
        let mut r = RectU8::of(200, 230, u8::MAX - 5, u8::MAX - 3);
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU8::of(199, 229, u8::MAX - 4, u8::MAX - 2));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU8::of(198, 228, u8::MAX - 3, u8::MAX - 1));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU8::of(197, 227, u8::MAX - 2, u8::MAX));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU8::of(196, 225, u8::MAX - 1, u8::MAX));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU8::of(195, 223, u8::MAX, u8::MAX));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU8::of(193, 221, u8::MAX, u8::MAX));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU8::of(191, 219, u8::MAX, u8::MAX));
        assert_eq!(try_assign_inflate_in_bounds(&mut r), Some(()));
        assert_eq!(r, RectU8::of(189, 217, u8::MAX, u8::MAX));
    }

    #[test]
    fn try_assign_inflate_in_bounds_to_bounds() {
        let mut r_odd = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert_eq!(try_assign_inflate_in_bounds(&mut r_odd), Some(()));
        assert_eq!(r_odd, RectU8::of(0, 0, u8::MAX, u8::MAX));

        let mut r_even_1 = RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1);
        assert_eq!(try_assign_inflate_in_bounds(&mut r_even_1), Some(()));
        assert_eq!(r_even_1, RectU8::of(0, 0, u8::MAX, u8::MAX));

        let mut r_even_2 = RectU8::of(1, 1, u8::MAX, u8::MAX);
        assert_eq!(try_assign_inflate_in_bounds(&mut r_even_2), Some(()));
        assert_eq!(r_even_2, RectU8::of(0, 0, u8::MAX, u8::MAX));
    }

    #[test]
    fn try_assign_inflate_in_bounds_width_to_bounds() {
        let mut r_min = RectU8::of(1, 10, 100, 100);
        assert_eq!(try_assign_inflate_in_bounds(&mut r_min), Some(()));
        assert_eq!(r_min, RectU8::of(0, 9, 101, 101));

        let mut r_max = RectU8::of(10, 10, u8::MAX - 1, 100);
        assert_eq!(try_assign_inflate_in_bounds(&mut r_max), Some(()));
        assert_eq!(r_max, RectU8::of(9, 9, u8::MAX, 101));
    }

    #[test]
    fn try_assign_inflate_in_bounds_height_to_bounds() {
        let mut r_min = RectU8::of(10, 1, 100, 100);
        assert_eq!(try_assign_inflate_in_bounds(&mut r_min), Some(()));
        assert_eq!(r_min, RectU8::of(9, 0, 101, 101));

        let mut r_max = RectU8::of(10, 10, 100, u8::MAX - 1);
        assert_eq!(try_assign_inflate_in_bounds(&mut r_max), Some(()));
        assert_eq!(r_max, RectU8::of(9, 9, 101, u8::MAX));
    }

    #[test]
    fn try_assign_inflate_in_bounds_out_of_bounds() {
        let mut r = RectU8::of(0, 0, u8::MAX, u8::MAX);
        assert_eq!(try_assign_inflate_in_bounds(&mut r), None);
        assert_eq!(r, RectU8::of(0, 0, u8::MAX, u8::MAX));

        let mut r_width = RectU8::of(0, 10, u8::MAX, 100);
        assert_eq!(try_assign_inflate_in_bounds(&mut r_width), None);
        assert_eq!(r_width, RectU8::of(0, 10, u8::MAX, 100));

        let mut r_height = RectU8::of(10, 0, 100, u8::MAX);
        assert_eq!(try_assign_inflate_in_bounds(&mut r_height), None);
        assert_eq!(r_height, RectU8::of(10, 0, 100, u8::MAX));
    }

    #[test]
    fn try_inflate_in_bounds_min_bounds() {
        assert_eq!(try_inflate_in_bounds(&RectU8::of(7, 2, 4, 13)), Some(RectU8::of(6, 1, 5, 14)));
        assert_eq!(try_inflate_in_bounds(&RectU8::of(6, 1, 5, 14)), Some(RectU8::of(5, 0, 6, 15)));
        assert_eq!(try_inflate_in_bounds(&RectU8::of(5, 0, 6, 15)), Some(RectU8::of(4, 0, 7, 17)));
        assert_eq!(try_inflate_in_bounds(&RectU8::of(4, 0, 7, 17)), Some(RectU8::of(3, 0, 8, 19)));
        assert_eq!(try_inflate_in_bounds(&RectU8::of(3, 0, 8, 19)), Some(RectU8::of(2, 0, 9, 21)));
        assert_eq!(try_inflate_in_bounds(&RectU8::of(2, 0, 9, 21)), Some(RectU8::of(1, 0, 10, 23)));
        assert_eq!(try_inflate_in_bounds(&RectU8::of(1, 0, 10, 23)), Some(RectU8::of(0, 0, 11, 25)));
        assert_eq!(try_inflate_in_bounds(&RectU8::of(0, 0, 11, 25)), Some(RectU8::of(0, 0, 13, 27)));
    }

    #[test]
    fn try_inflate_in_bounds_max_bounds() {
        assert_eq!(try_inflate_in_bounds(&RectU8::of(200, 230, u8::MAX - 5, u8::MAX - 3)), Some(RectU8::of(199, 229, u8::MAX - 4, u8::MAX - 2)));
        assert_eq!(try_inflate_in_bounds(&RectU8::of(199, 229, u8::MAX - 4, u8::MAX - 2)), Some(RectU8::of(198, 228, u8::MAX - 3, u8::MAX - 1)));
        assert_eq!(try_inflate_in_bounds(&RectU8::of(198, 228, u8::MAX - 3, u8::MAX - 1)), Some(RectU8::of(197, 227, u8::MAX - 2, u8::MAX)));
        assert_eq!(try_inflate_in_bounds(&RectU8::of(197, 227, u8::MAX - 2, u8::MAX)), Some(RectU8::of(196, 225, u8::MAX - 1, u8::MAX)));
        assert_eq!(try_inflate_in_bounds(&RectU8::of(196, 225, u8::MAX - 1, u8::MAX)), Some(RectU8::of(195, 223, u8::MAX, u8::MAX)));
        assert_eq!(try_inflate_in_bounds(&RectU8::of(195, 223, u8::MAX, u8::MAX)), Some(RectU8::of(193, 221, u8::MAX, u8::MAX)));
        assert_eq!(try_inflate_in_bounds(&RectU8::of(193, 221, u8::MAX, u8::MAX)), Some(RectU8::of(191, 219, u8::MAX, u8::MAX)));
        assert_eq!(try_inflate_in_bounds(&RectU8::of(191, 219, u8::MAX, u8::MAX)), Some(RectU8::of(189, 217, u8::MAX, u8::MAX)));
    }

    #[test]
    fn try_inflate_in_bounds_to_bounds() {
        assert_eq!(try_inflate_in_bounds(&RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1)), Some(RectU8::of(0, 0, u8::MAX, u8::MAX)));
        assert_eq!(try_inflate_in_bounds(&RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1)), Some(RectU8::of(0, 0, u8::MAX, u8::MAX)));
        assert_eq!(try_inflate_in_bounds(&RectU8::of(1, 1, u8::MAX, u8::MAX)), Some(RectU8::of(0, 0, u8::MAX, u8::MAX)));
    }

    #[test]
    fn try_inflate_in_bounds_width_to_bounds() {
        assert_eq!(try_inflate_in_bounds(&RectU8::of(1, 10, 100, 100)), Some(RectU8::of(0, 9, 101, 101)));
        assert_eq!(try_inflate_in_bounds(&RectU8::of(10, 10, u8::MAX - 1, 100)), Some(RectU8::of(9, 9, u8::MAX, 101)));
    }

    #[test]
    fn try_inflate_in_bounds_height_to_bounds() {
        assert_eq!(try_inflate_in_bounds(&RectU8::of(10, 1, 100, 100)), Some(RectU8::of(9, 0, 101, 101)));
        assert_eq!(try_inflate_in_bounds(&RectU8::of(10, 10, 100, u8::MAX - 1)), Some(RectU8::of(9, 9, 101, u8::MAX)));
    }

    #[test]
    fn try_inflate_in_bounds_out_of_bounds() {
        assert_eq!(try_inflate_in_bounds(&RectU8::of(0, 0, u8::MAX, u8::MAX)), None);
        assert_eq!(try_inflate_in_bounds(&RectU8::of(0, 10, u8::MAX, 100)), None);
        assert_eq!(try_inflate_in_bounds(&RectU8::of(10, 0, 100, u8::MAX)), None);
    }

    #[test]
    fn assign_inflate_in_bounds_min_bounds() {
        let mut r = RectU8::of(7, 2, 4, 13);
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU8::of(6, 1, 5, 14));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU8::of(5, 0, 6, 15));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU8::of(4, 0, 7, 17));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU8::of(3, 0, 8, 19));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU8::of(2, 0, 9, 21));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU8::of(1, 0, 10, 23));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU8::of(0, 0, 11, 25));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU8::of(0, 0, 13, 27));
    }

    #[test]
    fn assign_inflate_in_bounds_max_bounds() {
        let mut r = RectU8::of(200, 230, u8::MAX - 5, u8::MAX - 3);
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU8::of(199, 229, u8::MAX - 4, u8::MAX - 2));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU8::of(198, 228, u8::MAX - 3, u8::MAX - 1));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU8::of(197, 227, u8::MAX - 2, u8::MAX));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU8::of(196, 225, u8::MAX - 1, u8::MAX));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU8::of(195, 223, u8::MAX, u8::MAX));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU8::of(193, 221, u8::MAX, u8::MAX));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU8::of(191, 219, u8::MAX, u8::MAX));
        assign_inflate_in_bounds(&mut r);
        assert_eq!(r, RectU8::of(189, 217, u8::MAX, u8::MAX));
    }

    #[test]
    fn inflate_in_bounds_min_bounds() {
        assert_eq!(inflate_in_bounds(&RectU8::of(7, 2, 4, 13)), RectU8::of(6, 1, 5, 14));
        assert_eq!(inflate_in_bounds(&RectU8::of(6, 1, 5, 14)), RectU8::of(5, 0, 6, 15));
        assert_eq!(inflate_in_bounds(&RectU8::of(5, 0, 6, 15)), RectU8::of(4, 0, 7, 17));
        assert_eq!(inflate_in_bounds(&RectU8::of(4, 0, 7, 17)), RectU8::of(3, 0, 8, 19));
        assert_eq!(inflate_in_bounds(&RectU8::of(3, 0, 8, 19)), RectU8::of(2, 0, 9, 21));
        assert_eq!(inflate_in_bounds(&RectU8::of(2, 0, 9, 21)), RectU8::of(1, 0, 10, 23));
        assert_eq!(inflate_in_bounds(&RectU8::of(1, 0, 10, 23)), RectU8::of(0, 0, 11, 25));
        assert_eq!(inflate_in_bounds(&RectU8::of(0, 0, 11, 25)), RectU8::of(0, 0, 13, 27));
    }

    #[test]
    fn inflate_in_bounds_max_bounds() {
        assert_eq!(inflate_in_bounds(&RectU8::of(200, 230, u8::MAX - 5, u8::MAX - 3)), RectU8::of(199, 229, u8::MAX - 4, u8::MAX - 2));
        assert_eq!(inflate_in_bounds(&RectU8::of(199, 229, u8::MAX - 4, u8::MAX - 2)), RectU8::of(198, 228, u8::MAX - 3, u8::MAX - 1));
        assert_eq!(inflate_in_bounds(&RectU8::of(198, 228, u8::MAX - 3, u8::MAX - 1)), RectU8::of(197, 227, u8::MAX - 2, u8::MAX));
        assert_eq!(inflate_in_bounds(&RectU8::of(197, 227, u8::MAX - 2, u8::MAX)), RectU8::of(196, 225, u8::MAX - 1, u8::MAX));
        assert_eq!(inflate_in_bounds(&RectU8::of(196, 225, u8::MAX - 1, u8::MAX)), RectU8::of(195, 223, u8::MAX, u8::MAX));
        assert_eq!(inflate_in_bounds(&RectU8::of(195, 223, u8::MAX, u8::MAX)), RectU8::of(193, 221, u8::MAX, u8::MAX));
        assert_eq!(inflate_in_bounds(&RectU8::of(193, 221, u8::MAX, u8::MAX)), RectU8::of(191, 219, u8::MAX, u8::MAX));
        assert_eq!(inflate_in_bounds(&RectU8::of(191, 219, u8::MAX, u8::MAX)), RectU8::of(189, 217, u8::MAX, u8::MAX));
    }
}
