use std::ops::RangeInclusive;

use crate::cartesian::point::{point_i8::PointI8, point_u8};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct RectU8 {
    pub min: point_u8::PointU8,
    pub max: point_u8::PointU8,
}

impl RectU8 {
    pub fn of(x1: u8, y1: u8, x2: u8, y2: u8) -> Self {
        RectU8 { min: point_u8::PointU8::of(x1, y1), max: point_u8::PointU8::of(x2, y2) }
    }

    pub fn iter_x(&self) -> RangeInclusive<u8> {
        self.min.x..=self.max.x
    }

    pub fn iter_y(&self) -> RangeInclusive<u8> {
        self.min.y..=self.max.y
    }
}

impl std::fmt::Display for RectU8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_x(r: &RectU8) -> u8 {
    point_u8::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectU8) -> u8 {
    point_u8::delta_y(&r.min, &r.max)
}

pub fn max_delta(r: &RectU8) -> u8 {
    std::cmp::max(delta_x(r), delta_y(r))
}

pub fn len_x(r: &RectU8) -> u8 {
    delta_x(r) + 1
}

pub fn len_y(r: &RectU8) -> u8 {
    delta_y(r) + 1
}

pub fn max_len(r: &RectU8) -> u8 {
    std::cmp::max(len_x(r), len_y(r))
}

pub fn inflate(r: &mut RectU8) {
    let is_min_x = r.min.x == 0;
    let is_min_y = r.min.y == 0;
    let is_max_x = r.max.x == u8::MAX;
    let is_max_y = r.max.y == u8::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return;
    }
    let min_x_modifier = 1 - u8::from(is_min_x) + u8::from(is_max_x);
    let min_y_modifier = 1 - u8::from(is_min_y) + u8::from(is_max_y);
    let max_x_modifier = 1 + u8::from(is_min_x) - u8::from(is_max_x);
    let max_y_modifier = 1 + u8::from(is_min_y) - u8::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
}

pub fn deflate(r: &mut RectU8) {
    if delta_x(r) < 3 || delta_y(r) < 3 {
        return;
    }
    r.min.x += 1;
    r.min.y += 1;
    r.max.x -= 1;
    r.max.y -= 1;
}

pub fn resize(r: &mut RectU8, size: u8) {
    if size < 3 {
        return;
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
}

pub fn saturating_translate(r: &mut RectU8, delta: &PointI8) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i16::from(r.min.x) + i16::from(delta.x);
    let temp_min_y = i16::from(r.min.y) + i16::from(delta.y);
    let clamped_x = temp_min_x.clamp(0, i16::from(u8::MAX) - i16::from(dx));
    let clamped_y = temp_min_y.clamp(0, i16::from(u8::MAX) - i16::from(dy));
    let min_x = clamped_x as u8;
    let min_y = clamped_y as u8;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = min_x + dx;
    r.max.y = min_y + dy;
}

pub fn try_checked_translate(r: &mut RectU8, delta: &PointI8) -> Result<(), ()> {
    let min_x = u8::try_from(i16::from(r.min.x) + i16::from(delta.x)).map_err(|_| ())?;
    let min_y = u8::try_from(i16::from(r.min.y) + i16::from(delta.y)).map_err(|_| ())?;
    let max_x = u8::try_from(i16::from(r.max.x) + i16::from(delta.x)).map_err(|_| ())?;
    let max_y = u8::try_from(i16::from(r.max.y) + i16::from(delta.y)).map_err(|_| ())?;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Ok(())
}

pub fn checked_translate(r: &mut RectU8, delta: &PointI8) {
    try_checked_translate(r, delta).unwrap()
}

pub fn contains(r: &RectU8, p: &point_u8::PointU8) -> bool {
    p.x >= r.min.x && p.x <= r.max.x && p.y >= r.min.y && p.y <= r.max.y
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::{point_i8::PointI8, point_u8::PointU8};

    use super::{
        RectU8, contains, deflate, delta_x, delta_y, inflate, len_x, len_y, max_delta, max_len, resize, saturating_translate, try_checked_translate,
        checked_translate,
    };

    #[test]
    fn rect_u8() {
        assert_eq!(RectU8::of(0, 2, 4, 8), RectU8 { min: PointU8 { x: 0, y: 2 }, max: PointU8 { x: 4, y: 8 } });
        assert_eq!(RectU8::of(u8::MAX, 0, 0, u8::MAX).to_string(), "((255, 0), (0, 255))");
    }

    #[test]
    fn iter_x() {
        assert_eq!(RectU8::of(3, 6, 2, 8).iter_x().collect::<Vec<u8>>(), []);
        assert_eq!(RectU8::of(3, 6, 3, 8).iter_x().collect::<Vec<u8>>(), [3]);
        assert_eq!(RectU8::of(3, 6, 4, 8).iter_x().collect::<Vec<u8>>(), [3, 4]);
        assert_eq!(RectU8::of(3, 6, 5, 8).iter_x().collect::<Vec<u8>>(), [3, 4, 5]);
        assert_eq!(RectU8::of(3, 6, 6, 8).iter_x().collect::<Vec<u8>>(), [3, 4, 5, 6]);
        assert_eq!(RectU8::of(3, 6, 6, 8).iter_x().rev().collect::<Vec<u8>>(), [6, 5, 4, 3]);
        assert_eq!(RectU8::of(3, 6, 5, 8).iter_x().rev().collect::<Vec<u8>>(), [5, 4, 3]);
        assert_eq!(RectU8::of(3, 6, 4, 8).iter_x().rev().collect::<Vec<u8>>(), [4, 3]);
        assert_eq!(RectU8::of(3, 6, 3, 8).iter_x().rev().collect::<Vec<u8>>(), [3]);
        assert_eq!(RectU8::of(3, 6, 2, 8).iter_x().rev().collect::<Vec<u8>>(), []);
    }

    #[test]
    fn iter_y() {
        assert_eq!(RectU8::of(3, 6, 4, 5).iter_y().collect::<Vec<u8>>(), []);
        assert_eq!(RectU8::of(3, 6, 4, 6).iter_y().collect::<Vec<u8>>(), [6]);
        assert_eq!(RectU8::of(3, 6, 4, 7).iter_y().collect::<Vec<u8>>(), [6, 7]);
        assert_eq!(RectU8::of(3, 6, 4, 8).iter_y().collect::<Vec<u8>>(), [6, 7, 8]);
        assert_eq!(RectU8::of(3, 6, 4, 9).iter_y().collect::<Vec<u8>>(), [6, 7, 8, 9]);
        assert_eq!(RectU8::of(3, 6, 4, 9).iter_y().rev().collect::<Vec<u8>>(), [9, 8, 7, 6]);
        assert_eq!(RectU8::of(3, 6, 4, 8).iter_y().rev().collect::<Vec<u8>>(), [8, 7, 6]);
        assert_eq!(RectU8::of(3, 6, 4, 7).iter_y().rev().collect::<Vec<u8>>(), [7, 6]);
        assert_eq!(RectU8::of(3, 6, 4, 6).iter_y().rev().collect::<Vec<u8>>(), [6]);
        assert_eq!(RectU8::of(3, 6, 4, 5).iter_y().rev().collect::<Vec<u8>>(), []);
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectU8::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_x(&RectU8::of(0, 0, u8::MAX, 0)), u8::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectU8::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_y(&RectU8::of(0, 0, 0, u8::MAX)), u8::MAX);
    }

    #[test]
    fn test_max_delta() {
        assert_eq!(max_delta(&RectU8::of(0, 5, 10, 10)), 10);
        assert_eq!(max_delta(&RectU8::of(5, 0, 9, 9)), 9);
    }

    #[test]
    fn max_delta_0() {
        assert_eq!(max_delta(&RectU8::of(0, 0, 0, 0)), 0);
        assert_eq!(max_delta(&RectU8::of(1, 1, 1, 1)), 0);
        assert_eq!(max_delta(&RectU8::of(5, 10, 5, 10)), 0);
    }

    #[test]
    fn max_delta_1() {
        assert_eq!(max_delta(&RectU8::of(0, 0, 1, 1)), 1);
        assert_eq!(max_delta(&RectU8::of(5, 5, 6, 6)), 1);
        assert_eq!(max_delta(&RectU8::of(0, 0, 0, 1)), 1);
        assert_eq!(max_delta(&RectU8::of(5, 9, 5, 10)), 1);
    }

    #[test]
    fn max_delta_bounds() {
        assert_eq!(max_delta(&RectU8::of(0, 0, u8::MAX, u8::MAX - 1)), u8::MAX);
        assert_eq!(max_delta(&RectU8::of(0, 0, u8::MAX - 1, u8::MAX)), u8::MAX);
    }

    #[test]
    fn test_len_x() {
        assert_eq!(len_x(&RectU8::of(0, 0, 0, 0)), 1);
        assert_eq!(len_x(&RectU8::of(0, 0, u8::MAX - 1, 0)), u8::MAX);
    }

    #[test]
    fn test_len_y() {
        assert_eq!(len_y(&RectU8::of(0, 0, 0, 0)), 1);
        assert_eq!(len_y(&RectU8::of(0, 0, 0, u8::MAX - 1)), u8::MAX);
    }

    #[test]
    fn test_max_len() {
        assert_eq!(max_len(&RectU8::of(0, 5, 10, 10)), 11);
        assert_eq!(max_len(&RectU8::of(5, 0, 9, 9)), 10);
    }

    #[test]
    fn max_len_1() {
        assert_eq!(max_len(&RectU8::of(0, 0, 0, 0)), 1);
        assert_eq!(max_len(&RectU8::of(1, 1, 1, 1)), 1);
        assert_eq!(max_len(&RectU8::of(5, 10, 5, 10)), 1);
    }

    #[test]
    fn max_len_2() {
        assert_eq!(max_len(&RectU8::of(0, 0, 1, 1)), 2);
        assert_eq!(max_len(&RectU8::of(5, 5, 6, 6)), 2);
        assert_eq!(max_len(&RectU8::of(0, 0, 0, 1)), 2);
        assert_eq!(max_len(&RectU8::of(5, 9, 5, 10)), 2);
    }

    #[test]
    fn max_len_bounds() {
        assert_eq!(max_len(&RectU8::of(1, 0, u8::MAX - 1, u8::MAX - 1)), u8::MAX);
        assert_eq!(max_len(&RectU8::of(0, 1, u8::MAX - 1, u8::MAX - 1)), u8::MAX);
        assert_eq!(max_len(&RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 1)), u8::MAX);
        assert_eq!(max_len(&RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 2)), u8::MAX);
    }

    #[test]
    fn inflate_min_bounds() {
        let mut r = RectU8::of(7, 2, 4, 13);
        inflate(&mut r);
        assert_eq!(r, RectU8::of(6, 1, 5, 14));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(5, 0, 6, 15));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(4, 0, 7, 17));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(3, 0, 8, 19));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(2, 0, 9, 21));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(1, 0, 10, 23));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(0, 0, 11, 25));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(0, 0, 13, 27));
    }

    #[test]
    fn inflate_max_bounds() {
        let mut r = RectU8::of(200, 230, u8::MAX - 5, u8::MAX - 3);
        inflate(&mut r);
        assert_eq!(r, RectU8::of(199, 229, u8::MAX - 4, u8::MAX - 2));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(198, 228, u8::MAX - 3, u8::MAX - 1));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(197, 227, u8::MAX - 2, u8::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(196, 225, u8::MAX - 1, u8::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(195, 223, u8::MAX, u8::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(193, 221, u8::MAX, u8::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(191, 219, u8::MAX, u8::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU8::of(189, 217, u8::MAX, u8::MAX));
    }

    #[test]
    fn inflate_almost_min_bounds() {
        let mut r = RectU8::of(1, 1, u8::MAX, u8::MAX);
        inflate(&mut r);
        assert_eq!(r, RectU8::of(0, 0, u8::MAX, u8::MAX));
    }

    #[test]
    fn inflate_almost_max_bounds() {
        let mut r = RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1);
        inflate(&mut r);
        assert_eq!(r, RectU8::of(0, 0, u8::MAX, u8::MAX));
    }

    #[test]
    fn inflate_max_width() {
        let mut r = RectU8::of(0, 10, u8::MAX, 250);
        inflate(&mut r);
        assert_eq!(r, RectU8::of(0, 10, u8::MAX, 250));
    }

    #[test]
    fn inflate_max_height() {
        let mut r = RectU8::of(10, 0, 250, u8::MAX);
        inflate(&mut r);
        assert_eq!(r, RectU8::of(10, 0, 250, u8::MAX));
    }

    #[test]
    fn deflate_odd() {
        let mut r = RectU8::of(0, 0, 9, 9);
        deflate(&mut r);
        assert_eq!(r, RectU8::of(1, 1, 8, 8));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(2, 2, 7, 7));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(3, 3, 6, 6));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(4, 4, 5, 5));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(4, 4, 5, 5));
    }

    #[test]
    fn deflate_even() {
        let mut r = RectU8::of(0, 0, 10, 10);
        deflate(&mut r);
        assert_eq!(r, RectU8::of(1, 1, 9, 9));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(2, 2, 8, 8));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(3, 3, 7, 7));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(4, 4, 6, 6));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(4, 4, 6, 6));
    }

    #[test]
    fn resize_odd() {
        let mut r = RectU8::of(5, 5, 15, 15);
        resize(&mut r, 11);
        assert_eq!(r, RectU8::of(5, 5, 15, 15));
        resize(&mut r, 9);
        assert_eq!(r, RectU8::of(6, 6, 14, 14));
        resize(&mut r, 7);
        assert_eq!(r, RectU8::of(7, 7, 13, 13));
        resize(&mut r, 5);
        assert_eq!(r, RectU8::of(8, 8, 12, 12));
        resize(&mut r, 3);
        assert_eq!(r, RectU8::of(9, 9, 11, 11));
        resize(&mut r, 1);
        assert_eq!(r, RectU8::of(9, 9, 11, 11));
        resize(&mut r, 3);
        assert_eq!(r, RectU8::of(9, 9, 11, 11));
        resize(&mut r, 9);
        assert_eq!(r, RectU8::of(6, 6, 14, 14));
    }

    #[test]
    fn resize_even() {
        let mut r = RectU8::of(5, 5, 14, 14);
        resize(&mut r, 10);
        assert_eq!(r, RectU8::of(5, 5, 14, 14));
        resize(&mut r, 8);
        assert_eq!(r, RectU8::of(6, 6, 13, 13));
        resize(&mut r, 6);
        assert_eq!(r, RectU8::of(7, 7, 12, 12));
        resize(&mut r, 4);
        assert_eq!(r, RectU8::of(8, 8, 11, 11));
        resize(&mut r, 2);
        assert_eq!(r, RectU8::of(8, 8, 11, 11));
        resize(&mut r, 4);
        assert_eq!(r, RectU8::of(8, 8, 11, 11));
        resize(&mut r, 8);
        assert_eq!(r, RectU8::of(6, 6, 13, 13));
    }

    #[test]
    fn resize_odd_min_bounds_big_delta() {
        let mut r = RectU8::of(0, 0, 2, 2);
        resize(&mut r, u8::MAX);
        assert_eq!(r, RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1));
    }

    #[test]
    fn resize_even_min_bounds_big_delta() {
        let mut r = RectU8::of(0, 0, 3, 3);
        resize(&mut r, u8::MAX - 1);
        assert_eq!(r, RectU8::of(0, 0, u8::MAX - 2, u8::MAX - 2));
    }

    #[test]
    fn resize_odd_max_bounds_big_delta() {
        let mut r = RectU8::of(u8::MAX - 2, u8::MAX - 2, u8::MAX, u8::MAX);
        resize(&mut r, u8::MAX);
        assert_eq!(r, RectU8::of(1, 1, u8::MAX, u8::MAX));
    }

    #[test]
    fn resize_even_max_bounds_big_delta() {
        let mut r = RectU8::of(u8::MAX - 3, u8::MAX - 3, u8::MAX, u8::MAX);
        resize(&mut r, u8::MAX - 1);
        assert_eq!(r, RectU8::of(2, 2, u8::MAX, u8::MAX));
    }

    #[test]
    fn test_saturating_translate() {
        let mut r = RectU8::of(0, 0, 12, 15);
        saturating_translate(&mut r, &PointI8::of(5, 4));
        assert_eq!(r, RectU8::of(5, 4, 17, 19));
        saturating_translate(&mut r, &PointI8::of(-4, -2));
        assert_eq!(r, RectU8::of(1, 2, 13, 17));
        saturating_translate(&mut r, &PointI8::of(10, 20));
        assert_eq!(r, RectU8::of(11, 22, 23, 37));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut r = RectU8::of(2, 5, 12, 15);
        saturating_translate(&mut r, &PointI8::of(-10, -10));
        assert_eq!(r, RectU8::of(0, 0, 10, 10));
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut r = RectU8::of(240, 235, u8::MAX - 5, u8::MAX - 10);
        saturating_translate(&mut r, &PointI8::of(20, 20));
        assert_eq!(r, RectU8::of(245, 245, u8::MAX, u8::MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_big_delta() {
        let mut r = RectU8::of(0, 0, 10, 10);
        saturating_translate(&mut r, &PointI8::min());
        assert_eq!(r, RectU8::of(0, 0, 10, 10));
    }

    #[test]
    fn saturating_translate_max_bounds_big_delta() {
        let mut r = RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX, u8::MAX);
        saturating_translate(&mut r, &PointI8::max());
        assert_eq!(r, RectU8::of(u8::MAX - 10, u8::MAX - 10, u8::MAX, u8::MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_big_rect_big_delta() {
        let mut r = RectU8::of(1, 1, u8::MAX, u8::MAX);
        saturating_translate(&mut r, &PointI8::min());
        assert_eq!(r, RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1));
    }

    #[test]
    fn saturating_translate_max_bounds_big_rect_big_delta() {
        let mut r = RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1);
        saturating_translate(&mut r, &PointI8::max());
        assert_eq!(r, RectU8::of(1, 1, u8::MAX, u8::MAX));
    }

    #[test]
    fn test_try_checked_translate() {
        let mut r = RectU8::of(0, 0, 12, 15);
        assert_eq!(try_checked_translate(&mut r, &PointI8::of(5, 4)), Ok(()));
        assert_eq!(r, RectU8::of(5, 4, 17, 19));
        assert_eq!(try_checked_translate(&mut r, &PointI8::of(-4, -2)), Ok(()));
        assert_eq!(r, RectU8::of(1, 2, 13, 17));
        assert_eq!(try_checked_translate(&mut r, &PointI8::of(10, 20)), Ok(()));
        assert_eq!(r, RectU8::of(11, 22, 23, 37));
    }

    #[test]
    fn try_checked_translate_min_bounds() {
        let mut r = RectU8::of(2, 5, 12, 15);
        assert_eq!(try_checked_translate(&mut r, &PointI8::of(-10, -10)), Err(()));
        assert_eq!(r, RectU8::of(2, 5, 12, 15));
    }

    #[test]
    fn try_checked_translate_max_bounds() {
        let mut r = RectU8::of(240, 235, u8::MAX - 5, u8::MAX - 10);
        assert_eq!(try_checked_translate(&mut r, &PointI8::of(20, 20)), Err(()));
        assert_eq!(r, RectU8::of(240, 235, u8::MAX - 5, u8::MAX - 10));
    }

    #[test]
    fn try_checked_translate_min_bounds_big_rect_big_delta() {
        let mut r = RectU8::of(1, 1, u8::MAX, u8::MAX);
        assert_eq!(try_checked_translate(&mut r, &PointI8::min()), Err(()));
        assert_eq!(try_checked_translate(&mut r, &PointI8::of(i8::MIN, 0)), Err(()));
        assert_eq!(try_checked_translate(&mut r, &PointI8::of(0, i8::MIN)), Err(()));
        assert_eq!(r, RectU8::of(1, 1, u8::MAX, u8::MAX));
    }

    #[test]
    fn try_checked_translate_max_bounds_big_rect_big_delta() {
        let mut r = RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1);
        assert_eq!(try_checked_translate(&mut r, &PointI8::max()), Err(()));
        assert_eq!(try_checked_translate(&mut r, &PointI8::of(i8::MAX, 0)), Err(()));
        assert_eq!(try_checked_translate(&mut r, &PointI8::of(0, i8::MAX)), Err(()));
        assert_eq!(r, RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1));
    }

    #[test]
    fn try_checked_translate_min_bounds_big_rect_small_delta() {
        let mut r = RectU8::of(1, 1, u8::MAX, u8::MAX);
        assert_eq!(try_checked_translate(&mut r, &PointI8::of(-1, -1)), Ok(()));
        assert_eq!(r, RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1));
    }

    #[test]
    fn try_checked_translate_max_bounds_big_rect_small_delta() {
        let mut r = RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1);
        assert_eq!(try_checked_translate(&mut r, &PointI8::of(1, 1)), Ok(()));
        assert_eq!(r, RectU8::of(1, 1, u8::MAX, u8::MAX));
    }

    #[test]
    fn test_checked_translate() {
        let mut r = RectU8::of(0, 0, 12, 15);
        checked_translate(&mut r, &PointI8::of(5, 4));
        assert_eq!(r, RectU8::of(5, 4, 17, 19));
        checked_translate(&mut r, &PointI8::of(-4, -2));
        assert_eq!(r, RectU8::of(1, 2, 13, 17));
        checked_translate(&mut r, &PointI8::of(10, 20));
        assert_eq!(r, RectU8::of(11, 22, 23, 37));
    }

    #[test]
    fn contains_inside_borders() {
        let r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert!(contains(&r, &PointU8::of(1, 1)));
        assert!(contains(&r, &PointU8::of(1, u8::MAX - 1)));
        assert!(contains(&r, &PointU8::of(u8::MAX - 1, 1)));
        assert!(contains(&r, &PointU8::of(u8::MAX - 1, u8::MAX - 1)));
    }

    #[test]
    fn contains_outside_borders() {
        let r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert!(!contains(&r, &PointU8::of(0, 0)));
        assert!(!contains(&r, &PointU8::of(0, u8::MAX)));
        assert!(!contains(&r, &PointU8::of(u8::MAX, 0)));
        assert!(!contains(&r, &PointU8::max()));
        assert!(!contains(&r, &PointU8::of(1, 0)));
        assert!(!contains(&r, &PointU8::of(1, u8::MAX)));
        assert!(!contains(&r, &PointU8::of(u8::MAX - 1, 0)));
        assert!(!contains(&r, &PointU8::of(u8::MAX - 1, u8::MAX)));
        assert!(!contains(&r, &PointU8::of(0, 1)));
        assert!(!contains(&r, &PointU8::of(0, u8::MAX - 1)));
        assert!(!contains(&r, &PointU8::of(u8::MAX, 1)));
        assert!(!contains(&r, &PointU8::of(u8::MAX, u8::MAX - 1)));
    }

    #[test]
    fn contains_inside() {
        let r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert!(contains(&r, &PointU8::of(10, 10)));
        assert!(contains(&r, &PointU8::of(u8::MAX - 10, u8::MAX - 10)));
    }
}
