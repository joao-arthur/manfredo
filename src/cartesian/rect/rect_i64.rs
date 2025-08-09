use std::ops::RangeInclusive;

use crate::cartesian::{
    point::point_i64,
    rect::{rect_i8::RectI8, rect_i16::RectI16, rect_i32::RectI32},
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct RectI64 {
    pub min: point_i64::PointI64,
    pub max: point_i64::PointI64,
}

impl RectI64 {
    pub fn of(x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
        RectI64 { min: point_i64::PointI64::of(x1, y1), max: point_i64::PointI64::of(x2, y2) }
    }

    pub fn iter_x(&self) -> RangeInclusive<i64> {
        self.min.x..=self.max.x
    }

    pub fn iter_y(&self) -> RangeInclusive<i64> {
        self.min.y..=self.max.y
    }
}

impl From<RectI8> for RectI64 {
    fn from(r: RectI8) -> Self {
        RectI64 { min: point_i64::PointI64::of(r.min.x.into(), r.min.y.into()), max: point_i64::PointI64::of(r.max.x.into(), r.max.y.into()) }
    }
}

impl From<RectI16> for RectI64 {
    fn from(r: RectI16) -> Self {
        RectI64 { min: point_i64::PointI64::of(r.min.x.into(), r.min.y.into()), max: point_i64::PointI64::of(r.max.x.into(), r.max.y.into()) }
    }
}

impl From<RectI32> for RectI64 {
    fn from(r: RectI32) -> Self {
        RectI64 { min: point_i64::PointI64::of(r.min.x.into(), r.min.y.into()), max: point_i64::PointI64::of(r.max.x.into(), r.max.y.into()) }
    }
}

impl std::fmt::Display for RectI64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_x(r: &RectI64) -> u64 {
    point_i64::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectI64) -> u64 {
    point_i64::delta_y(&r.min, &r.max)
}

pub fn max_delta(r: &RectI64) -> u64 {
    std::cmp::max(delta_x(r), delta_y(r))
}

pub fn len_x(r: &RectI64) -> u64 {
    delta_x(r) + 1
}

pub fn len_y(r: &RectI64) -> u64 {
    delta_y(r) + 1
}

pub fn max_len(r: &RectI64) -> u64 {
    std::cmp::max(len_x(r), len_y(r))
}

pub fn inflate(r: &mut RectI64) {
    let is_min_x = r.min.x == i64::MIN;
    let is_min_y = r.min.y == i64::MIN;
    let is_max_x = r.max.x == i64::MAX;
    let is_max_y = r.max.y == i64::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return;
    }
    let min_x_modifier = 1 - i64::from(is_min_x) + i64::from(is_max_x);
    let min_y_modifier = 1 - i64::from(is_min_y) + i64::from(is_max_y);
    let max_x_modifier = 1 + i64::from(is_min_x) - i64::from(is_max_x);
    let max_y_modifier = 1 + i64::from(is_min_y) - i64::from(is_max_y);
    r.min.x = r.min.x.saturating_sub(min_x_modifier);
    r.min.y = r.min.y.saturating_sub(min_y_modifier);
    r.max.x = r.max.x.saturating_add(max_x_modifier);
    r.max.y = r.max.y.saturating_add(max_y_modifier);
}

pub fn deflate(r: &mut RectI64) {
    if delta_x(r) < 3 || delta_y(r) < 3 {
        return;
    }
    r.min.x += 1;
    r.min.y += 1;
    r.max.x -= 1;
    r.max.y -= 1;
}

pub fn resize(r: &mut RectI64, size: u64) {
    if size < 3 {
        return;
    }
    let diff_x = i128::from(delta_x(r)) + 1 - i128::from(size);
    let diff_y = i128::from(delta_y(r)) + 1 - i128::from(size);
    let temp_min_x = i128::from(r.min.x) + diff_x / 2;
    let temp_min_y = i128::from(r.min.y) + diff_y / 2;
    let min_x = temp_min_x.clamp(i128::from(i64::MIN), i128::from(i64::MAX) - i128::from(size) + 1);
    let min_y = temp_min_y.clamp(i128::from(i64::MIN), i128::from(i64::MAX) - i128::from(size) + 1);
    r.min.x = min_x as i64;
    r.min.y = min_y as i64;
    r.max.x = (min_x + i128::from(size) - 1) as i64;
    r.max.y = (min_y + i128::from(size) - 1) as i64;
}

pub fn saturating_translate(r: &mut RectI64, delta: &point_i64::PointI64) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = i128::from(r.min.x) + i128::from(delta.x);
    let temp_min_y = i128::from(r.min.y) + i128::from(delta.y);
    let min_x = temp_min_x.clamp(i128::from(i64::MIN), i128::from(i64::MAX) - i128::from(dx));
    let min_y = temp_min_y.clamp(i128::from(i64::MIN), i128::from(i64::MAX) - i128::from(dy));
    r.min.x = min_x as i64;
    r.min.y = min_y as i64;
    r.max.x = (min_x + i128::from(dx)) as i64;
    r.max.y = (min_y + i128::from(dy)) as i64;
}

pub fn try_checked_translate(r: &mut RectI64, delta: &point_i64::PointI64) -> Result<(), ()> {
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

pub fn contains(r: &RectI64, p: &point_i64::PointI64) -> bool {
    p.x >= r.min.x && p.x <= r.max.x && p.y >= r.min.y && p.y <= r.max.y
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{
        point::point_i64::PointI64,
        rect::{rect_i8::RectI8, rect_i16::RectI16, rect_i32::RectI32},
    };

    use super::{
        RectI64, try_checked_translate, contains, deflate, delta_x, delta_y, inflate, len_x, len_y, max_delta, max_len, resize, saturating_translate,
    };

    #[test]
    fn rect_i64() {
        assert_eq!(RectI64::of(i64::MIN, -1, 1, i64::MAX), RectI64 { min: PointI64 { x: i64::MIN, y: -1 }, max: PointI64 { x: 1, y: i64::MAX } });
    }

    #[test]
    fn from() {
        assert_eq!(
            RectI64::from(RectI8::of(0, 0, i8::MAX, i8::MAX)),
            RectI64 { min: PointI64 { x: 0, y: 0 }, max: PointI64 { x: i8::MAX.into(), y: i8::MAX.into() } }
        );
        assert_eq!(
            RectI64::from(RectI16::of(0, 0, i16::MAX, i16::MAX)),
            RectI64 { min: PointI64 { x: 0, y: 0 }, max: PointI64 { x: i16::MAX.into(), y: i16::MAX.into() } }
        );
        assert_eq!(
            RectI64::from(RectI32::of(0, 0, i32::MAX, i32::MAX)),
            RectI64 { min: PointI64 { x: 0, y: 0 }, max: PointI64 { x: i32::MAX.into(), y: i32::MAX.into() } }
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(RectI64::of(i64::MIN, -0, 0, i64::MAX).to_string(), "((-9223372036854775808, 0), (0, 9223372036854775807))");
    }

    #[test]
    fn iter_x() {
        assert_eq!(RectI64::of(-6, -8, -7, -6).iter_x().collect::<Vec<i64>>(), []);
        assert_eq!(RectI64::of(-6, -8, -6, -6).iter_x().collect::<Vec<i64>>(), [-6]);
        assert_eq!(RectI64::of(-6, -8, -5, -6).iter_x().collect::<Vec<i64>>(), [-6, -5]);
        assert_eq!(RectI64::of(-6, -8, -4, -6).iter_x().collect::<Vec<i64>>(), [-6, -5, -4]);
        assert_eq!(RectI64::of(-6, -8, -3, -6).iter_x().collect::<Vec<i64>>(), [-6, -5, -4, -3]);
        assert_eq!(RectI64::of(-6, -8, -3, -6).iter_x().rev().collect::<Vec<i64>>(), [-3, -4, -5, -6]);
        assert_eq!(RectI64::of(-6, -8, -4, -6).iter_x().rev().collect::<Vec<i64>>(), [-4, -5, -6]);
        assert_eq!(RectI64::of(-6, -8, -5, -6).iter_x().rev().collect::<Vec<i64>>(), [-5, -6]);
        assert_eq!(RectI64::of(-6, -8, -6, -6).iter_x().rev().collect::<Vec<i64>>(), [-6]);
        assert_eq!(RectI64::of(-6, -8, -7, -6).iter_x().rev().collect::<Vec<i64>>(), []);
    }

    #[test]
    fn iter_y() {
        assert_eq!(RectI64::of(-6, -8, -4, -9).iter_y().collect::<Vec<i64>>(), []);
        assert_eq!(RectI64::of(-6, -8, -4, -8).iter_y().collect::<Vec<i64>>(), [-8]);
        assert_eq!(RectI64::of(-6, -8, -4, -7).iter_y().collect::<Vec<i64>>(), [-8, -7]);
        assert_eq!(RectI64::of(-6, -8, -4, -6).iter_y().collect::<Vec<i64>>(), [-8, -7, -6]);
        assert_eq!(RectI64::of(-6, -8, -4, -5).iter_y().collect::<Vec<i64>>(), [-8, -7, -6, -5]);
        assert_eq!(RectI64::of(-6, -8, -4, -5).iter_y().rev().collect::<Vec<i64>>(), [-5, -6, -7, -8]);
        assert_eq!(RectI64::of(-6, -8, -4, -6).iter_y().rev().collect::<Vec<i64>>(), [-6, -7, -8]);
        assert_eq!(RectI64::of(-6, -8, -4, -7).iter_y().rev().collect::<Vec<i64>>(), [-7, -8]);
        assert_eq!(RectI64::of(-6, -8, -4, -8).iter_y().rev().collect::<Vec<i64>>(), [-8]);
        assert_eq!(RectI64::of(-6, -8, -4, -9).iter_y().rev().collect::<Vec<i64>>(), []);
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectI64::of(0, i64::MIN, 0, i64::MAX)), 0);
        assert_eq!(delta_x(&RectI64::of(i64::MIN, 0, i64::MAX, 0)), u64::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectI64::of(i64::MIN, 0, i64::MAX, 0)), 0);
        assert_eq!(delta_y(&RectI64::of(0, i64::MIN, 0, i64::MAX)), u64::MAX);
    }

    #[test]
    fn test_max_delta() {
        assert_eq!(max_delta(&RectI64::of(0, 5, 10, 10)), 10);
        assert_eq!(max_delta(&RectI64::of(-10, -10, -5, 0)), 10);
        assert_eq!(max_delta(&RectI64::of(-5, 0, 5, 5)), 10);
    }

    #[test]
    fn max_delta_0() {
        assert_eq!(max_delta(&RectI64::of(0, 0, 0, 0)), 0);
        assert_eq!(max_delta(&RectI64::of(1, 1, 1, 1)), 0);
        assert_eq!(max_delta(&RectI64::of(-1, -1, -1, -1)), 0);
        assert_eq!(max_delta(&RectI64::of(5, 10, 5, 10)), 0);
    }

    #[test]
    fn max_delta_1() {
        assert_eq!(max_delta(&RectI64::of(0, 0, 1, 1)), 1);
        assert_eq!(max_delta(&RectI64::of(5, 5, 6, 6)), 1);
        assert_eq!(max_delta(&RectI64::of(-6, -6, -5, -5)), 1);
        assert_eq!(max_delta(&RectI64::of(0, 0, 0, 1)), 1);
        assert_eq!(max_delta(&RectI64::of(5, 9, 5, 10)), 1);
    }

    #[test]
    fn max_delta_bounds() {
        assert_eq!(max_delta(&RectI64::of(i64::MIN + 1, i64::MIN, i64::MAX, i64::MAX)), u64::MAX);
        assert_eq!(max_delta(&RectI64::of(i64::MIN, i64::MIN + 1, i64::MAX, i64::MAX)), u64::MAX);
        assert_eq!(max_delta(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX)), u64::MAX);
        assert_eq!(max_delta(&RectI64::of(i64::MIN, i64::MIN, i64::MAX, i64::MAX - 1)), u64::MAX);
    }

    #[test]
    fn test_len_x() {
        assert_eq!(len_x(&RectI64::of(0, i64::MIN, 0, i64::MAX)), 1);
        assert_eq!(len_x(&RectI64::of(i64::MIN, 0, i64::MAX - 1, 0)), u64::MAX);
    }

    #[test]
    fn test_len_y() {
        assert_eq!(len_y(&RectI64::of(i64::MIN, 0, i64::MAX, 0)), 1);
        assert_eq!(len_y(&RectI64::of(0, i64::MIN, 0, i64::MAX - 1)), u64::MAX);
    }

    #[test]
    fn test_max_len() {
        assert_eq!(max_len(&RectI64::of(0, 5, 10, 10)), 11);
        assert_eq!(max_len(&RectI64::of(-10, -10, -5, 0)), 11);
        assert_eq!(max_len(&RectI64::of(-5, 0, 5, 5)), 11);
    }

    #[test]
    fn max_len_1() {
        assert_eq!(max_len(&RectI64::of(0, 0, 0, 0)), 1);
        assert_eq!(max_len(&RectI64::of(1, 1, 1, 1)), 1);
        assert_eq!(max_len(&RectI64::of(-1, -1, -1, -1)), 1);
        assert_eq!(max_len(&RectI64::of(5, 10, 5, 10)), 1);
    }

    #[test]
    fn max_len_2() {
        assert_eq!(max_len(&RectI64::of(0, 0, 1, 1)), 2);
        assert_eq!(max_len(&RectI64::of(5, 5, 6, 6)), 2);
        assert_eq!(max_len(&RectI64::of(-6, -6, -5, -5)), 2);
        assert_eq!(max_len(&RectI64::of(0, 0, 0, 1)), 2);
        assert_eq!(max_len(&RectI64::of(5, 9, 5, 10)), 2);
    }

    #[test]
    fn max_len_bounds() {
        assert_eq!(max_len(&RectI64::of(i64::MIN + 1, i64::MIN, i64::MAX - 1, i64::MAX - 1)), u64::MAX);
        assert_eq!(max_len(&RectI64::of(i64::MIN, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1)), u64::MAX);
        assert_eq!(max_len(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 1)), u64::MAX);
        assert_eq!(max_len(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 2)), u64::MAX);
    }

    #[test]
    fn inflate_min_bounds() {
        let mut r = RectI64::of(i64::MIN + 7, i64::MIN + 2, 4, 13);
        inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 6, i64::MIN + 1, 5, 14));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 5, i64::MIN, 6, 15));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 4, i64::MIN, 7, 17));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 3, i64::MIN, 8, 19));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 2, i64::MIN, 9, 21));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN + 1, i64::MIN, 10, 23));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, 11, 25));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, 13, 27));
    }

    #[test]
    fn inflate_max_bounds() {
        let mut r = RectI64::of(-100, 30, i64::MAX - 5, i64::MAX - 3);
        inflate(&mut r);
        assert_eq!(r, RectI64::of(-101, 29, i64::MAX - 4, i64::MAX - 2));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(-102, 28, i64::MAX - 3, i64::MAX - 1));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(-103, 27, i64::MAX - 2, i64::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(-104, 25, i64::MAX - 1, i64::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(-105, 23, i64::MAX, i64::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(-107, 21, i64::MAX, i64::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(-109, 19, i64::MAX, i64::MAX));
        inflate(&mut r);
        assert_eq!(r, RectI64::of(-111, 17, i64::MAX, i64::MAX));
    }

    #[test]
    fn inflate_almost_min_bounds() {
        let mut r = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX);
        inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MAX, i64::MAX));
    }

    #[test]
    fn inflate_almost_max_bounds() {
        let mut r = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1);
        inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MAX, i64::MAX));
    }

    #[test]
    fn inflate_max_width() {
        let mut r = RectI64::of(i64::MIN, 10, i64::MAX, 50);
        inflate(&mut r);
        assert_eq!(r, RectI64::of(i64::MIN, 10, i64::MAX, 50));
    }

    #[test]
    fn inflate_max_height() {
        let mut r = RectI64::of(10, i64::MIN, 50, i64::MAX);
        inflate(&mut r);
        assert_eq!(r, RectI64::of(10, i64::MIN, 50, i64::MAX));
    }

    #[test]
    fn deflate_odd() {
        let mut r = RectI64::of(-5, -5, 5, 5);
        deflate(&mut r);
        assert_eq!(r, RectI64::of(-4, -4, 4, 4));
        deflate(&mut r);
        assert_eq!(r, RectI64::of(-3, -3, 3, 3));
        deflate(&mut r);
        assert_eq!(r, RectI64::of(-2, -2, 2, 2));
        deflate(&mut r);
        assert_eq!(r, RectI64::of(-1, -1, 1, 1));
        deflate(&mut r);
        assert_eq!(r, RectI64::of(-1, -1, 1, 1));
    }

    #[test]
    fn deflate_even() {
        let mut r = RectI64::of(-5, -5, 4, 4);
        deflate(&mut r);
        assert_eq!(r, RectI64::of(-4, -4, 3, 3));
        deflate(&mut r);
        assert_eq!(r, RectI64::of(-3, -3, 2, 2));
        deflate(&mut r);
        assert_eq!(r, RectI64::of(-2, -2, 1, 1));
        deflate(&mut r);
        assert_eq!(r, RectI64::of(-1, -1, 0, 0));
        deflate(&mut r);
        assert_eq!(r, RectI64::of(-1, -1, 0, 0));
    }

    #[test]
    fn resize_odd() {
        let mut r = RectI64::of(-5, -5, 5, 5);
        resize(&mut r, 11);
        assert_eq!(r, RectI64::of(-5, -5, 5, 5));
        resize(&mut r, 9);
        assert_eq!(r, RectI64::of(-4, -4, 4, 4));
        resize(&mut r, 7);
        assert_eq!(r, RectI64::of(-3, -3, 3, 3));
        resize(&mut r, 5);
        assert_eq!(r, RectI64::of(-2, -2, 2, 2));
        resize(&mut r, 3);
        assert_eq!(r, RectI64::of(-1, -1, 1, 1));
        resize(&mut r, 1);
        assert_eq!(r, RectI64::of(-1, -1, 1, 1));
        resize(&mut r, 1);
        assert_eq!(r, RectI64::of(-1, -1, 1, 1));
        resize(&mut r, 3);
        assert_eq!(r, RectI64::of(-1, -1, 1, 1));
        resize(&mut r, 5);
        assert_eq!(r, RectI64::of(-2, -2, 2, 2));
        resize(&mut r, 7);
        assert_eq!(r, RectI64::of(-3, -3, 3, 3));
    }

    #[test]
    fn resize_even() {
        let mut r = RectI64::of(-5, -5, 4, 4);
        resize(&mut r, 10);
        assert_eq!(r, RectI64::of(-5, -5, 4, 4));
        resize(&mut r, 8);
        assert_eq!(r, RectI64::of(-4, -4, 3, 3));
        resize(&mut r, 6);
        assert_eq!(r, RectI64::of(-3, -3, 2, 2));
        resize(&mut r, 4);
        assert_eq!(r, RectI64::of(-2, -2, 1, 1));
        resize(&mut r, 2);
        assert_eq!(r, RectI64::of(-2, -2, 1, 1));
        resize(&mut r, 2);
        assert_eq!(r, RectI64::of(-2, -2, 1, 1));
        resize(&mut r, 4);
        assert_eq!(r, RectI64::of(-2, -2, 1, 1));
        resize(&mut r, 6);
        assert_eq!(r, RectI64::of(-3, -3, 2, 2));
    }

    #[test]
    fn resize_even_2nd_scenario() {
        let mut r = RectI64::of(-4, -4, 5, 5);
        resize(&mut r, 10);
        assert_eq!(r, RectI64::of(-4, -4, 5, 5));
        resize(&mut r, 8);
        assert_eq!(r, RectI64::of(-3, -3, 4, 4));
        resize(&mut r, 6);
        assert_eq!(r, RectI64::of(-2, -2, 3, 3));
        resize(&mut r, 4);
        assert_eq!(r, RectI64::of(-1, -1, 2, 2));
        resize(&mut r, 2);
        assert_eq!(r, RectI64::of(-1, -1, 2, 2));
        resize(&mut r, 2);
        assert_eq!(r, RectI64::of(-1, -1, 2, 2));
        resize(&mut r, 4);
        assert_eq!(r, RectI64::of(-1, -1, 2, 2));
        resize(&mut r, 6);
        assert_eq!(r, RectI64::of(-2, -2, 3, 3));
    }

    #[test]
    fn resize_odd_min_bounds_big_delta() {
        let mut r = RectI64::of(i64::MIN, i64::MIN, i64::MIN + 2, i64::MIN + 2);
        resize(&mut r, u64::MAX);
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1));
    }

    #[test]
    fn resize_even_min_bounds_big_delta() {
        let mut r = RectI64::of(i64::MIN, i64::MIN, i64::MIN + 3, i64::MIN + 3);
        resize(&mut r, u64::MAX - 1);
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 2, i64::MAX - 2));
    }

    #[test]
    fn resize_odd_max_bounds_big_delta() {
        let mut r = RectI64::of(i64::MAX - 2, i64::MAX - 2, i64::MAX, i64::MAX);
        resize(&mut r, u64::MAX);
        assert_eq!(r, RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX));
    }

    #[test]
    fn resize_even_max_bounds_big_delta() {
        let mut r = RectI64::of(i64::MAX - 3, i64::MAX - 3, i64::MAX, i64::MAX);
        resize(&mut r, u64::MAX - 1);
        assert_eq!(r, RectI64::of(i64::MIN + 2, i64::MIN + 2, i64::MAX, i64::MAX));
    }

    #[test]
    fn test_saturating_translate() {
        let mut r = RectI64::of(0, 0, 10, 10);
        saturating_translate(&mut r, &PointI64::of(10, 20));
        assert_eq!(r, RectI64::of(10, 20, 20, 30));
        saturating_translate(&mut r, &PointI64::of(-20, -15));
        assert_eq!(r, RectI64::of(-10, 5, 0, 15));
        saturating_translate(&mut r, &PointI64::of(3, -2));
        assert_eq!(r, RectI64::of(-7, 3, 3, 13));
    }

    #[test]
    fn saturating_translate_min_bounds() {
        let mut r = RectI64::of(i64::MIN + 5, i64::MIN + 10, 12, 15);
        saturating_translate(&mut r, &PointI64::of(-10, -10));
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, 7, 5));
    }

    #[test]
    fn saturating_translate_max_bounds() {
        let mut r = RectI64::of(40, 35, i64::MAX - 5, i64::MAX - 10);
        saturating_translate(&mut r, &PointI64::of(20, 20));
        assert_eq!(r, RectI64::of(45, 45, i64::MAX, i64::MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_big_delta() {
        let mut r = RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 10);
        saturating_translate(&mut r, &PointI64::min());
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MIN + 10, i64::MIN + 10));
    }

    #[test]
    fn saturating_translate_max_bounds_big_delta() {
        let mut r = RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX, i64::MAX);
        saturating_translate(&mut r, &PointI64::max());
        assert_eq!(r, RectI64::of(i64::MAX - 10, i64::MAX - 10, i64::MAX, i64::MAX));
    }

    #[test]
    fn saturating_translate_min_bounds_big_rect_big_delta() {
        let mut r = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX);
        saturating_translate(&mut r, &PointI64::min());
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1));
    }

    #[test]
    fn saturating_translate_max_bounds_big_rect_big_delta() {
        let mut r = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1);
        saturating_translate(&mut r, &PointI64::max());
        assert_eq!(r, RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX));
    }

    #[test]
    fn test_checked_translate() {
        let mut r = RectI64::of(0, 0, 10, 10);
        assert_eq!(try_checked_translate(&mut r, &PointI64::of(10, 20)), Ok(()));
        assert_eq!(r, RectI64::of(10, 20, 20, 30));
        assert_eq!(try_checked_translate(&mut r, &PointI64::of(-20, -15)), Ok(()));
        assert_eq!(r, RectI64::of(-10, 5, 0, 15));
        assert_eq!(try_checked_translate(&mut r, &PointI64::of(3, -2)), Ok(()));
        assert_eq!(r, RectI64::of(-7, 3, 3, 13));
    }

    #[test]
    fn checked_translate_min_bounds() {
        let mut r = RectI64::of(i64::MIN + 5, i64::MIN + 10, 12, 15);
        assert_eq!(try_checked_translate(&mut r, &PointI64::of(-10, -10)), Err(()));
        assert_eq!(r, RectI64::of(i64::MIN + 5, i64::MIN + 10, 12, 15));
    }

    #[test]
    fn checked_translate_max_bounds() {
        let mut r = RectI64::of(40, 35, i64::MAX - 5, i64::MAX - 10);
        assert_eq!(try_checked_translate(&mut r, &PointI64::of(20, 20)), Err(()));
        assert_eq!(r, RectI64::of(40, 35, i64::MAX - 5, i64::MAX - 10));
    }

    #[test]
    fn checked_translate_min_bounds_big_rect_big_delta() {
        let mut r = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX);
        assert_eq!(try_checked_translate(&mut r, &PointI64::min()), Err(()));
        assert_eq!(try_checked_translate(&mut r, &PointI64::of(i64::MIN, 0)), Err(()));
        assert_eq!(try_checked_translate(&mut r, &PointI64::of(0, i64::MIN)), Err(()));
        assert_eq!(r, RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX));
    }

    #[test]
    fn checked_translate_max_bounds_big_rect_big_delta() {
        let mut r = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1);
        assert_eq!(try_checked_translate(&mut r, &PointI64::max()), Err(()));
        assert_eq!(try_checked_translate(&mut r, &PointI64::of(i64::MAX, 0)), Err(()));
        assert_eq!(try_checked_translate(&mut r, &PointI64::of(0, i64::MAX)), Err(()));
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1));
    }

    #[test]
    fn checked_translate_min_bounds_big_rect_small_delta() {
        let mut r = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX);
        assert_eq!(try_checked_translate(&mut r, &PointI64::of(-1, -1)), Ok(()));
        assert_eq!(r, RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1));
    }

    #[test]
    fn checked_translate_max_bounds_big_rect_small_delta() {
        let mut r = RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1);
        assert_eq!(try_checked_translate(&mut r, &PointI64::of(1, 1)), Ok(()));
        assert_eq!(r, RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX));
    }

    #[test]
    fn contains_inside_borders() {
        let r = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1);
        assert!(contains(&r, &PointI64::of(i64::MIN + 1, i64::MIN + 1)));
        assert!(contains(&r, &PointI64::of(i64::MIN + 1, i64::MAX - 1)));
        assert!(contains(&r, &PointI64::of(i64::MAX - 1, i64::MIN + 1)));
        assert!(contains(&r, &PointI64::of(i64::MAX - 1, i64::MAX - 1)));
    }

    #[test]
    fn contains_outside_borders() {
        let r = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1);
        assert!(!contains(&r, &PointI64::min()));
        assert!(!contains(&r, &PointI64::of(i64::MIN, i64::MAX)));
        assert!(!contains(&r, &PointI64::of(i64::MAX, i64::MIN)));
        assert!(!contains(&r, &PointI64::max()));
        assert!(!contains(&r, &PointI64::of(i64::MIN + 1, i64::MIN)));
        assert!(!contains(&r, &PointI64::of(i64::MIN + 1, i64::MAX)));
        assert!(!contains(&r, &PointI64::of(i64::MAX - 1, i64::MIN)));
        assert!(!contains(&r, &PointI64::of(i64::MAX - 1, i64::MAX)));
        assert!(!contains(&r, &PointI64::of(i64::MIN, i64::MIN + 1)));
        assert!(!contains(&r, &PointI64::of(i64::MIN, i64::MAX - 1)));
        assert!(!contains(&r, &PointI64::of(i64::MAX, i64::MIN + 1)));
        assert!(!contains(&r, &PointI64::of(i64::MAX, i64::MAX - 1)));
    }

    #[test]
    fn contains_inside() {
        let r = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1);
        assert!(contains(&r, &PointI64::of(i64::MIN + 10, i64::MIN + 10)));
        assert!(contains(&r, &PointI64::of(i64::MAX - 10, i64::MAX - 10)));
    }
}
