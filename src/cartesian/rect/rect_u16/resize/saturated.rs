use crate::cartesian::{
    point::point_u16,
    rect::rect_u16::{RectU16, delta_x, delta_y},
};

pub fn try_assign_resize(r: &mut RectU16, size: u16) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_x = i32::from(delta_x(r)) + 1 - i32::from(size);
    let diff_y = i32::from(delta_y(r)) + 1 - i32::from(size);
    let temp_min_x = i32::from(r.min.x) + diff_x / 2;
    let temp_min_y = i32::from(r.min.y) + diff_y / 2;
    let min_x = temp_min_x.clamp(0, i32::from(u16::MAX) - i32::from(size) + 1);
    let min_y = temp_min_y.clamp(0, i32::from(u16::MAX) - i32::from(size) + 1);
    r.min.x = min_x as u16;
    r.min.y = min_y as u16;
    r.max.x = (min_x + i32::from(size) - 1) as u16;
    r.max.y = (min_y + i32::from(size) - 1) as u16;
    Some(())
}

pub fn try_resize(r: &RectU16, size: u16) -> Option<RectU16> {
    if size < 3 {
        return None;
    }
    let diff_x = i32::from(delta_x(r)) + 1 - i32::from(size);
    let diff_y = i32::from(delta_y(r)) + 1 - i32::from(size);
    let temp_min_x = i32::from(r.min.x) + diff_x / 2;
    let temp_min_y = i32::from(r.min.y) + diff_y / 2;
    let clamped_min_x = temp_min_x.clamp(0, i32::from(u16::MAX) - i32::from(size) + 1);
    let clamped_min_y = temp_min_y.clamp(0, i32::from(u16::MAX) - i32::from(size) + 1);
    let min_x = clamped_min_x as u16;
    let min_y = clamped_min_y as u16;
    let max_x = (clamped_min_x + i32::from(size) - 1) as u16;
    let max_y = (clamped_min_y + i32::from(size) - 1) as u16;
    Some(RectU16 { min: point_u16::PointU16 { x: min_x, y: min_y }, max: point_u16::PointU16 { x: max_x, y: max_y } })
}

pub fn assign_resize(r: &mut RectU16, size: u16) {
    try_assign_resize(r, size).unwrap()
}

pub fn resize(r: &RectU16, size: u16) -> RectU16 {
    try_resize(r, size).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{RectU16, assign_resize, resize, try_assign_resize, try_resize};

    #[test]
    fn try_assign_resize_odd() {
        let mut r = RectU16::of(5, 5, 15, 15);
        assert_eq!(try_assign_resize(&mut r, 11), Some(()));
        assert_eq!(r, RectU16::of(5, 5, 15, 15));
        assert_eq!(try_assign_resize(&mut r, 9), Some(()));
        assert_eq!(r, RectU16::of(6, 6, 14, 14));
        assert_eq!(try_assign_resize(&mut r, 7), Some(()));
        assert_eq!(r, RectU16::of(7, 7, 13, 13));
        assert_eq!(try_assign_resize(&mut r, 5), Some(()));
        assert_eq!(r, RectU16::of(8, 8, 12, 12));
        assert_eq!(try_assign_resize(&mut r, 3), Some(()));
        assert_eq!(r, RectU16::of(9, 9, 11, 11));
        assert_eq!(try_assign_resize(&mut r, 9), Some(()));
        assert_eq!(r, RectU16::of(6, 6, 14, 14));
    }

    #[test]
    fn try_assign_resize_even() {
        let mut r = RectU16::of(5, 5, 14, 14);
        assert_eq!(try_assign_resize(&mut r, 10), Some(()));
        assert_eq!(r, RectU16::of(5, 5, 14, 14));
        assert_eq!(try_assign_resize(&mut r, 8), Some(()));
        assert_eq!(r, RectU16::of(6, 6, 13, 13));
        assert_eq!(try_assign_resize(&mut r, 6), Some(()));
        assert_eq!(r, RectU16::of(7, 7, 12, 12));
        assert_eq!(try_assign_resize(&mut r, 4), Some(()));
        assert_eq!(r, RectU16::of(8, 8, 11, 11));
        assert_eq!(try_assign_resize(&mut r, 8), Some(()));
        assert_eq!(r, RectU16::of(6, 6, 13, 13));
    }

    #[test]
    fn try_assign_resize_small_size() {
        let mut r = RectU16::of(10, 10, 100, 100);
        assert_eq!(try_assign_resize(&mut r, 0), None);
        assert_eq!(try_assign_resize(&mut r, 1), None);
        assert_eq!(try_assign_resize(&mut r, 2), None);
        assert_eq!(r, RectU16::of(10, 10, 100, 100));
    }

    #[test]
    fn try_assign_resize_same_size() {
        let mut r_11 = RectU16::of(10, 10, 20, 20);
        assert_eq!(try_assign_resize(&mut r_11, 11), Some(()));
        assert_eq!(r_11, RectU16::of(10, 10, 20, 20));

        let mut r_12 = RectU16::of(10, 10, 21, 21);
        assert_eq!(try_assign_resize(&mut r_12, 12), Some(()));
        assert_eq!(r_12, RectU16::of(10, 10, 21, 21));

        let mut r_13 = RectU16::of(9, 9, 21, 21);
        assert_eq!(try_assign_resize(&mut r_13, 13), Some(()));
        assert_eq!(r_13, RectU16::of(9, 9, 21, 21));
    }

    #[test]
    fn try_assign_resize_odd_small_rect_out_of_bounds() {
        let mut r_min = RectU16::of(0, 0, 2, 2);
        assert_eq!(try_assign_resize(&mut r_min, 11), Some(()));
        assert_eq!(r_min, RectU16::of(0, 0, 10, 10));

        let mut r_max = RectU16::of(u16::MAX - 2, u16::MAX - 2, u16::MAX, u16::MAX);
        assert_eq!(try_assign_resize(&mut r_max, 11), Some(()));
        assert_eq!(r_max, RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX, u16::MAX));
    }

    #[test]
    fn try_assign_resize_even_small_rect_out_of_bounds() {
        let mut r = RectU16::of(0, 0, 3, 3);
        assert_eq!(try_assign_resize(&mut r, 11), Some(()));
        assert_eq!(r, RectU16::of(0, 0, 10, 10));

        let mut r = RectU16::of(u16::MAX - 3, u16::MAX - 3, u16::MAX, u16::MAX);
        assert_eq!(try_assign_resize(&mut r, 11), Some(()));
        assert_eq!(r, RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX, u16::MAX));
    }

    #[test]
    fn try_assign_resize_odd_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU16::of(0, 0, 2, 2);
        assert_eq!(try_assign_resize(&mut r_min, u16::MAX), Some(()));
        assert_eq!(r_min, RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1));

        let mut r_max = RectU16::of(u16::MAX - 2, u16::MAX - 2, u16::MAX, u16::MAX);
        assert_eq!(try_assign_resize(&mut r_max, u16::MAX), Some(()));
        assert_eq!(r_max, RectU16::of(1, 1, u16::MAX, u16::MAX));
    }

    #[test]
    fn try_assign_resize_even_small_rect_limits_out_of_bounds() {
        let mut r = RectU16::of(0, 0, 3, 3);
        assert_eq!(try_assign_resize(&mut r, u16::MAX - 1), Some(()));
        assert_eq!(r, RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 2));

        let mut r = RectU16::of(u16::MAX - 3, u16::MAX - 3, u16::MAX, u16::MAX);
        assert_eq!(try_assign_resize(&mut r, u16::MAX - 1), Some(()));
        assert_eq!(r, RectU16::of(2, 2, u16::MAX, u16::MAX));
    }

    #[test]
    fn try_assign_resize_big_rect_limits_out_of_bounds() {
        let mut r_odd_1 = RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1);
        assert_eq!(try_assign_resize(&mut r_odd_1, u16::MAX), Some(()));
        assert_eq!(r_odd_1, RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1));

        let mut r_odd_1 = RectU16::of(1, 1, u16::MAX, u16::MAX);
        assert_eq!(try_assign_resize(&mut r_odd_1, u16::MAX), Some(()));
        assert_eq!(r_odd_1, RectU16::of(1, 1, u16::MAX, u16::MAX));

        let mut r_even = RectU16::of(0, 0, u16::MAX, u16::MAX);
        assert_eq!(try_assign_resize(&mut r_even, u16::MAX), Some(()));
        assert_eq!(r_even, RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1));
    }

    #[test]
    fn try_resize_odd() {
        assert_eq!(try_resize(&RectU16::of(5, 5, 15, 15), 11), Some(RectU16::of(5, 5, 15, 15)));
        assert_eq!(try_resize(&RectU16::of(5, 5, 15, 15), 9), Some(RectU16::of(6, 6, 14, 14)));
        assert_eq!(try_resize(&RectU16::of(6, 6, 14, 14), 7), Some(RectU16::of(7, 7, 13, 13)));
        assert_eq!(try_resize(&RectU16::of(7, 7, 13, 13), 5), Some(RectU16::of(8, 8, 12, 12)));
        assert_eq!(try_resize(&RectU16::of(8, 8, 12, 12), 3), Some(RectU16::of(9, 9, 11, 11)));
        assert_eq!(try_resize(&RectU16::of(9, 9, 11, 11), 9), Some(RectU16::of(6, 6, 14, 14)));
    }

    #[test]
    fn try_resize_even() {
        assert_eq!(try_resize(&RectU16::of(5, 5, 14, 14), 10), Some(RectU16::of(5, 5, 14, 14)));
        assert_eq!(try_resize(&RectU16::of(5, 5, 14, 14), 8), Some(RectU16::of(6, 6, 13, 13)));
        assert_eq!(try_resize(&RectU16::of(6, 6, 13, 13), 6), Some(RectU16::of(7, 7, 12, 12)));
        assert_eq!(try_resize(&RectU16::of(7, 7, 12, 12), 4), Some(RectU16::of(8, 8, 11, 11)));
        assert_eq!(try_resize(&RectU16::of(8, 8, 11, 11), 8), Some(RectU16::of(6, 6, 13, 13)));
    }

    #[test]
    fn try_resize_small_size() {
        assert_eq!(try_resize(&RectU16::of(10, 10, 100, 100), 0), None);
        assert_eq!(try_resize(&RectU16::of(10, 10, 100, 100), 1), None);
        assert_eq!(try_resize(&RectU16::of(10, 10, 100, 100), 2), None);
    }

    #[test]
    fn try_resize_same_size() {
        assert_eq!(try_resize(&RectU16::of(10, 10, 20, 20), 11), Some(RectU16::of(10, 10, 20, 20)));
        assert_eq!(try_resize(&RectU16::of(10, 10, 21, 21), 12), Some(RectU16::of(10, 10, 21, 21)));
        assert_eq!(try_resize(&RectU16::of(9, 9, 21, 21), 13), Some(RectU16::of(9, 9, 21, 21)));
    }

    #[test]
    fn try_resize_odd_small_rect_out_of_bounds() {
        assert_eq!(try_resize(&RectU16::of(0, 0, 2, 2), 11), Some(RectU16::of(0, 0, 10, 10)));
        assert_eq!(
            try_resize(&RectU16::of(u16::MAX - 2, u16::MAX - 2, u16::MAX, u16::MAX), 11),
            Some(RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX, u16::MAX))
        );
    }

    #[test]
    fn try_resize_even_small_rect_out_of_bounds() {
        assert_eq!(try_resize(&RectU16::of(0, 0, 3, 3), 11), Some(RectU16::of(0, 0, 10, 10)));
        assert_eq!(
            try_resize(&RectU16::of(u16::MAX - 3, u16::MAX - 3, u16::MAX, u16::MAX), 11),
            Some(RectU16::of(u16::MAX - 10, u16::MAX - 10, u16::MAX, u16::MAX))
        );
    }

    #[test]
    fn try_resize_odd_small_rect_limits_out_of_bounds() {
        assert_eq!(try_resize(&RectU16::of(0, 0, 2, 2), u16::MAX), Some(RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1)));
        assert_eq!(try_resize(&RectU16::of(u16::MAX - 2, u16::MAX - 2, u16::MAX, u16::MAX), u16::MAX), Some(RectU16::of(1, 1, u16::MAX, u16::MAX)));
    }

    #[test]
    fn try_resize_even_small_rect_limits_out_of_bounds() {
        assert_eq!(try_resize(&RectU16::of(0, 0, 3, 3), u16::MAX - 1), Some(RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 2)));
        assert_eq!(
            try_resize(&RectU16::of(u16::MAX - 3, u16::MAX - 3, u16::MAX, u16::MAX), u16::MAX - 1),
            Some(RectU16::of(2, 2, u16::MAX, u16::MAX))
        );
    }

    #[test]
    fn try_resize_big_rect_limits_out_of_bounds() {
        assert_eq!(try_resize(&RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1), u16::MAX), Some(RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1)));
        assert_eq!(try_resize(&RectU16::of(1, 1, u16::MAX, u16::MAX), u16::MAX), Some(RectU16::of(1, 1, u16::MAX, u16::MAX)));
        assert_eq!(try_resize(&RectU16::of(0, 0, u16::MAX, u16::MAX), u16::MAX), Some(RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1)));
    }

    #[test]
    fn assign_resize_odd() {
        let mut r = RectU16::of(5, 5, 15, 15);
        assign_resize(&mut r, 11);
        assert_eq!(r, RectU16::of(5, 5, 15, 15));
        assign_resize(&mut r, 9);
        assert_eq!(r, RectU16::of(6, 6, 14, 14));
        assign_resize(&mut r, 7);
        assert_eq!(r, RectU16::of(7, 7, 13, 13));
        assign_resize(&mut r, 5);
        assert_eq!(r, RectU16::of(8, 8, 12, 12));
        assign_resize(&mut r, 3);
        assert_eq!(r, RectU16::of(9, 9, 11, 11));
        assign_resize(&mut r, 9);
        assert_eq!(r, RectU16::of(6, 6, 14, 14));
    }

    #[test]
    fn assign_resize_even() {
        let mut r = RectU16::of(5, 5, 14, 14);
        assign_resize(&mut r, 10);
        assert_eq!(r, RectU16::of(5, 5, 14, 14));
        assign_resize(&mut r, 8);
        assert_eq!(r, RectU16::of(6, 6, 13, 13));
        assign_resize(&mut r, 6);
        assert_eq!(r, RectU16::of(7, 7, 12, 12));
        assign_resize(&mut r, 4);
        assert_eq!(r, RectU16::of(8, 8, 11, 11));
        assign_resize(&mut r, 8);
        assert_eq!(r, RectU16::of(6, 6, 13, 13));
    }

    #[test]
    fn resize_odd() {
        assert_eq!(resize(&RectU16::of(5, 5, 15, 15), 11), RectU16::of(5, 5, 15, 15));
        assert_eq!(resize(&RectU16::of(5, 5, 15, 15), 9), RectU16::of(6, 6, 14, 14));
        assert_eq!(resize(&RectU16::of(6, 6, 14, 14), 7), RectU16::of(7, 7, 13, 13));
        assert_eq!(resize(&RectU16::of(7, 7, 13, 13), 5), RectU16::of(8, 8, 12, 12));
        assert_eq!(resize(&RectU16::of(8, 8, 12, 12), 3), RectU16::of(9, 9, 11, 11));
        assert_eq!(resize(&RectU16::of(9, 9, 11, 11), 9), RectU16::of(6, 6, 14, 14));
    }

    #[test]
    fn resize_even() {
        assert_eq!(resize(&RectU16::of(5, 5, 14, 14), 10), RectU16::of(5, 5, 14, 14));
        assert_eq!(resize(&RectU16::of(5, 5, 14, 14), 8), RectU16::of(6, 6, 13, 13));
        assert_eq!(resize(&RectU16::of(6, 6, 13, 13), 6), RectU16::of(7, 7, 12, 12));
        assert_eq!(resize(&RectU16::of(7, 7, 12, 12), 4), RectU16::of(8, 8, 11, 11));
        assert_eq!(resize(&RectU16::of(8, 8, 11, 11), 8), RectU16::of(6, 6, 13, 13));
    }
}
