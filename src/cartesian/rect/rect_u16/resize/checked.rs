use crate::cartesian::{
    point::point_u16::PointU16,
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
    let min_x = u16::try_from(temp_min_x).ok()?;
    let min_y = u16::try_from(temp_min_y).ok()?;
    let max_x = min_x.checked_add(size - 1)?;
    let max_y = min_y.checked_add(size - 1)?;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
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
    let min_x = u16::try_from(temp_min_x).ok()?;
    let min_y = u16::try_from(temp_min_y).ok()?;
    let max_x = min_x.checked_add(size - 1)?;
    let max_y = min_y.checked_add(size - 1)?;
    Some(RectU16 { min: PointU16 { x: min_x, y: min_y }, max: PointU16 { x: max_x, y: max_y } })
}

pub fn assign_resize(r: &mut RectU16, size: u16) {
    try_assign_resize(r, size).unwrap()
}

pub fn resize(r: &RectU16, size: u16) -> RectU16 {
    try_resize(r, size).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_u16::RectU16;

    use super::{assign_resize, resize, try_assign_resize, try_resize};

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
        let mut r = RectU16::of(10, 10, 20, 20);
        assert_eq!(try_assign_resize(&mut r, 0), None);
        assert_eq!(try_assign_resize(&mut r, 1), None);
        assert_eq!(try_assign_resize(&mut r, 2), None);
        assert_eq!(r, RectU16::of(10, 10, 20, 20));
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
    fn try_assign_resize_odd_small_rect_same_size() {
        let mut r_min = RectU16::of(0, 0, 2, 2);
        assert_eq!(try_assign_resize(&mut r_min, 3), Some(()));
        assert_eq!(r_min, RectU16::of(0, 0, 2, 2));

        let mut r_max = RectU16::of(u16::MAX - 2, u16::MAX - 2, u16::MAX, u16::MAX);
        assert_eq!(try_assign_resize(&mut r_max, 3), Some(()));
        assert_eq!(r_max, RectU16::of(u16::MAX - 2, u16::MAX - 2, u16::MAX, u16::MAX));
    }

    #[test]
    fn try_assign_resize_even_small_rect_same_size() {
        let mut r = RectU16::of(0, 0, 3, 3);
        assert_eq!(try_assign_resize(&mut r, 4), Some(()));
        assert_eq!(r, RectU16::of(0, 0, 3, 3));

        let mut r = RectU16::of(u16::MAX - 3, u16::MAX - 3, u16::MAX, u16::MAX);
        assert_eq!(try_assign_resize(&mut r, 4), Some(()));
        assert_eq!(r, RectU16::of(u16::MAX - 3, u16::MAX - 3, u16::MAX, u16::MAX));
    }

    #[test]
    fn try_assign_resize_odd_small_rect_to_bounds() {
        let mut r_min = RectU16::of(2, 2, 4, 4);
        assert_eq!(try_assign_resize(&mut r_min, 7), Some(()));
        assert_eq!(r_min, RectU16::of(0, 0, 6, 6));

        let mut r_max = RectU16::of(u16::MAX - 4, u16::MAX - 4, u16::MAX - 2, u16::MAX - 2);
        assert_eq!(try_assign_resize(&mut r_max, 7), Some(()));
        assert_eq!(r_max, RectU16::of(u16::MAX - 6, u16::MAX - 6, u16::MAX, u16::MAX));
    }

    #[test]
    fn try_assign_resize_even_small_rect_to_bounds() {
        let mut r = RectU16::of(2, 2, 5, 5);
        assert_eq!(try_assign_resize(&mut r, 8), Some(()));
        assert_eq!(r, RectU16::of(0, 0, 7, 7));

        let mut r = RectU16::of(u16::MAX - 5, u16::MAX - 5, u16::MAX - 2, u16::MAX - 2);
        assert_eq!(try_assign_resize(&mut r, 8), Some(()));
        assert_eq!(r, RectU16::of(u16::MAX - 7, u16::MAX - 7, u16::MAX, u16::MAX));
    }

    #[test]
    fn try_assign_resize_odd_small_rect_out_of_bounds() {
        let mut r_min = RectU16::of(0, 0, 2, 2);
        assert_eq!(try_assign_resize(&mut r_min, 5), None);
        assert_eq!(r_min, RectU16::of(0, 0, 2, 2));

        let mut r_min_x = RectU16::of(0, 2, 2, 4);
        assert_eq!(try_assign_resize(&mut r_min_x, 5), None);
        assert_eq!(r_min_x, RectU16::of(0, 2, 2, 4));

        let mut r_min_y = RectU16::of(2, 0, 4, 2);
        assert_eq!(try_assign_resize(&mut r_min_y, 5), None);
        assert_eq!(r_min_y, RectU16::of(2, 0, 4, 2));

        let mut r_max = RectU16::of(u16::MAX - 2, u16::MAX - 2, u16::MAX, u16::MAX);
        assert_eq!(try_assign_resize(&mut r_max, 5), None);
        assert_eq!(r_max, RectU16::of(u16::MAX - 2, u16::MAX - 2, u16::MAX, u16::MAX));

        let mut r_max_x = RectU16::of(u16::MAX - 2, u16::MAX - 4, u16::MAX, u16::MAX - 2);
        assert_eq!(try_assign_resize(&mut r_max_x, 5), None);
        assert_eq!(r_max_x, RectU16::of(u16::MAX - 2, u16::MAX - 4, u16::MAX, u16::MAX - 2));

        let mut r_max_y = RectU16::of(u16::MAX - 4, u16::MAX - 2, u16::MAX - 2, u16::MAX);
        assert_eq!(try_assign_resize(&mut r_max_y, 5), None);
        assert_eq!(r_max_y, RectU16::of(u16::MAX - 4, u16::MAX - 2, u16::MAX - 2, u16::MAX));
    }

    #[test]
    fn try_assign_resize_even_small_rect_out_of_bounds() {
        let mut r_min = RectU16::of(0, 0, 3, 3);
        assert_eq!(try_assign_resize(&mut r_min, 6), None);
        assert_eq!(r_min, RectU16::of(0, 0, 3, 3));

        let mut r_min_x = RectU16::of(0, 3, 3, 6);
        assert_eq!(try_assign_resize(&mut r_min_x, 6), None);
        assert_eq!(r_min_x, RectU16::of(0, 3, 3, 6));

        let mut r_min_y = RectU16::of(3, 0, 6, 3);
        assert_eq!(try_assign_resize(&mut r_min_y, 6), None);
        assert_eq!(r_min_y, RectU16::of(3, 0, 6, 3));

        let mut r_max = RectU16::of(u16::MAX - 3, u16::MAX - 3, u16::MAX, u16::MAX);
        assert_eq!(try_assign_resize(&mut r_max, 6), None);
        assert_eq!(r_max, RectU16::of(u16::MAX - 3, u16::MAX - 3, u16::MAX, u16::MAX));

        let mut r_max_x = RectU16::of(u16::MAX - 3, u16::MAX - 6, u16::MAX, u16::MAX - 3);
        assert_eq!(try_assign_resize(&mut r_max_x, 6), None);
        assert_eq!(r_max_x, RectU16::of(u16::MAX - 3, u16::MAX - 6, u16::MAX, u16::MAX - 3));

        let mut r_max_y = RectU16::of(u16::MAX - 6, u16::MAX - 3, u16::MAX - 3, u16::MAX);
        assert_eq!(try_assign_resize(&mut r_max_y, 6), None);
        assert_eq!(r_max_y, RectU16::of(u16::MAX - 6, u16::MAX - 3, u16::MAX - 3, u16::MAX));
    }

    #[test]
    fn try_assign_resize_odd_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU16::of(0, 0, 2, 2);
        assert_eq!(try_assign_resize(&mut r_min, u16::MAX), None);
        assert_eq!(r_min, RectU16::of(0, 0, 2, 2));

        let mut r_min_x = RectU16::of(0, 2, 2, 4);
        assert_eq!(try_assign_resize(&mut r_min_x, u16::MAX), None);
        assert_eq!(r_min_x, RectU16::of(0, 2, 2, 4));

        let mut r_min_y = RectU16::of(2, 0, 4, 2);
        assert_eq!(try_assign_resize(&mut r_min_y, u16::MAX), None);
        assert_eq!(r_min_y, RectU16::of(2, 0, 4, 2));

        let mut r_max = RectU16::of(u16::MAX - 2, u16::MAX - 2, u16::MAX, u16::MAX);
        assert_eq!(try_assign_resize(&mut r_max, u16::MAX), None);
        assert_eq!(r_max, RectU16::of(u16::MAX - 2, u16::MAX - 2, u16::MAX, u16::MAX));

        let mut r_max_x = RectU16::of(u16::MAX - 2, u16::MAX - 4, u16::MAX, u16::MAX - 2);
        assert_eq!(try_assign_resize(&mut r_max_x, u16::MAX), None);
        assert_eq!(r_max_x, RectU16::of(u16::MAX - 2, u16::MAX - 4, u16::MAX, u16::MAX - 2));

        let mut r_max_y = RectU16::of(u16::MAX - 4, u16::MAX - 2, u16::MAX - 2, u16::MAX);
        assert_eq!(try_assign_resize(&mut r_max_y, u16::MAX), None);
        assert_eq!(r_max_y, RectU16::of(u16::MAX - 4, u16::MAX - 2, u16::MAX - 2, u16::MAX));
    }

    #[test]
    fn try_assign_resize_even_small_rect_limits_out_of_bounds() {
        let mut r_min = RectU16::of(0, 0, 3, 3);
        assert_eq!(try_assign_resize(&mut r_min, u16::MAX - 1), None);
        assert_eq!(r_min, RectU16::of(0, 0, 3, 3));

        let mut r_min_x = RectU16::of(0, 3, 3, 6);
        assert_eq!(try_assign_resize(&mut r_min_x, u16::MAX - 1), None);
        assert_eq!(r_min_x, RectU16::of(0, 3, 3, 6));

        let mut r_min_y = RectU16::of(3, 0, 6, 3);
        assert_eq!(try_assign_resize(&mut r_min_y, u16::MAX - 1), None);
        assert_eq!(r_min_y, RectU16::of(3, 0, 6, 3));

        let mut r_max = RectU16::of(u16::MAX - 3, u16::MAX - 3, u16::MAX, u16::MAX);
        assert_eq!(try_assign_resize(&mut r_max, u16::MAX - 1), None);
        assert_eq!(r_max, RectU16::of(u16::MAX - 3, u16::MAX - 3, u16::MAX, u16::MAX));

        let mut r_max_x = RectU16::of(u16::MAX - 3, u16::MAX - 6, u16::MAX, u16::MAX - 3);
        assert_eq!(try_assign_resize(&mut r_max_x, u16::MAX - 1), None);
        assert_eq!(r_max_x, RectU16::of(u16::MAX - 3, u16::MAX - 6, u16::MAX, u16::MAX - 3));

        let mut r_max_y = RectU16::of(u16::MAX - 6, u16::MAX - 3, u16::MAX - 3, u16::MAX);
        assert_eq!(try_assign_resize(&mut r_max_y, u16::MAX - 1), None);
        assert_eq!(r_max_y, RectU16::of(u16::MAX - 6, u16::MAX - 3, u16::MAX - 3, u16::MAX));
    }

    #[test]
    fn try_assign_resize_big_rect_limits_out_of_bounds() {
        let mut r_odd_1 = RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1);
        assert_eq!(try_assign_resize(&mut r_odd_1, u16::MAX), Some(()));
        assert_eq!(r_odd_1, RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1));

        let mut r_odd_1 = RectU16::of(1, 1, u16::MAX, u16::MAX);
        assert_eq!(try_assign_resize(&mut r_odd_1, u16::MAX), Some(()));
        assert_eq!(r_odd_1, RectU16::of(1, 1, u16::MAX, u16::MAX));

        let mut r_even = RectU16::largest();
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
        let r = RectU16::of(10, 10, 20, 20);
        assert_eq!(try_resize(&r, 0), None);
        assert_eq!(try_resize(&r, 1), None);
        assert_eq!(try_resize(&r, 2), None);
    }

    #[test]
    fn try_resize_same_size() {
        assert_eq!(try_resize(&RectU16::of(10, 10, 20, 20), 11), Some(RectU16::of(10, 10, 20, 20)));
        assert_eq!(try_resize(&RectU16::of(10, 10, 21, 21), 12), Some(RectU16::of(10, 10, 21, 21)));
        assert_eq!(try_resize(&RectU16::of(9, 9, 21, 21), 13), Some(RectU16::of(9, 9, 21, 21)));
    }

    #[test]
    fn try_resize_odd_small_rect_same_size() {
        assert_eq!(try_resize(&RectU16::of(0, 0, 2, 2), 3), Some(RectU16::of(0, 0, 2, 2)));
        assert_eq!(
            try_resize(&RectU16::of(u16::MAX - 2, u16::MAX - 2, u16::MAX, u16::MAX), 3),
            Some(RectU16::of(u16::MAX - 2, u16::MAX - 2, u16::MAX, u16::MAX))
        );
    }

    #[test]
    fn try_resize_even_small_rect_same_size() {
        assert_eq!(try_resize(&RectU16::of(0, 0, 3, 3), 4), Some(RectU16::of(0, 0, 3, 3)));
        assert_eq!(
            try_resize(&RectU16::of(u16::MAX - 3, u16::MAX - 3, u16::MAX, u16::MAX), 4),
            Some(RectU16::of(u16::MAX - 3, u16::MAX - 3, u16::MAX, u16::MAX))
        );
    }

    #[test]
    fn try_resize_odd_small_rect_to_bounds() {
        assert_eq!(try_resize(&RectU16::of(2, 2, 4, 4), 7), Some(RectU16::of(0, 0, 6, 6)));
        assert_eq!(
            try_resize(&RectU16::of(u16::MAX - 4, u16::MAX - 4, u16::MAX - 2, u16::MAX - 2), 7),
            Some(RectU16::of(u16::MAX - 6, u16::MAX - 6, u16::MAX, u16::MAX))
        );
    }

    #[test]
    fn try_resize_even_small_rect_to_bounds() {
        assert_eq!(try_resize(&RectU16::of(2, 2, 5, 5), 8), Some(RectU16::of(0, 0, 7, 7)));
        assert_eq!(
            try_resize(&RectU16::of(u16::MAX - 5, u16::MAX - 5, u16::MAX - 2, u16::MAX - 2), 8),
            Some(RectU16::of(u16::MAX - 7, u16::MAX - 7, u16::MAX, u16::MAX))
        );
    }

    #[test]
    fn try_resize_odd_small_rect_out_of_bounds() {
        assert_eq!(try_resize(&RectU16::of(0, 0, 2, 2), 5), None);
        assert_eq!(try_resize(&RectU16::of(0, 2, 2, 4), 5), None);
        assert_eq!(try_resize(&RectU16::of(2, 0, 4, 2), 5), None);
        assert_eq!(try_resize(&RectU16::of(u16::MAX - 2, u16::MAX - 2, u16::MAX, u16::MAX), 5), None);
        assert_eq!(try_resize(&RectU16::of(u16::MAX - 2, u16::MAX - 4, u16::MAX, u16::MAX - 2), 5), None);
        assert_eq!(try_resize(&RectU16::of(u16::MAX - 4, u16::MAX - 2, u16::MAX - 2, u16::MAX), 5), None);
    }

    #[test]
    fn try_resize_even_small_rect_out_of_bounds() {
        assert_eq!(try_resize(&RectU16::of(0, 0, 3, 3), 6), None);
        assert_eq!(try_resize(&RectU16::of(0, 3, 3, 6), 6), None);
        assert_eq!(try_resize(&RectU16::of(0, 0, 3, 3), 6), None);
        assert_eq!(try_resize(&RectU16::of(u16::MAX - 3, u16::MAX - 3, u16::MAX, u16::MAX), 6), None);
        assert_eq!(try_resize(&RectU16::of(u16::MAX - 3, u16::MAX - 6, u16::MAX, u16::MAX - 3), 6), None);
        assert_eq!(try_resize(&RectU16::of(u16::MAX - 6, u16::MAX - 3, u16::MAX - 3, u16::MAX), 6), None);
    }

    #[test]
    fn try_resize_odd_small_rect_limits_out_of_bounds() {
        assert_eq!(try_resize(&RectU16::of(0, 0, 2, 2), u16::MAX), None);
        assert_eq!(try_resize(&RectU16::of(0, 2, 2, 4), u16::MAX), None);
        assert_eq!(try_resize(&RectU16::of(2, 0, 4, 2), u16::MAX), None);
        assert_eq!(try_resize(&RectU16::of(u16::MAX - 2, u16::MAX - 2, u16::MAX, u16::MAX), u16::MAX), None);
        assert_eq!(try_resize(&RectU16::of(u16::MAX - 2, u16::MAX - 4, u16::MAX, u16::MAX - 2), u16::MAX), None);
        assert_eq!(try_resize(&RectU16::of(u16::MAX - 4, u16::MAX - 2, u16::MAX - 2, u16::MAX), u16::MAX), None);
    }

    #[test]
    fn try_resize_even_small_rect_limits_out_of_bounds() {
        assert_eq!(try_resize(&RectU16::of(0, 0, 3, 3), u16::MAX - 1), None);
        assert_eq!(try_resize(&RectU16::of(0, 3, 3, 6), u16::MAX - 1), None);
        assert_eq!(try_resize(&RectU16::of(0, 0, 3, 3), u16::MAX - 1), None);
        assert_eq!(try_resize(&RectU16::of(u16::MAX - 3, u16::MAX - 3, u16::MAX, u16::MAX), u16::MAX - 1), None);
        assert_eq!(try_resize(&RectU16::of(u16::MAX - 3, u16::MAX - 6, u16::MAX, u16::MAX - 3), u16::MAX - 1), None);
        assert_eq!(try_resize(&RectU16::of(u16::MAX - 6, u16::MAX - 3, u16::MAX - 3, u16::MAX), u16::MAX - 1), None);
    }

    #[test]
    fn try_resize_big_rect_limits_out_of_bounds() {
        assert_eq!(try_resize(&RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1), u16::MAX), Some(RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1)));
        assert_eq!(try_resize(&RectU16::of(1, 1, u16::MAX, u16::MAX), u16::MAX), Some(RectU16::of(1, 1, u16::MAX, u16::MAX)));
        assert_eq!(try_resize(&RectU16::largest(), u16::MAX), Some(RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1)));
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
