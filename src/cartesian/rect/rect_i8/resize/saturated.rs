use crate::cartesian::{
    point::point_i8::PointI8,
    rect::rect_i8::{RectI8, delta_x, delta_y},
};

pub fn try_assign_resize(r: &mut RectI8, size: u8) -> Option<()> {
    if size < 3 {
        return None;
    }
    let diff_x = i16::from(delta_x(r)) + 1 - i16::from(size);
    let diff_y = i16::from(delta_y(r)) + 1 - i16::from(size);
    let temp_min_x = i16::from(r.min.x) + diff_x / 2;
    let temp_min_y = i16::from(r.min.y) + diff_y / 2;
    let min_x = temp_min_x.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(size) + 1);
    let min_y = temp_min_y.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(size) + 1);
    r.min.x = min_x as i8;
    r.min.y = min_y as i8;
    r.max.x = (min_x + i16::from(size) - 1) as i8;
    r.max.y = (min_y + i16::from(size) - 1) as i8;
    Some(())
}

pub fn try_resize(r: &RectI8, size: u8) -> Option<RectI8> {
    if size < 3 {
        return None;
    }
    let diff_x = i16::from(delta_x(r)) + 1 - i16::from(size);
    let diff_y = i16::from(delta_y(r)) + 1 - i16::from(size);
    let temp_min_x = i16::from(r.min.x) + diff_x / 2;
    let temp_min_y = i16::from(r.min.y) + diff_y / 2;
    let clamped_min_x = temp_min_x.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(size) + 1);
    let clamped_min_y = temp_min_y.clamp(i16::from(i8::MIN), i16::from(i8::MAX) - i16::from(size) + 1);
    let min_x = clamped_min_x as i8;
    let min_y = clamped_min_y as i8;
    let max_x = (clamped_min_x + i16::from(size) - 1) as i8;
    let max_y = (clamped_min_y + i16::from(size) - 1) as i8;
    Some(RectI8 { min: PointI8 { x: min_x, y: min_y }, max: PointI8 { x: max_x, y: max_y } })
}

pub fn assign_resize(r: &mut RectI8, size: u8) {
    try_assign_resize(r, size).unwrap()
}

pub fn resize(r: &RectI8, size: u8) -> RectI8 {
    try_resize(r, size).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::cartesian::rect::rect_i8::RectI8;

    use super::{ assign_resize, resize, try_assign_resize, try_resize};

    #[test]
    fn try_assign_resize_odd() {
        let mut r = RectI8::of(-5, -5, 5, 5);
        assert_eq!(try_assign_resize(&mut r, 11), Some(()));
        assert_eq!(r, RectI8::of(-5, -5, 5, 5));
        assert_eq!(try_assign_resize(&mut r, 9), Some(()));
        assert_eq!(r, RectI8::of(-4, -4, 4, 4));
        assert_eq!(try_assign_resize(&mut r, 7), Some(()));
        assert_eq!(r, RectI8::of(-3, -3, 3, 3));
        assert_eq!(try_assign_resize(&mut r, 5), Some(()));
        assert_eq!(r, RectI8::of(-2, -2, 2, 2));
        assert_eq!(try_assign_resize(&mut r, 3), Some(()));
        assert_eq!(r, RectI8::of(-1, -1, 1, 1));
        assert_eq!(try_assign_resize(&mut r, 9), Some(()));
        assert_eq!(r, RectI8::of(-4, -4, 4, 4));
    }

    #[test]
    fn try_assign_resize_even() {
        let mut r = RectI8::of(-5, -5, 4, 4);
        assert_eq!(try_assign_resize(&mut r, 10), Some(()));
        assert_eq!(r, RectI8::of(-5, -5, 4, 4));
        assert_eq!(try_assign_resize(&mut r, 8), Some(()));
        assert_eq!(r, RectI8::of(-4, -4, 3, 3));
        assert_eq!(try_assign_resize(&mut r, 6), Some(()));
        assert_eq!(r, RectI8::of(-3, -3, 2, 2));
        assert_eq!(try_assign_resize(&mut r, 4), Some(()));
        assert_eq!(r, RectI8::of(-2, -2, 1, 1));
        assert_eq!(try_assign_resize(&mut r, 8), Some(()));
        assert_eq!(r, RectI8::of(-4, -4, 3, 3));
    }

    #[test]
    fn try_assign_resize_small_size() {
        let mut r = RectI8::of(10, 10, 20, 20);
        assert_eq!(try_assign_resize(&mut r, 0), None);
        assert_eq!(try_assign_resize(&mut r, 1), None);
        assert_eq!(try_assign_resize(&mut r, 2), None);
        assert_eq!(r, RectI8::of(10, 10, 20, 20));
    }

    #[test]
    fn try_assign_resize_same_size() {
        let mut r_11 = RectI8::of(10, 10, 20, 20);
        assert_eq!(try_assign_resize(&mut r_11, 11), Some(()));
        assert_eq!(r_11, RectI8::of(10, 10, 20, 20));

        let mut r_12 = RectI8::of(10, 10, 21, 21);
        assert_eq!(try_assign_resize(&mut r_12, 12), Some(()));
        assert_eq!(r_12, RectI8::of(10, 10, 21, 21));

        let mut r_13 = RectI8::of(9, 9, 21, 21);
        assert_eq!(try_assign_resize(&mut r_13, 13), Some(()));
        assert_eq!(r_13, RectI8::of(9, 9, 21, 21));
    }

    #[test]
    fn try_assign_resize_odd_small_rect_out_of_bounds() {
        let mut r_min = RectI8::of(i8::MIN, i8::MIN, i8::MIN + 2, i8::MIN + 2);
        assert_eq!(try_assign_resize(&mut r_min, 11), Some(()));
        assert_eq!(r_min, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 10));

        let mut r_max = RectI8::of(i8::MAX - 2, i8::MAX - 2, i8::MAX, i8::MAX);
        assert_eq!(try_assign_resize(&mut r_max, 11), Some(()));
        assert_eq!(r_max, RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX, i8::MAX));
    }

    #[test]
    fn try_assign_resize_even_small_rect_out_of_bounds() {
        let mut r = RectI8::of(i8::MIN, i8::MIN, i8::MIN + 3, i8::MIN + 3);
        assert_eq!(try_assign_resize(&mut r, 11), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 10));

        let mut r = RectI8::of(i8::MAX - 3, i8::MAX - 3, i8::MAX, i8::MAX);
        assert_eq!(try_assign_resize(&mut r, 11), Some(()));
        assert_eq!(r, RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX, i8::MAX));
    }

    #[test]
    fn try_assign_resize_odd_small_rect_limits_out_of_bounds() {
        let mut r_min = RectI8::of(i8::MIN, i8::MIN, i8::MIN + 2, i8::MIN + 2);
        assert_eq!(try_assign_resize(&mut r_min, u8::MAX), Some(()));
        assert_eq!(r_min, RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1));

        let mut r_max = RectI8::of(i8::MAX - 2, i8::MAX - 2, i8::MAX, i8::MAX);
        assert_eq!(try_assign_resize(&mut r_max, u8::MAX), Some(()));
        assert_eq!(r_max, RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX));
    }

    #[test]
    fn try_assign_resize_even_small_rect_limits_out_of_bounds() {
        let mut r = RectI8::of(i8::MIN, i8::MIN, i8::MIN + 3, i8::MIN + 3);
        assert_eq!(try_assign_resize(&mut r, u8::MAX - 1), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 2));

        let mut r = RectI8::of(i8::MAX - 3, i8::MAX - 3, i8::MAX, i8::MAX);
        assert_eq!(try_assign_resize(&mut r, u8::MAX - 1), Some(()));
        assert_eq!(r, RectI8::of(i8::MIN + 2, i8::MIN + 2, i8::MAX, i8::MAX));
    }

    #[test]
    fn try_assign_resize_big_rect_limits_out_of_bounds() {
        let mut r_odd_1 = RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1);
        assert_eq!(try_assign_resize(&mut r_odd_1, u8::MAX), Some(()));
        assert_eq!(r_odd_1, RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1));

        let mut r_odd_1 = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX);
        assert_eq!(try_assign_resize(&mut r_odd_1, u8::MAX), Some(()));
        assert_eq!(r_odd_1, RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX));

        let mut r_even = RectI8::largest();
        assert_eq!(try_assign_resize(&mut r_even, u8::MAX), Some(()));
        assert_eq!(r_even, RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1));
    }

    #[test]
    fn try_resize_odd() {
        assert_eq!(try_resize(&RectI8::of(-5, -5, 5, 5), 11), Some(RectI8::of(-5, -5, 5, 5)));
        assert_eq!(try_resize(&RectI8::of(-5, -5, 5, 5), 9), Some(RectI8::of(-4, -4, 4, 4)));
        assert_eq!(try_resize(&RectI8::of(-4, -4, 4, 4), 7), Some(RectI8::of(-3, -3, 3, 3)));
        assert_eq!(try_resize(&RectI8::of(-3, -3, 3, 3), 5), Some(RectI8::of(-2, -2, 2, 2)));
        assert_eq!(try_resize(&RectI8::of(-2, -2, 2, 2), 3), Some(RectI8::of(-1, -1, 1, 1)));
        assert_eq!(try_resize(&RectI8::of(-1, -1, 1, 1), 9), Some(RectI8::of(-4, -4, 4, 4)));
    }

    #[test]
    fn try_resize_even() {
        assert_eq!(try_resize(&RectI8::of(-5, -5, 4, 4), 10), Some(RectI8::of(-5, -5, 4, 4)));
        assert_eq!(try_resize(&RectI8::of(-5, -5, 4, 4), 8), Some(RectI8::of(-4, -4, 3, 3)));
        assert_eq!(try_resize(&RectI8::of(-4, -4, 3, 3), 6), Some(RectI8::of(-3, -3, 2, 2)));
        assert_eq!(try_resize(&RectI8::of(-3, -3, 2, 2), 4), Some(RectI8::of(-2, -2, 1, 1)));
        assert_eq!(try_resize(&RectI8::of(-2, -2, 1, 1), 8), Some(RectI8::of(-4, -4, 3, 3)));
    }

    #[test]
    fn try_resize_small_size() {
        let r = RectI8::of(10, 10, 20, 20);
        assert_eq!(try_resize(&r, 0), None);
        assert_eq!(try_resize(&r, 1), None);
        assert_eq!(try_resize(&r, 2), None);
    }

    #[test]
    fn try_resize_same_size() {
        assert_eq!(try_resize(&RectI8::of(10, 10, 20, 20), 11), Some(RectI8::of(10, 10, 20, 20)));
        assert_eq!(try_resize(&RectI8::of(10, 10, 21, 21), 12), Some(RectI8::of(10, 10, 21, 21)));
        assert_eq!(try_resize(&RectI8::of(9, 9, 21, 21), 13), Some(RectI8::of(9, 9, 21, 21)));
    }

    #[test]
    fn try_resize_odd_small_rect_out_of_bounds() {
        assert_eq!(
            try_resize(&RectI8::of(i8::MIN, i8::MIN, i8::MIN + 2, i8::MIN + 2), 11),
            Some(RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 10))
        );
        assert_eq!(
            try_resize(&RectI8::of(i8::MAX - 2, i8::MAX - 2, i8::MAX, i8::MAX), 11),
            Some(RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX, i8::MAX))
        );
    }

    #[test]
    fn try_resize_even_small_rect_out_of_bounds() {
        assert_eq!(
            try_resize(&RectI8::of(i8::MIN, i8::MIN, i8::MIN + 3, i8::MIN + 3), 11),
            Some(RectI8::of(i8::MIN, i8::MIN, i8::MIN + 10, i8::MIN + 10))
        );
        assert_eq!(
            try_resize(&RectI8::of(i8::MAX - 3, i8::MAX - 3, i8::MAX, i8::MAX), 11),
            Some(RectI8::of(i8::MAX - 10, i8::MAX - 10, i8::MAX, i8::MAX))
        );
    }

    #[test]
    fn try_resize_odd_small_rect_limits_out_of_bounds() {
        assert_eq!(
            try_resize(&RectI8::of(i8::MIN, i8::MIN, i8::MIN + 2, i8::MIN + 2), u8::MAX),
            Some(RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1))
        );
        assert_eq!(
            try_resize(&RectI8::of(i8::MAX - 2, i8::MAX - 2, i8::MAX, i8::MAX), u8::MAX),
            Some(RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX))
        );
    }

    #[test]
    fn try_resize_even_small_rect_limits_out_of_bounds() {
        assert_eq!(
            try_resize(&RectI8::of(i8::MIN, i8::MIN, i8::MIN + 3, i8::MIN + 3), u8::MAX - 1),
            Some(RectI8::of(i8::MIN, i8::MIN, i8::MAX - 2, i8::MAX - 2))
        );
        assert_eq!(
            try_resize(&RectI8::of(i8::MAX - 3, i8::MAX - 3, i8::MAX, i8::MAX), u8::MAX - 1),
            Some(RectI8::of(i8::MIN + 2, i8::MIN + 2, i8::MAX, i8::MAX))
        );
    }

    #[test]
    fn try_resize_big_rect_limits_out_of_bounds() {
        assert_eq!(
            try_resize(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1), u8::MAX),
            Some(RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1))
        );
        assert_eq!(
            try_resize(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX), u8::MAX),
            Some(RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX))
        );
        assert_eq!(try_resize(&RectI8::largest(), u8::MAX), Some(RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1)));
    }

    #[test]
    fn assign_resize_odd() {
        let mut r = RectI8::of(-5, -5, 5, 5);
        assign_resize(&mut r, 11);
        assert_eq!(r, RectI8::of(-5, -5, 5, 5));
        assign_resize(&mut r, 9);
        assert_eq!(r, RectI8::of(-4, -4, 4, 4));
        assign_resize(&mut r, 7);
        assert_eq!(r, RectI8::of(-3, -3, 3, 3));
        assign_resize(&mut r, 5);
        assert_eq!(r, RectI8::of(-2, -2, 2, 2));
        assign_resize(&mut r, 3);
        assert_eq!(r, RectI8::of(-1, -1, 1, 1));
        assign_resize(&mut r, 9);
        assert_eq!(r, RectI8::of(-4, -4, 4, 4));
    }

    #[test]
    fn assign_resize_even() {
        let mut r = RectI8::of(-5, -5, 4, 4);
        assign_resize(&mut r, 10);
        assert_eq!(r, RectI8::of(-5, -5, 4, 4));
        assign_resize(&mut r, 8);
        assert_eq!(r, RectI8::of(-4, -4, 3, 3));
        assign_resize(&mut r, 6);
        assert_eq!(r, RectI8::of(-3, -3, 2, 2));
        assign_resize(&mut r, 4);
        assert_eq!(r, RectI8::of(-2, -2, 1, 1));
        assign_resize(&mut r, 8);
        assert_eq!(r, RectI8::of(-4, -4, 3, 3));
    }

    #[test]
    fn resize_odd() {
        assert_eq!(resize(&RectI8::of(-5, -5, 5, 5), 11), RectI8::of(-5, -5, 5, 5));
        assert_eq!(resize(&RectI8::of(-5, -5, 5, 5), 9), RectI8::of(-4, -4, 4, 4));
        assert_eq!(resize(&RectI8::of(-4, -4, 4, 4), 7), RectI8::of(-3, -3, 3, 3));
        assert_eq!(resize(&RectI8::of(-3, -3, 3, 3), 5), RectI8::of(-2, -2, 2, 2));
        assert_eq!(resize(&RectI8::of(-2, -2, 2, 2), 3), RectI8::of(-1, -1, 1, 1));
        assert_eq!(resize(&RectI8::of(-1, -1, 1, 1), 9), RectI8::of(-4, -4, 4, 4));
    }

    #[test]
    fn resize_even() {
        assert_eq!(resize(&RectI8::of(-5, -5, 4, 4), 10), RectI8::of(-5, -5, 4, 4));
        assert_eq!(resize(&RectI8::of(-5, -5, 4, 4), 8), RectI8::of(-4, -4, 3, 3));
        assert_eq!(resize(&RectI8::of(-4, -4, 3, 3), 6), RectI8::of(-3, -3, 2, 2));
        assert_eq!(resize(&RectI8::of(-3, -3, 2, 2), 4), RectI8::of(-2, -2, 1, 1));
        assert_eq!(resize(&RectI8::of(-2, -2, 1, 1), 8), RectI8::of(-4, -4, 3, 3));
    }
}
