use std::ops::RangeInclusive;

use crate::cartesian::point::point_i16;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct RectI16 {
    pub min: point_i16::PointI16,
    pub max: point_i16::PointI16,
}

impl RectI16 {
    pub fn of(x1: i16, y1: i16, x2: i16, y2: i16) -> Self {
        RectI16 { min: point_i16::PointI16::of(x1, y1), max: point_i16::PointI16::of(x2, y2) }
    }

    pub fn iter_x(&self) -> RangeInclusive<i16> {
        self.min.x..=self.max.x
    }

    pub fn iter_y(&self) -> RangeInclusive<i16> {
        self.min.y..=self.max.y
    }
}

impl std::fmt::Display for RectI16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_x(r: &RectI16) -> u16 {
    point_i16::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectI16) -> u16 {
    point_i16::delta_y(&r.min, &r.max)
}

pub fn max_delta(r: &RectI16) -> u16 {
    std::cmp::max(delta_x(r), delta_y(r))
}

pub fn len_x(r: &RectI16) -> u16 {
    delta_x(r) + 1
}

pub fn len_y(r: &RectI16) -> u16 {
    delta_y(r) + 1
}

pub fn max_len(r: &RectI16) -> u16 {
    std::cmp::max(len_x(r), len_y(r))
}

pub fn inflate(r: &mut RectI16) {
    let is_min_x = r.min.x == i16::MIN;
    let is_min_y = r.min.y == i16::MIN;
    let is_max_x = r.max.x == i16::MAX;
    let is_max_y = r.max.y == i16::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return;
    }
    let min_x_modifier = 1 - i16::from(is_min_x) + i16::from(is_max_x);
    let min_y_modifier = 1 - i16::from(is_min_y) + i16::from(is_max_y);
    let max_x_modifier = 1 + i16::from(is_min_x) - i16::from(is_max_x);
    let max_y_modifier = 1 + i16::from(is_min_y) - i16::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
}

pub fn deflate(r: &mut RectI16) {
    if delta_x(r) < 3 || delta_y(r) < 3 {
        return;
    }
    r.min.x += 1;
    r.min.y += 1;
    r.max.x -= 1;
    r.max.y -= 1;
}

pub fn resize(r: &mut RectI16, size: u16) {
    if size < 3 {
        return;
    }
    let diff_x = i32::from(delta_x(r)) + 1 - i32::from(size);
    let diff_y = i32::from(delta_y(r)) + 1 - i32::from(size);
    let temp_min_x = i32::from(r.min.x) + diff_x / 2;
    let temp_min_y = i32::from(r.min.y) + diff_y / 2;
    let min_x = temp_min_x.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(size) + 1);
    let min_y = temp_min_y.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(size) + 1);
    r.min.x = min_x as i16;
    r.min.y = min_y as i16;
    r.max.x = (min_x + i32::from(size) - 1) as i16;
    r.max.y = (min_y + i32::from(size) - 1) as i16;
}

pub fn saturating_translate(r: &mut RectI16, delta: &point_i16::PointI16) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i32::from(r.min.x) + i32::from(delta.x);
    let temp_min_y = i32::from(r.min.y) + i32::from(delta.y);
    let min_x = temp_min_x.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(dx));
    let min_y = temp_min_y.clamp(i32::from(i16::MIN), i32::from(i16::MAX) - i32::from(dy));
    r.min.x = min_x as i16;
    r.min.y = min_y as i16;
    r.max.x = (min_x + i32::from(dx)) as i16;
    r.max.y = (min_y + i32::from(dy)) as i16;
}

pub fn checked_translate(r: &mut RectI16, delta: &point_i16::PointI16) -> Result<(), ()> {
    let min_x = r.min.x.checked_add(delta.x).ok_or(())?;
    let min_y = r.min.y.checked_add(delta.y).ok_or(())?;
    let max_x = r.max.x.checked_add(delta.x).ok_or(())?;
    let max_y = r.max.y.checked_add(delta.y).ok_or(())?;
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = max_x;
    r.max.y = max_y;
    Ok(())
}

pub fn contains(r: &RectI16, p: &point_i16::PointI16) -> bool {
    p.x >= r.min.x && p.x <= r.max.x && p.y >= r.min.y && p.y <= r.max.y
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_i16::PointI16;

    use super::{
        RectI16, checked_translate, contains, deflate, delta_x, delta_y, inflate, len_x, len_y, max_delta, max_len, resize, saturating_translate,
    };

    #[test]
    fn rect_i16() {
        assert_eq!(RectI16::of(i16::MIN, -1, 1, i16::MAX), RectI16 { min: PointI16 { x: i16::MIN, y: -1 }, max: PointI16 { x: 1, y: i16::MAX } });
        assert_eq!(RectI16::of(i16::MIN, -0, 0, i16::MAX).to_string(), "((-32768, 0), (0, 32767))");
    }

    #[test]
    fn iter_x() {
        assert_eq!(RectI16::of(-6, -8, -7, -6).iter_x().collect::<Vec<i16>>(), []);
        assert_eq!(RectI16::of(-6, -8, -6, -6).iter_x().collect::<Vec<i16>>(), [-6]);
        assert_eq!(RectI16::of(-6, -8, -5, -6).iter_x().collect::<Vec<i16>>(), [-6, -5]);
        assert_eq!(RectI16::of(-6, -8, -4, -6).iter_x().collect::<Vec<i16>>(), [-6, -5, -4]);
        assert_eq!(RectI16::of(-6, -8, -3, -6).iter_x().collect::<Vec<i16>>(), [-6, -5, -4, -3]);
        assert_eq!(RectI16::of(-6, -8, -3, -6).iter_x().rev().collect::<Vec<i16>>(), [-3, -4, -5, -6]);
        assert_eq!(RectI16::of(-6, -8, -4, -6).iter_x().rev().collect::<Vec<i16>>(), [-4, -5, -6]);
        assert_eq!(RectI16::of(-6, -8, -5, -6).iter_x().rev().collect::<Vec<i16>>(), [-5, -6]);
        assert_eq!(RectI16::of(-6, -8, -6, -6).iter_x().rev().collect::<Vec<i16>>(), [-6]);
        assert_eq!(RectI16::of(-6, -8, -7, -6).iter_x().rev().collect::<Vec<i16>>(), []);
    }

    #[test]
    fn iter_y() {
        assert_eq!(RectI16::of(-6, -8, -4, -9).iter_y().collect::<Vec<i16>>(), []);
        assert_eq!(RectI16::of(-6, -8, -4, -8).iter_y().collect::<Vec<i16>>(), [-8]);
        assert_eq!(RectI16::of(-6, -8, -4, -7).iter_y().collect::<Vec<i16>>(), [-8, -7]);
        assert_eq!(RectI16::of(-6, -8, -4, -6).iter_y().collect::<Vec<i16>>(), [-8, -7, -6]);
        assert_eq!(RectI16::of(-6, -8, -4, -5).iter_y().collect::<Vec<i16>>(), [-8, -7, -6, -5]);
        assert_eq!(RectI16::of(-6, -8, -4, -5).iter_y().rev().collect::<Vec<i16>>(), [-5, -6, -7, -8]);
        assert_eq!(RectI16::of(-6, -8, -4, -6).iter_y().rev().collect::<Vec<i16>>(), [-6, -7, -8]);
        assert_eq!(RectI16::of(-6, -8, -4, -7).iter_y().rev().collect::<Vec<i16>>(), [-7, -8]);
        assert_eq!(RectI16::of(-6, -8, -4, -8).iter_y().rev().collect::<Vec<i16>>(), [-8]);
        assert_eq!(RectI16::of(-6, -8, -4, -9).iter_y().rev().collect::<Vec<i16>>(), []);
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectI16::of(0, i16::MIN, 0, i16::MAX)), 0);
        assert_eq!(delta_x(&RectI16::of(i16::MIN, 0, i16::MAX, 0)), u16::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectI16::of(i16::MIN, 0, i16::MAX, 0)), 0);
        assert_eq!(delta_y(&RectI16::of(0, i16::MIN, 0, i16::MAX)), u16::MAX);
    }

    #[test]
    fn test_max_delta() {
        assert_eq!(max_delta(&RectI16::of(0, 5, 10, 10)), 10);
        assert_eq!(max_delta(&RectI16::of(-10, -10, -5, 0)), 10);
        assert_eq!(max_delta(&RectI16::of(-5, 0, 5, 5)), 10);
    }

    #[test]
    fn max_delta_0() {
        assert_eq!(max_delta(&RectI16::of(0, 0, 0, 0)), 0);
        assert_eq!(max_delta(&RectI16::of(1, 1, 1, 1)), 0);
        assert_eq!(max_delta(&RectI16::of(-1, -1, -1, -1)), 0);
        assert_eq!(max_delta(&RectI16::of(5, 10, 5, 10)), 0);
    }

    #[test]
    fn max_delta_1() {
        assert_eq!(max_delta(&RectI16::of(0, 0, 1, 1)), 1);
        assert_eq!(max_delta(&RectI16::of(5, 5, 6, 6)), 1);
        assert_eq!(max_delta(&RectI16::of(-6, -6, -5, -5)), 1);
        assert_eq!(max_delta(&RectI16::of(0, 0, 0, 1)), 1);
        assert_eq!(max_delta(&RectI16::of(5, 9, 5, 10)), 1);
    }

    #[test]
    fn max_delta_bounds() {
        assert_eq!(max_delta(&RectI16::of(i16::MIN + 1, i16::MIN, i16::MAX, i16::MAX)), u16::MAX);
        assert_eq!(max_delta(&RectI16::of(i16::MIN, i16::MIN + 1, i16::MAX, i16::MAX)), u16::MAX);
        assert_eq!(max_delta(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX)), u16::MAX);
        assert_eq!(max_delta(&RectI16::of(i16::MIN, i16::MIN, i16::MAX, i16::MAX - 1)), u16::MAX);
    }

    #[test]
    fn test_len_x() {
        assert_eq!(len_x(&RectI16::of(0, i16::MIN, 0, i16::MAX)), 1);
        assert_eq!(len_x(&RectI16::of(i16::MIN, 0, i16::MAX - 1, 0)), u16::MAX);
    }

    #[test]
    fn test_len_y() {
        assert_eq!(len_y(&RectI16::of(i16::MIN, 0, i16::MAX, 0)), 1);
        assert_eq!(len_y(&RectI16::of(0, i16::MIN, 0, i16::MAX - 1)), u16::MAX);
    }

    #[test]
    fn test_max_len() {
        assert_eq!(max_len(&RectI16::of(0, 5, 10, 10)), 11);
        assert_eq!(max_len(&RectI16::of(-10, -10, -5, 0)), 11);
        assert_eq!(max_len(&RectI16::of(-5, 0, 5, 5)), 11);
    }

    #[test]
    fn max_len_1() {
        assert_eq!(max_len(&RectI16::of(0, 0, 0, 0)), 1);
        assert_eq!(max_len(&RectI16::of(1, 1, 1, 1)), 1);
        assert_eq!(max_len(&RectI16::of(-1, -1, -1, -1)), 1);
        assert_eq!(max_len(&RectI16::of(5, 10, 5, 10)), 1);
    }

    #[test]
    fn max_len_2() {
        assert_eq!(max_len(&RectI16::of(0, 0, 1, 1)), 2);
        assert_eq!(max_len(&RectI16::of(5, 5, 6, 6)), 2);
        assert_eq!(max_len(&RectI16::of(-6, -6, -5, -5)), 2);
        assert_eq!(max_len(&RectI16::of(0, 0, 0, 1)), 2);
        assert_eq!(max_len(&RectI16::of(5, 9, 5, 10)), 2);
    }

    #[test]
    fn max_len_bounds() {
        assert_eq!(max_len(&RectI16::of(i16::MIN + 1, i16::MIN, i16::MAX - 1, i16::MAX - 1)), u16::MAX);
        assert_eq!(max_len(&RectI16::of(i16::MIN, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1)), u16::MAX);
        assert_eq!(max_len(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 1)), u16::MAX);
        assert_eq!(max_len(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 2)), u16::MAX);
    }

    #[test]
    fn inflate_min_bounds() {
        let mut r = RectI16::of(i16::MIN + 7, i16::MIN + 2, 4, 13);
        inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 6, i16::MIN + 1, 5, 14));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 5, i16::MIN, 6, 15));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 4, i16::MIN, 7, 17));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 3, i16::MIN, 8, 19));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 2, i16::MIN, 9, 21));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN + 1, i16::MIN, 10, 23));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, 11, 25));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, 13, 27));
    }

    #[test]
    fn inflate_max_bounds() {
        let mut r = RectI16::of(-100, 30, i16::MAX - 5, i16::MAX - 3);
        inflate(&mut r);
        assert_eq!(r, RectI16::of(-101, 29, i16::MAX - 4, i16::MAX - 2));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(-102, 28, i16::MAX - 3, i16::MAX - 1));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(-103, 27, i16::MAX - 2, i16::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(-104, 25, i16::MAX - 1, i16::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(-105, 23, i16::MAX, i16::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(-107, 21, i16::MAX, i16::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(-109, 19, i16::MAX, i16::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI16::of(-111, 17, i16::MAX, i16::MAX));
    }

    #[test]
    fn inflate_almost_min_bounds() {
        let mut r = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX);
        inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MAX, i16::MAX));
    }

    #[test]
    fn inflate_almost_max_bounds() {
        let mut r = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1);
        inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MAX, i16::MAX));
    }

    #[test]
    fn inflate_max_width() {
        let mut r = RectI16::of(i16::MIN, 10, i16::MAX, 50);
        inflate(&mut r);
        assert_eq!(r, RectI16::of(i16::MIN, 10, i16::MAX, 50));
    }

    #[test]
    fn inflate_max_height() {
        let mut r = RectI16::of(10, i16::MIN, 50, i16::MAX);
        inflate(&mut r);
        assert_eq!(r, RectI16::of(10, i16::MIN, 50, i16::MAX));
    }

    #[test]
    fn deflate_odd() {
        let mut r = RectI16::of(-5, -5, 5, 5);
        deflate(&mut r);
        assert_eq!(r, RectI16::of(-4, -4, 4, 4));
        deflate(&mut r);
        assert_eq!(r, RectI16::of(-3, -3, 3, 3));
        deflate(&mut r);
        assert_eq!(r, RectI16::of(-2, -2, 2, 2));
        deflate(&mut r);
        assert_eq!(r, RectI16::of(-1, -1, 1, 1));
        deflate(&mut r);
        assert_eq!(r, RectI16::of(-1, -1, 1, 1));
    }

    #[test]
    fn deflate_even() {
        let mut r = RectI16::of(-5, -5, 4, 4);
        deflate(&mut r);
        assert_eq!(r, RectI16::of(-4, -4, 3, 3));
        deflate(&mut r);
        assert_eq!(r, RectI16::of(-3, -3, 2, 2));
        deflate(&mut r);
        assert_eq!(r, RectI16::of(-2, -2, 1, 1));
        deflate(&mut r);
        assert_eq!(r, RectI16::of(-1, -1, 0, 0));
        deflate(&mut r);
        assert_eq!(r, RectI16::of(-1, -1, 0, 0));
    }

    #[test]
    fn resize_odd() {
        let mut r = RectI16::of(-5, -5, 5, 5);
        resize(&mut r, 11);
        assert_eq!(r, RectI16::of(-5, -5, 5, 5));
        resize(&mut r, 9);
        assert_eq!(r, RectI16::of(-4, -4, 4, 4));
        resize(&mut r, 7);
        assert_eq!(r, RectI16::of(-3, -3, 3, 3));
        resize(&mut r, 5);
        assert_eq!(r, RectI16::of(-2, -2, 2, 2));
        resize(&mut r, 3);
        assert_eq!(r, RectI16::of(-1, -1, 1, 1));
        resize(&mut r, 1);
        assert_eq!(r, RectI16::of(-1, -1, 1, 1));
        resize(&mut r, 1);
        assert_eq!(r, RectI16::of(-1, -1, 1, 1));
        resize(&mut r, 3);
        assert_eq!(r, RectI16::of(-1, -1, 1, 1));
        resize(&mut r, 5);
        assert_eq!(r, RectI16::of(-2, -2, 2, 2));
        resize(&mut r, 7);
        assert_eq!(r, RectI16::of(-3, -3, 3, 3));
    }

    #[test]
    fn resize_even() {
        let mut r = RectI16::of(-5, -5, 4, 4);
        resize(&mut r, 10);
        assert_eq!(r, RectI16::of(-5, -5, 4, 4));
        resize(&mut r, 8);
        assert_eq!(r, RectI16::of(-4, -4, 3, 3));
        resize(&mut r, 6);
        assert_eq!(r, RectI16::of(-3, -3, 2, 2));
        resize(&mut r, 4);
        assert_eq!(r, RectI16::of(-2, -2, 1, 1));
        resize(&mut r, 2);
        assert_eq!(r, RectI16::of(-2, -2, 1, 1));
        resize(&mut r, 2);
        assert_eq!(r, RectI16::of(-2, -2, 1, 1));
        resize(&mut r, 4);
        assert_eq!(r, RectI16::of(-2, -2, 1, 1));
        resize(&mut r, 6);
        assert_eq!(r, RectI16::of(-3, -3, 2, 2));
    }

    #[test]
    fn resize_even_2nd_scenario() {
        let mut r = RectI16::of(-4, -4, 5, 5);
        resize(&mut r, 10);
        assert_eq!(r, RectI16::of(-4, -4, 5, 5));
        resize(&mut r, 8);
        assert_eq!(r, RectI16::of(-3, -3, 4, 4));
        resize(&mut r, 6);
        assert_eq!(r, RectI16::of(-2, -2, 3, 3));
        resize(&mut r, 4);
        assert_eq!(r, RectI16::of(-1, -1, 2, 2));
        resize(&mut r, 2);
        assert_eq!(r, RectI16::of(-1, -1, 2, 2));
        resize(&mut r, 2);
        assert_eq!(r, RectI16::of(-1, -1, 2, 2));
        resize(&mut r, 4);
        assert_eq!(r, RectI16::of(-1, -1, 2, 2));
        resize(&mut r, 6);
        assert_eq!(r, RectI16::of(-2, -2, 3, 3));
    }

    #[test]
    fn resize_odd_min_bounds_big_delta() {
        let mut r = RectI16::of(i16::MIN, i16::MIN, i16::MIN + 2, i16::MIN + 2);
        resize(&mut r, u16::MAX);
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1));
    }

    #[test]
    fn resize_even_min_bounds_big_delta() {
        let mut r = RectI16::of(i16::MIN, i16::MIN, i16::MIN + 3, i16::MIN + 3);
        resize(&mut r, u16::MAX - 1);
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 2, i16::MAX - 2));
    }

    #[test]
    fn resize_odd_max_bounds_big_delta() {
        let mut r = RectI16::of(i16::MAX - 2, i16::MAX - 2, i16::MAX, i16::MAX);
        resize(&mut r, u16::MAX);
        assert_eq!(r, RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX));
    }

    #[test]
    fn resize_even_max_bounds_big_delta() {
        let mut r = RectI16::of(i16::MAX - 3, i16::MAX - 3, i16::MAX, i16::MAX);
        resize(&mut r, u16::MAX - 1);
        assert_eq!(r, RectI16::of(i16::MIN + 2, i16::MIN + 2, i16::MAX, i16::MAX));
    }

    #[test]
    fn test_saturating_translate() {
        let mut r = RectI16::of(0, 0, 10, 10);
        saturating_translate(&mut r, &PointI16::of(10, 20));
        assert_eq!(r, RectI16::of(10, 20, 20, 30));
        saturating_translate(&mut r, &PointI16::of(-20, -15));
        assert_eq!(r, RectI16::of(-10, 5, 0, 15));
        saturating_translate(&mut r, &PointI16::of(3, -2));
        assert_eq!(r, RectI16::of(-7, 3, 3, 13));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut r = RectI16::of(i16::MIN + 5, i16::MIN + 10, 12, 15);
        saturating_translate(&mut r, &PointI16::of(-10, -10));
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, 7, 5));
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut r = RectI16::of(40, 35, i16::MAX - 5, i16::MAX - 10);
        saturating_translate(&mut r, &PointI16::of(20, 20));
        assert_eq!(r, RectI16::of(45, 45, i16::MAX, i16::MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_big_delta() {
        let mut r = RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 10);
        saturating_translate(&mut r, &PointI16::min());
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MIN + 10, i16::MIN + 10));
    }

    #[test]
    fn saturating_translate_max_bounds_big_delta() {
        let mut r = RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX, i16::MAX);
        saturating_translate(&mut r, &PointI16::max());
        assert_eq!(r, RectI16::of(i16::MAX - 10, i16::MAX - 10, i16::MAX, i16::MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_big_rect_big_delta() {
        let mut r = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX);
        saturating_translate(&mut r, &PointI16::min());
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1));
    }

    #[test]
    fn saturating_translate_max_bounds_big_rect_big_delta() {
        let mut r = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1);
        saturating_translate(&mut r, &PointI16::max());
        assert_eq!(r, RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX));
    }

    #[test]
    fn test_checked_translate() {
        let mut r = RectI16::of(0, 0, 10, 10);
        assert_eq!(checked_translate(&mut r, &PointI16::of(10, 20)), Ok(()));
        assert_eq!(r, RectI16::of(10, 20, 20, 30));
        assert_eq!(checked_translate(&mut r, &PointI16::of(-20, -15)), Ok(()));
        assert_eq!(r, RectI16::of(-10, 5, 0, 15));
        assert_eq!(checked_translate(&mut r, &PointI16::of(3, -2)), Ok(()));
        assert_eq!(r, RectI16::of(-7, 3, 3, 13));
    }

    #[test]
    fn checked_translate_min_bounds() {
        let mut r = RectI16::of(i16::MIN + 5, i16::MIN + 10, 12, 15);
        assert_eq!(checked_translate(&mut r, &PointI16::of(-10, -10)), Err(()));
        assert_eq!(r, RectI16::of(i16::MIN + 5, i16::MIN + 10, 12, 15));
    }

    #[test]
    fn checked_translate_max_bounds() {
        let mut r = RectI16::of(40, 35, i16::MAX - 5, i16::MAX - 10);
        assert_eq!(checked_translate(&mut r, &PointI16::of(20, 20)), Err(()));
        assert_eq!(r, RectI16::of(40, 35, i16::MAX - 5, i16::MAX - 10));
    }

    #[test]
    fn checked_translate_min_bounds_big_rect_big_delta() {
        let mut r = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX);
        assert_eq!(checked_translate(&mut r, &PointI16::min()), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI16::of(i16::MIN, 0)), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI16::of(0, i16::MIN)), Err(()));
        assert_eq!(r, RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX));
    }

    #[test]
    fn checked_translate_max_bounds_big_rect_big_delta() {
        let mut r = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1);
        assert_eq!(checked_translate(&mut r, &PointI16::max()), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI16::of(i16::MAX, 0)), Err(()));
        assert_eq!(checked_translate(&mut r, &PointI16::of(0, i16::MAX)), Err(()));
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1));
    }

    #[test]
    fn checked_translate_min_bounds_big_rect_small_delta() {
        let mut r = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX);
        assert_eq!(checked_translate(&mut r, &PointI16::of(-1, -1)), Ok(()));
        assert_eq!(r, RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1));
    }

    #[test]
    fn checked_translate_max_bounds_big_rect_small_delta() {
        let mut r = RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1);
        assert_eq!(checked_translate(&mut r, &PointI16::of(1, 1)), Ok(()));
        assert_eq!(r, RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX));
    }

    #[test]
    fn contains_inside_borders() {
        let r = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assert!(contains(&r, &PointI16::of(i16::MIN + 1, i16::MIN + 1)));
        assert!(contains(&r, &PointI16::of(i16::MIN + 1, i16::MAX - 1)));
        assert!(contains(&r, &PointI16::of(i16::MAX - 1, i16::MIN + 1)));
        assert!(contains(&r, &PointI16::of(i16::MAX - 1, i16::MAX - 1)));
    }

    #[test]
    fn contains_outside_borders() {
        let r = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assert!(!contains(&r, &PointI16::of(i16::MIN, i16::MIN)));
        assert!(!contains(&r, &PointI16::of(i16::MIN, i16::MAX)));
        assert!(!contains(&r, &PointI16::of(i16::MAX, i16::MIN)));
        assert!(!contains(&r, &PointI16::of(i16::MAX, i16::MAX)));
        assert!(!contains(&r, &PointI16::of(i16::MIN + 1, i16::MIN)));
        assert!(!contains(&r, &PointI16::of(i16::MIN + 1, i16::MAX)));
        assert!(!contains(&r, &PointI16::of(i16::MAX - 1, i16::MIN)));
        assert!(!contains(&r, &PointI16::of(i16::MAX - 1, i16::MAX)));
        assert!(!contains(&r, &PointI16::of(i16::MIN, i16::MIN + 1)));
        assert!(!contains(&r, &PointI16::of(i16::MIN, i16::MAX - 1)));
        assert!(!contains(&r, &PointI16::of(i16::MAX, i16::MIN + 1)));
        assert!(!contains(&r, &PointI16::of(i16::MAX, i16::MAX - 1)));
    }

    #[test]
    fn contains_inside() {
        let r = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
        assert!(contains(&r, &PointI16::of(i16::MIN + 10, i16::MIN + 10)));
        assert!(contains(&r, &PointI16::of(i16::MAX - 10, i16::MAX - 10)));
    }
}
