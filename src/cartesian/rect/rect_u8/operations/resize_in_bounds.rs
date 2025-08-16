use crate::cartesian::{
    point::point_u8,
    rect::rect_u8::{RectU8, delta_x, delta_y},
};

pub fn try_assign_resize_in_bounds(r: &mut RectU8, size: u8) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_x = i16::from(delta_x(r)) + 1 - i16::from(size);
    let diff_y = i16::from(delta_y(r)) + 1 - i16::from(size);
    let temp_min_x = i16::from(r.min.x) + diff_x / 2;
    let temp_min_y = i16::from(r.min.y) + diff_y / 2;
    let min_x = temp_min_x.clamp(0, i16::from(u8::MAX) - i16::from(size) + 1);
    let min_y = temp_min_y.clamp(0, i16::from(u8::MAX) - i16::from(size) + 1);
    r.min.x = min_x as u8;
    r.min.y = min_y as u8;
    r.max.x = (min_x + i16::from(size) - 1) as u8;
    r.max.y = (min_y + i16::from(size) - 1) as u8;
    Some(())
}

pub fn try_resize_in_bounds(r: &RectU8, size: u8) -> Option<RectU8> {
    if size < 3 {
        return None;
    }
    let diff_x = i16::from(delta_x(r)) + 1 - i16::from(size);
    let diff_y = i16::from(delta_y(r)) + 1 - i16::from(size);
    let temp_min_x = i16::from(r.min.x) + diff_x / 2;
    let temp_min_y = i16::from(r.min.y) + diff_y / 2;
    let clamped_min_x = temp_min_x.clamp(0, i16::from(u8::MAX) - i16::from(size) + 1);
    let clamped_min_y = temp_min_y.clamp(0, i16::from(u8::MAX) - i16::from(size) + 1);
    let min_x = clamped_min_x as u8;
    let min_y = clamped_min_y as u8;
    let max_x = (clamped_min_x + i16::from(size) - 1) as u8;
    let max_y = (clamped_min_y + i16::from(size) - 1) as u8;
    Some(RectU8 { min: point_u8::PointU8 { x: min_x, y: min_y }, max: point_u8::PointU8 { x: max_x, y: max_y } })
}

pub fn assign_resize_in_bounds(r: &mut RectU8, size: u8) {
    try_assign_resize_in_bounds(r, size).unwrap()
}

pub fn resize_in_bounds(r: &RectU8, size: u8) -> RectU8 {
    try_resize_in_bounds(r, size).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{RectU8, assign_resize_in_bounds, resize_in_bounds, try_assign_resize_in_bounds, try_resize_in_bounds};

    #[test]
    fn try_assign_resize_in_bounds_odd() {
        let mut r = RectU8::of(5, 5, 15, 15);
        assert_eq!(try_assign_resize_in_bounds(&mut r, 11), Some(()));
        assert_eq!(r, RectU8::of(5, 5, 15, 15));
        assert_eq!(try_assign_resize_in_bounds(&mut r, 9), Some(()));
        assert_eq!(r, RectU8::of(6, 6, 14, 14));
        assert_eq!(try_assign_resize_in_bounds(&mut r, 7), Some(()));
        assert_eq!(r, RectU8::of(7, 7, 13, 13));
        assert_eq!(try_assign_resize_in_bounds(&mut r, 5), Some(()));
        assert_eq!(r, RectU8::of(8, 8, 12, 12));
        assert_eq!(try_assign_resize_in_bounds(&mut r, 3), Some(()));
        assert_eq!(r, RectU8::of(9, 9, 11, 11));
        assert_eq!(try_assign_resize_in_bounds(&mut r, 9), Some(()));
        assert_eq!(r, RectU8::of(6, 6, 14, 14));
    }

    #[test]
    fn try_assign_resize_in_bounds_even() {
        let mut r = RectU8::of(5, 5, 14, 14);
        assert_eq!(try_assign_resize_in_bounds(&mut r, 10), Some(()));
        assert_eq!(r, RectU8::of(5, 5, 14, 14));
        assert_eq!(try_assign_resize_in_bounds(&mut r, 8), Some(()));
        assert_eq!(r, RectU8::of(6, 6, 13, 13));
        assert_eq!(try_assign_resize_in_bounds(&mut r, 6), Some(()));
        assert_eq!(r, RectU8::of(7, 7, 12, 12));
        assert_eq!(try_assign_resize_in_bounds(&mut r, 4), Some(()));
        assert_eq!(r, RectU8::of(8, 8, 11, 11));
        assert_eq!(try_assign_resize_in_bounds(&mut r, 8), Some(()));
        assert_eq!(r, RectU8::of(6, 6, 13, 13));
    }

    #[test]
    fn try_assign_resize_in_bounds_small_size() {
        let mut r = RectU8::of(10, 10, 100, 100);
        assert_eq!(try_assign_resize_in_bounds(&mut r, 0), None);
        assert_eq!(try_assign_resize_in_bounds(&mut r, 1), None);
        assert_eq!(try_assign_resize_in_bounds(&mut r, 2), None);
        assert_eq!(r, RectU8::of(10, 10, 100, 100));
    }

    #[test]
    fn try_assign_resize_in_bounds_same_size() {
        let mut r_11 = RectU8::of(10, 10, 20, 20);
        assert_eq!(try_assign_resize_in_bounds(&mut r_11, 11), Some(()));
        assert_eq!(r_11, RectU8::of(10, 10, 20, 20));

        let mut r_12 = RectU8::of(10, 10, 21, 21);
        assert_eq!(try_assign_resize_in_bounds(&mut r_12, 12), Some(()));
        assert_eq!(r_12, RectU8::of(10, 10, 21, 21));

        let mut r_13 = RectU8::of(9, 9, 21, 21);
        assert_eq!(try_assign_resize_in_bounds(&mut r_13, 13), Some(()));
        assert_eq!(r_13, RectU8::of(9, 9, 21, 21));
    }

    #[test]
    fn try_assign_resize_in_bounds_odd_small_rect_out_of_bounds() {
        let mut r_min = RectU8::of(0, 0, 2, 2);
        assert_eq!(try_assign_resize_in_bounds(&mut r_min, 11), Some(()));
        assert_eq!(r_min, RectU8::of(0, 0, 10, 10));

        let mut r_max = RectU8::of(u8::MAX - 2, u8::MAX - 2, u8::MAX, u8::MAX);
        assert_eq!(try_assign_resize_in_bounds(&mut r_max, 11), Some(()));
        assert_eq!(r_max, RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX, u8::MAX));
    }

    #[test]
    fn try_assign_resize_in_bounds_even_small_rect_out_of_bounds() {
        let mut r = RectU8::of(0, 0, 3, 3);
        assert_eq!(try_assign_resize_in_bounds(&mut r, 11), Some(()));
        assert_eq!(r, RectU8::of(0, 0, 10, 10));

        let mut r = RectU8::of(u8::MAX - 3, u8::MAX - 3, u8::MAX, u8::MAX);
        assert_eq!(try_assign_resize_in_bounds(&mut r, 11), Some(()));
        assert_eq!(r, RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX, u8::MAX));
    }

    #[test]
    fn try_assign_resize_in_bounds_odd_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU8::of(0, 0, 2, 2);
        assert_eq!(try_assign_resize_in_bounds(&mut r_min, u8::MAX), Some(()));
        assert_eq!(r_min, RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1));

        let mut r_max = RectU8::of(u8::MAX - 2, u8::MAX - 2, u8::MAX, u8::MAX);
        assert_eq!(try_assign_resize_in_bounds(&mut r_max, u8::MAX), Some(()));
        assert_eq!(r_max, RectU8::of(1, 1, u8::MAX, u8::MAX));
    }

    #[test]
    fn try_assign_resize_in_bounds_even_small_rect_limits_out_of_bounds() {
        let mut r = RectU8::of(0, 0, 3, 3);
        assert_eq!(try_assign_resize_in_bounds(&mut r, u8::MAX - 1), Some(()));
        assert_eq!(r, RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 2));

        let mut r = RectU8::of(u8::MAX - 3, u8::MAX - 3, u8::MAX, u8::MAX);
        assert_eq!(try_assign_resize_in_bounds(&mut r, u8::MAX - 1), Some(()));
        assert_eq!(r, RectU8::of(2, 2, u8::MAX, u8::MAX));
    }

    #[test]
    fn try_assign_resize_in_bounds_big_rect_limits_out_of_bounds() {
        let mut r_odd_1 = RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1);
        assert_eq!(try_assign_resize_in_bounds(&mut r_odd_1, u8::MAX), Some(()));
        assert_eq!(r_odd_1, RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1));

        let mut r_odd_1 = RectU8::of(1, 1, u8::MAX, u8::MAX);
        assert_eq!(try_assign_resize_in_bounds(&mut r_odd_1, u8::MAX), Some(()));
        assert_eq!(r_odd_1, RectU8::of(1, 1, u8::MAX, u8::MAX));

        let mut r_even = RectU8::of(0, 0, u8::MAX, u8::MAX);
        assert_eq!(try_assign_resize_in_bounds(&mut r_even, u8::MAX), Some(()));
        assert_eq!(r_even, RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1));
    }

    #[test]
    fn try_resize_in_bounds_odd() {
        assert_eq!(try_resize_in_bounds(&RectU8::of(5, 5, 15, 15), 11), Some(RectU8::of(5, 5, 15, 15)));
        assert_eq!(try_resize_in_bounds(&RectU8::of(5, 5, 15, 15), 9), Some(RectU8::of(6, 6, 14, 14)));
        assert_eq!(try_resize_in_bounds(&RectU8::of(6, 6, 14, 14), 7), Some(RectU8::of(7, 7, 13, 13)));
        assert_eq!(try_resize_in_bounds(&RectU8::of(7, 7, 13, 13), 5), Some(RectU8::of(8, 8, 12, 12)));
        assert_eq!(try_resize_in_bounds(&RectU8::of(8, 8, 12, 12), 3), Some(RectU8::of(9, 9, 11, 11)));
        assert_eq!(try_resize_in_bounds(&RectU8::of(9, 9, 11, 11), 9), Some(RectU8::of(6, 6, 14, 14)));
    }

    #[test]
    fn try_resize_in_bounds_even() {
        assert_eq!(try_resize_in_bounds(&RectU8::of(5, 5, 14, 14), 10), Some(RectU8::of(5, 5, 14, 14)));
        assert_eq!(try_resize_in_bounds(&RectU8::of(5, 5, 14, 14), 8), Some(RectU8::of(6, 6, 13, 13)));
        assert_eq!(try_resize_in_bounds(&RectU8::of(6, 6, 13, 13), 6), Some(RectU8::of(7, 7, 12, 12)));
        assert_eq!(try_resize_in_bounds(&RectU8::of(7, 7, 12, 12), 4), Some(RectU8::of(8, 8, 11, 11)));
        assert_eq!(try_resize_in_bounds(&RectU8::of(8, 8, 11, 11), 8), Some(RectU8::of(6, 6, 13, 13)));
    }

    #[test]
    fn try_resize_in_bounds_small_size() {
        assert_eq!(try_resize_in_bounds(&RectU8::of(10, 10, 100, 100), 0), None);
        assert_eq!(try_resize_in_bounds(&RectU8::of(10, 10, 100, 100), 1), None);
        assert_eq!(try_resize_in_bounds(&RectU8::of(10, 10, 100, 100), 2), None);
    }

    #[test]
    fn try_resize_in_bounds_same_size() {
        assert_eq!(try_resize_in_bounds(&RectU8::of(10, 10, 20, 20), 11), Some(RectU8::of(10, 10, 20, 20)));
        assert_eq!(try_resize_in_bounds(&RectU8::of(10, 10, 21, 21), 12), Some(RectU8::of(10, 10, 21, 21)));
        assert_eq!(try_resize_in_bounds(&RectU8::of(9, 9, 21, 21), 13), Some(RectU8::of(9, 9, 21, 21)));
    }

    #[test]
    fn try_resize_in_bounds_odd_small_rect_out_of_bounds() {
        assert_eq!(try_resize_in_bounds(&RectU8::of(0, 0, 2, 2), 11), Some(RectU8::of(0, 0, 10, 10)));
        assert_eq!(
            try_resize_in_bounds(&RectU8::of(u8::MAX - 2, u8::MAX - 2, u8::MAX, u8::MAX), 11),
            Some(RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX, u8::MAX))
        );
    }

    #[test]
    fn try_resize_in_bounds_even_small_rect_out_of_bounds() {
        assert_eq!(try_resize_in_bounds(&RectU8::of(0, 0, 3, 3), 11), Some(RectU8::of(0, 0, 10, 10)));
        assert_eq!(
            try_resize_in_bounds(&RectU8::of(u8::MAX - 3, u8::MAX - 3, u8::MAX, u8::MAX), 11),
            Some(RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX, u8::MAX))
        );
    }

    #[test]
    fn try_resize_in_bounds_odd_small_rect_limits_out_of_bounds() {
        assert_eq!(try_resize_in_bounds(&RectU8::of(0, 0, 2, 2), u8::MAX), Some(RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1)));
        assert_eq!(try_resize_in_bounds(&RectU8::of(u8::MAX - 2, u8::MAX - 2, u8::MAX, u8::MAX), u8::MAX), Some(RectU8::of(1, 1, u8::MAX, u8::MAX)));
    }

    #[test]
    fn try_resize_in_bounds_even_small_rect_limits_out_of_bounds() {
        assert_eq!(try_resize_in_bounds(&RectU8::of(0, 0, 3, 3), u8::MAX - 1), Some(RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 2)));
        assert_eq!(
            try_resize_in_bounds(&RectU8::of(u8::MAX - 3, u8::MAX - 3, u8::MAX, u8::MAX), u8::MAX - 1),
            Some(RectU8::of(2, 2, u8::MAX, u8::MAX))
        );
    }

    #[test]
    fn try_resize_in_bounds_big_rect_limits_out_of_bounds() {
        assert_eq!(try_resize_in_bounds(&RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1), u8::MAX), Some(RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1)));
        assert_eq!(try_resize_in_bounds(&RectU8::of(1, 1, u8::MAX, u8::MAX), u8::MAX), Some(RectU8::of(1, 1, u8::MAX, u8::MAX)));
        assert_eq!(try_resize_in_bounds(&RectU8::of(0, 0, u8::MAX, u8::MAX), u8::MAX), Some(RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1)));
    }

    #[test]
    fn assign_resize_in_bounds_odd() {
        let mut r = RectU8::of(5, 5, 15, 15);
        assign_resize_in_bounds(&mut r, 11);
        assert_eq!(r, RectU8::of(5, 5, 15, 15));
        assign_resize_in_bounds(&mut r, 9);
        assert_eq!(r, RectU8::of(6, 6, 14, 14));
        assign_resize_in_bounds(&mut r, 7);
        assert_eq!(r, RectU8::of(7, 7, 13, 13));
        assign_resize_in_bounds(&mut r, 5);
        assert_eq!(r, RectU8::of(8, 8, 12, 12));
        assign_resize_in_bounds(&mut r, 3);
        assert_eq!(r, RectU8::of(9, 9, 11, 11));
        assign_resize_in_bounds(&mut r, 9);
        assert_eq!(r, RectU8::of(6, 6, 14, 14));
    }

    #[test]
    fn assign_resize_in_bounds_even() {
        let mut r = RectU8::of(5, 5, 14, 14);
        assign_resize_in_bounds(&mut r, 10);
        assert_eq!(r, RectU8::of(5, 5, 14, 14));
        assign_resize_in_bounds(&mut r, 8);
        assert_eq!(r, RectU8::of(6, 6, 13, 13));
        assign_resize_in_bounds(&mut r, 6);
        assert_eq!(r, RectU8::of(7, 7, 12, 12));
        assign_resize_in_bounds(&mut r, 4);
        assert_eq!(r, RectU8::of(8, 8, 11, 11));
        assign_resize_in_bounds(&mut r, 8);
        assert_eq!(r, RectU8::of(6, 6, 13, 13));
    }

    #[test]
    fn resize_in_bounds_odd() {
        assert_eq!(resize_in_bounds(&RectU8::of(5, 5, 15, 15), 11), RectU8::of(5, 5, 15, 15));
        assert_eq!(resize_in_bounds(&RectU8::of(5, 5, 15, 15), 9), RectU8::of(6, 6, 14, 14));
        assert_eq!(resize_in_bounds(&RectU8::of(6, 6, 14, 14), 7), RectU8::of(7, 7, 13, 13));
        assert_eq!(resize_in_bounds(&RectU8::of(7, 7, 13, 13), 5), RectU8::of(8, 8, 12, 12));
        assert_eq!(resize_in_bounds(&RectU8::of(8, 8, 12, 12), 3), RectU8::of(9, 9, 11, 11));
        assert_eq!(resize_in_bounds(&RectU8::of(9, 9, 11, 11), 9), RectU8::of(6, 6, 14, 14));
    }

    #[test]
    fn resize_in_bounds_even() {
        assert_eq!(resize_in_bounds(&RectU8::of(5, 5, 14, 14), 10), RectU8::of(5, 5, 14, 14));
        assert_eq!(resize_in_bounds(&RectU8::of(5, 5, 14, 14), 8), RectU8::of(6, 6, 13, 13));
        assert_eq!(resize_in_bounds(&RectU8::of(6, 6, 13, 13), 6), RectU8::of(7, 7, 12, 12));
        assert_eq!(resize_in_bounds(&RectU8::of(7, 7, 12, 12), 4), RectU8::of(8, 8, 11, 11));
        assert_eq!(resize_in_bounds(&RectU8::of(8, 8, 11, 11), 8), RectU8::of(6, 6, 13, 13));
    }
}
