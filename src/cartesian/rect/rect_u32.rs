use std::ops::RangeInclusive;

use crate::cartesian::point::{point_i32::PointI32, point_u32};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct RectU32 {
    pub min: point_u32::PointU32,
    pub max: point_u32::PointU32,
}

impl RectU32 {
    pub fn of(x1: u32, y1: u32, x2: u32, y2: u32) -> Self {
        RectU32 { min: point_u32::PointU32::of(x1, y1), max: point_u32::PointU32::of(x2, y2) }
    }

    pub fn iter_x(&self) -> RangeInclusive<u32> {
        self.min.x..=self.max.x
    }

    pub fn iter_y(&self) -> RangeInclusive<u32> {
        self.min.y..=self.max.y
    }
}

impl std::fmt::Display for RectU32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_x(r: &RectU32) -> u32 {
    point_u32::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectU32) -> u32 {
    point_u32::delta_y(&r.min, &r.max)
}

pub fn max_delta(r: &RectU32) -> u32 {
    std::cmp::max(delta_x(r), delta_y(r))
}

pub fn len_x(r: &RectU32) -> u32 {
    delta_x(r) + 1
}

pub fn len_y(r: &RectU32) -> u32 {
    delta_y(r) + 1
}

pub fn max_len(r: &RectU32) -> u32 {
    std::cmp::max(len_x(r), len_y(r))
}

pub fn inflate(r: &mut RectU32) {
    let is_min_x = r.min.x == 0;
    let is_min_y = r.min.y == 0;
    let is_max_x = r.max.x == u32::MAX;
    let is_max_y = r.max.y == u32::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return;
    }
    let min_x_modifier = 1 - u32::from(is_min_x) + u32::from(is_max_x);
    let min_y_modifier = 1 - u32::from(is_min_y) + u32::from(is_max_y);
    let max_x_modifier = 1 + u32::from(is_min_x) - u32::from(is_max_x);
    let max_y_modifier = 1 + u32::from(is_min_y) - u32::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
}

pub fn deflate(r: &mut RectU32) {
    if delta_x(r) < 3 || delta_y(r) < 3 {
        return;
    }
    r.min.x += 1;
    r.min.y += 1;
    r.max.x -= 1;
    r.max.y -= 1;
}

pub fn resize(r: &mut RectU32, size: u32) {
    if size < 3 {
        return;
    }
    let diff_x = i64::from(delta_x(r)) + 1 - i64::from(size);
    let diff_y = i64::from(delta_y(r)) + 1 - i64::from(size);
    let temp_min_x = i64::from(r.min.x) + diff_x / 2;
    let temp_min_y = i64::from(r.min.y) + diff_y / 2;
    let min_x = temp_min_x.clamp(0, i64::from(u32::MAX) - i64::from(size) + 1);
    let min_y = temp_min_y.clamp(0, i64::from(u32::MAX) - i64::from(size) + 1);
    r.min.x = min_x as u32;
    r.min.y = min_y as u32;
    r.max.x = (min_x + i64::from(size) - 1) as u32;
    r.max.y = (min_y + i64::from(size) - 1) as u32;
}

pub fn saturating_translate(r: &mut RectU32, delta: &PointI32) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i64::from(r.min.x) + i64::from(delta.x);
    let temp_min_y = i64::from(r.min.y) + i64::from(delta.y);
    let min_x = temp_min_x.clamp(0, i64::from(u32::MAX) - i64::from(dx)) as u32;
    let min_y = temp_min_y.clamp(0, i64::from(u32::MAX) - i64::from(dy)) as u32;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = min_x + dx;
    r.max.y = min_y + dy;
}

pub fn contains(r: &RectU32, p: &point_u32::PointU32) -> bool {
    p.x >= r.min.x && p.x <= r.max.x && p.y >= r.min.y && p.y <= r.max.y
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::{point_i32::PointI32, point_u32::PointU32};

    use super::{RectU32, contains, deflate, delta_x, delta_y, inflate, len_x, len_y, max_delta, max_len, resize, saturating_translate};

    #[test]
    fn rect_u32() {
        assert_eq!(RectU32::of(256, 512, 1024, 2048), RectU32 { min: PointU32 { x: 256, y: 512 }, max: PointU32 { x: 1024, y: 2048 } });
        assert_eq!(RectU32::of(u32::MAX, 0, 0, u32::MAX).to_string(), "((4294967295, 0), (0, 4294967295))");
    }

    #[test]
    fn iter_x() {
        assert_eq!(RectU32::of(3, 6, 2, 8).iter_x().collect::<Vec<u32>>(), []);
        assert_eq!(RectU32::of(3, 6, 3, 8).iter_x().collect::<Vec<u32>>(), [3]);
        assert_eq!(RectU32::of(3, 6, 4, 8).iter_x().collect::<Vec<u32>>(), [3, 4]);
        assert_eq!(RectU32::of(3, 6, 5, 8).iter_x().collect::<Vec<u32>>(), [3, 4, 5]);
        assert_eq!(RectU32::of(3, 6, 6, 8).iter_x().collect::<Vec<u32>>(), [3, 4, 5, 6]);
        assert_eq!(RectU32::of(3, 6, 6, 8).iter_x().rev().collect::<Vec<u32>>(), [6, 5, 4, 3]);
        assert_eq!(RectU32::of(3, 6, 5, 8).iter_x().rev().collect::<Vec<u32>>(), [5, 4, 3]);
        assert_eq!(RectU32::of(3, 6, 4, 8).iter_x().rev().collect::<Vec<u32>>(), [4, 3]);
        assert_eq!(RectU32::of(3, 6, 3, 8).iter_x().rev().collect::<Vec<u32>>(), [3]);
        assert_eq!(RectU32::of(3, 6, 2, 8).iter_x().rev().collect::<Vec<u32>>(), []);
    }

    #[test]
    fn iter_y() {
        assert_eq!(RectU32::of(3, 6, 4, 5).iter_y().collect::<Vec<u32>>(), []);
        assert_eq!(RectU32::of(3, 6, 4, 6).iter_y().collect::<Vec<u32>>(), [6]);
        assert_eq!(RectU32::of(3, 6, 4, 7).iter_y().collect::<Vec<u32>>(), [6, 7]);
        assert_eq!(RectU32::of(3, 6, 4, 8).iter_y().collect::<Vec<u32>>(), [6, 7, 8]);
        assert_eq!(RectU32::of(3, 6, 4, 9).iter_y().collect::<Vec<u32>>(), [6, 7, 8, 9]);
        assert_eq!(RectU32::of(3, 6, 4, 9).iter_y().rev().collect::<Vec<u32>>(), [9, 8, 7, 6]);
        assert_eq!(RectU32::of(3, 6, 4, 8).iter_y().rev().collect::<Vec<u32>>(), [8, 7, 6]);
        assert_eq!(RectU32::of(3, 6, 4, 7).iter_y().rev().collect::<Vec<u32>>(), [7, 6]);
        assert_eq!(RectU32::of(3, 6, 4, 6).iter_y().rev().collect::<Vec<u32>>(), [6]);
        assert_eq!(RectU32::of(3, 6, 4, 5).iter_y().rev().collect::<Vec<u32>>(), []);
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectU32::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_x(&RectU32::of(0, 0, u32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectU32::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_y(&RectU32::of(0, 0, 0, u32::MAX)), u32::MAX);
    }

    #[test]
    fn test_max_delta() {
        assert_eq!(max_delta(&RectU32::of(0, 5, 10, 10)), 10);
        assert_eq!(max_delta(&RectU32::of(5, 0, 9, 9)), 9);
    }

    #[test]
    fn max_delta_0() {
        assert_eq!(max_delta(&RectU32::of(0, 0, 0, 0)), 0);
        assert_eq!(max_delta(&RectU32::of(1, 1, 1, 1)), 0);
        assert_eq!(max_delta(&RectU32::of(5, 10, 5, 10)), 0);
    }

    #[test]
    fn max_delta_1() {
        assert_eq!(max_delta(&RectU32::of(0, 0, 1, 1)), 1);
        assert_eq!(max_delta(&RectU32::of(5, 5, 6, 6)), 1);
        assert_eq!(max_delta(&RectU32::of(0, 0, 0, 1)), 1);
        assert_eq!(max_delta(&RectU32::of(5, 9, 5, 10)), 1);
    }

    #[test]
    fn max_delta_max() {
        assert_eq!(max_delta(&RectU32::of(0, 0, u32::MAX, u32::MAX - 1)), u32::MAX);
        assert_eq!(max_delta(&RectU32::of(0, 0, u32::MAX - 1, u32::MAX)), u32::MAX);
    }

    #[test]
    fn test_len_x() {
        assert_eq!(len_x(&RectU32::of(0, 0, 0, 0)), 1);
        assert_eq!(len_x(&RectU32::of(0, 0, u32::MAX - 1, 0)), u32::MAX);
    }

    #[test]
    fn test_len_y() {
        assert_eq!(len_y(&RectU32::of(0, 0, 0, 0)), 1);
        assert_eq!(len_y(&RectU32::of(0, 0, 0, u32::MAX - 1)), u32::MAX);
    }

    #[test]
    fn test_max_len() {
        assert_eq!(max_len(&RectU32::of(0, 5, 10, 10)), 11);
        assert_eq!(max_len(&RectU32::of(5, 0, 9, 9)), 10);
    }

    #[test]
    fn max_len_1() {
        assert_eq!(max_len(&RectU32::of(0, 0, 0, 0)), 1);
        assert_eq!(max_len(&RectU32::of(1, 1, 1, 1)), 1);
        assert_eq!(max_len(&RectU32::of(5, 10, 5, 10)), 1);
    }

    #[test]
    fn max_len_2() {
        assert_eq!(max_len(&RectU32::of(0, 0, 1, 1)), 2);
        assert_eq!(max_len(&RectU32::of(5, 5, 6, 6)), 2);
        assert_eq!(max_len(&RectU32::of(0, 0, 0, 1)), 2);
        assert_eq!(max_len(&RectU32::of(5, 9, 5, 10)), 2);
    }

    #[test]
    fn inflate_min_bounds() {
        let mut r = RectU32::of(7, 2, 4, 13);
        inflate(&mut r);
        assert_eq!(r, RectU32::of(6, 1, 5, 14));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(5, 0, 6, 15));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(4, 0, 7, 17));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(3, 0, 8, 19));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(2, 0, 9, 21));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(1, 0, 10, 23));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(0, 0, 11, 25));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(0, 0, 13, 27));
    }

    #[test]
    fn inflate_max_bounds() {
        let mut r = RectU32::of(200, 230, u32::MAX - 5, u32::MAX - 3);
        inflate(&mut r);
        assert_eq!(r, RectU32::of(199, 229, u32::MAX - 4, u32::MAX - 2));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(198, 228, u32::MAX - 3, u32::MAX - 1));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(197, 227, u32::MAX - 2, u32::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(196, 225, u32::MAX - 1, u32::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(195, 223, u32::MAX, u32::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(193, 221, u32::MAX, u32::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(191, 219, u32::MAX, u32::MAX));
        inflate(&mut r);
        assert_eq!(r, RectU32::of(189, 217, u32::MAX, u32::MAX));
    }

    #[test]
    fn inflate_almost_min_bounds() {
        let mut r = RectU32::of(1, 1, u32::MAX, u32::MAX);
        inflate(&mut r);
        assert_eq!(r, RectU32::of(0, 0, u32::MAX, u32::MAX));
    }

    #[test]
    fn inflate_almost_max_bounds() {
        let mut r = RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1);
        inflate(&mut r);
        assert_eq!(r, RectU32::of(0, 0, u32::MAX, u32::MAX));
    }

    #[test]
    fn inflate_max_width() {
        let mut r = RectU32::of(0, 10, u32::MAX, 250);
        inflate(&mut r);
        assert_eq!(r, RectU32::of(0, 10, u32::MAX, 250));
    }

    #[test]
    fn inflate_max_height() {
        let mut r = RectU32::of(10, 0, 250, u32::MAX);
        inflate(&mut r);
        assert_eq!(r, RectU32::of(10, 0, 250, u32::MAX));
    }

    #[test]
    fn deflate_odd() {
        let mut r = RectU32::of(0, 0, 9, 9);
        deflate(&mut r);
        assert_eq!(r, RectU32::of(1, 1, 8, 8));
        deflate(&mut r);
        assert_eq!(r, RectU32::of(2, 2, 7, 7));
        deflate(&mut r);
        assert_eq!(r, RectU32::of(3, 3, 6, 6));
        deflate(&mut r);
        assert_eq!(r, RectU32::of(4, 4, 5, 5));
        deflate(&mut r);
        assert_eq!(r, RectU32::of(4, 4, 5, 5));
    }

    #[test]
    fn deflate_even() {
        let mut r = RectU32::of(0, 0, 10, 10);
        deflate(&mut r);
        assert_eq!(r, RectU32::of(1, 1, 9, 9));
        deflate(&mut r);
        assert_eq!(r, RectU32::of(2, 2, 8, 8));
        deflate(&mut r);
        assert_eq!(r, RectU32::of(3, 3, 7, 7));
        deflate(&mut r);
        assert_eq!(r, RectU32::of(4, 4, 6, 6));
        deflate(&mut r);
        assert_eq!(r, RectU32::of(4, 4, 6, 6));
    }

    #[test]
    fn resize_odd_size() {
        let mut r = RectU32::of(5, 5, 15, 15);
        resize(&mut r, 11);
        assert_eq!(r, RectU32::of(5, 5, 15, 15));
        resize(&mut r, 9);
        assert_eq!(r, RectU32::of(6, 6, 14, 14));
        resize(&mut r, 7);
        assert_eq!(r, RectU32::of(7, 7, 13, 13));
        resize(&mut r, 5);
        assert_eq!(r, RectU32::of(8, 8, 12, 12));
        resize(&mut r, 3);
        assert_eq!(r, RectU32::of(9, 9, 11, 11));
        resize(&mut r, 1);
        assert_eq!(r, RectU32::of(9, 9, 11, 11));
        resize(&mut r, 3);
        assert_eq!(r, RectU32::of(9, 9, 11, 11));
        resize(&mut r, 9);
        assert_eq!(r, RectU32::of(6, 6, 14, 14));
    }

    #[test]
    fn resize_even_size() {
        let mut r = RectU32::of(5, 5, 14, 14);
        resize(&mut r, 10);
        assert_eq!(r, RectU32::of(5, 5, 14, 14));
        resize(&mut r, 8);
        assert_eq!(r, RectU32::of(6, 6, 13, 13));
        resize(&mut r, 6);
        assert_eq!(r, RectU32::of(7, 7, 12, 12));
        resize(&mut r, 4);
        assert_eq!(r, RectU32::of(8, 8, 11, 11));
        resize(&mut r, 2);
        assert_eq!(r, RectU32::of(8, 8, 11, 11));
        resize(&mut r, 4);
        assert_eq!(r, RectU32::of(8, 8, 11, 11));
        resize(&mut r, 8);
        assert_eq!(r, RectU32::of(6, 6, 13, 13));
    }

    #[test]
    fn resize_odd_min_bounds_big_delta() {
        let mut r = RectU32::of(0, 0, 2, 2);
        resize(&mut r, u32::MAX);
        assert_eq!(r, RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1));
    }

    #[test]
    fn resize_even_min_bounds_big_delta() {
        let mut r = RectU32::of(0, 0, 3, 3);
        resize(&mut r, u32::MAX - 1);
        assert_eq!(r, RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 2));
    }

    #[test]
    fn resize_odd_max_bounds_big_delta() {
        let mut r = RectU32::of(u32::MAX - 2, u32::MAX - 2, u32::MAX, u32::MAX);
        resize(&mut r, u32::MAX);
        assert_eq!(r, RectU32::of(1, 1, u32::MAX, u32::MAX));
    }

    #[test]
    fn resize_even_max_bounds_big_delta() {
        let mut r = RectU32::of(u32::MAX - 3, u32::MAX - 3, u32::MAX, u32::MAX);
        resize(&mut r, u32::MAX - 1);
        assert_eq!(r, RectU32::of(2, 2, u32::MAX, u32::MAX));
    }

    #[test]
    fn test_saturating_translate() {
        let mut r = RectU32::of(0, 0, 10, 10);
        saturating_translate(&mut r, &PointI32::of(10, 10));
        assert_eq!(r, RectU32::of(10, 10, 20, 20));
        saturating_translate(&mut r, &PointI32::of(-5, -5));
        assert_eq!(r, RectU32::of(5, 5, 15, 15));
        saturating_translate(&mut r, &PointI32::of(2, 2));
        assert_eq!(r, RectU32::of(7, 7, 17, 17));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut r = RectU32::of(2, 5, 12, 15);
        saturating_translate(&mut r, &PointI32::of(-10, -10));
        assert_eq!(r, RectU32::of(0, 0, 10, 10));
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut r = RectU32::of(240, 235, u32::MAX - 5, u32::MAX - 10);
        saturating_translate(&mut r, &PointI32::of(20, 20));
        assert_eq!(r, RectU32::of(245, 245, u32::MAX, u32::MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_big_delta() {
        let mut r = RectU32::of(0, 0, 10, 10);
        saturating_translate(&mut r, &PointI32::min());
        assert_eq!(r, RectU32::of(0, 0, 10, 10));
    }

    #[test]
    fn saturating_translate_max_bounds_big_delta() {
        let mut r = RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX, u32::MAX);
        saturating_translate(&mut r, &PointI32::max());
        assert_eq!(r, RectU32::of(u32::MAX - 10, u32::MAX - 10, u32::MAX, u32::MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_big_rect_big_delta() {
        let mut r = RectU32::of(1, 1, u32::MAX, u32::MAX);
        saturating_translate(&mut r, &PointI32::min());
        assert_eq!(r, RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1));
    }

    #[test]
    fn saturating_translate_max_bounds_big_rect_big_delta() {
        let mut r = RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1);
        saturating_translate(&mut r, &PointI32::max());
        assert_eq!(r, RectU32::of(1, 1, u32::MAX, u32::MAX));
    }

    #[test]
    fn contains_inside_borders() {
        assert!(contains(&RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1), &PointU32::of(1, 1)));
        assert!(contains(&RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1), &PointU32::of(1, u32::MAX - 1)));
        assert!(contains(&RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1), &PointU32::of(u32::MAX - 1, 1)));
        assert!(contains(&RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1), &PointU32::of(u32::MAX - 1, u32::MAX - 1)));
    }

    #[test]
    fn contains_outside_borders() {
        assert!(!contains(&RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1), &PointU32::of(0, 0)));
        assert!(!contains(&RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1), &PointU32::of(0, u32::MAX)));
        assert!(!contains(&RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1), &PointU32::of(u32::MAX, 0)));
        assert!(!contains(&RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1), &PointU32::of(u32::MAX, u32::MAX)));
    }

    #[test]
    fn contains_inside() {
        assert!(contains(&RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1), &PointU32::of(10, 10)));
        assert!(contains(&RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1), &PointU32::of(u32::MAX - 10, u32::MAX - 10)));
    }
}
