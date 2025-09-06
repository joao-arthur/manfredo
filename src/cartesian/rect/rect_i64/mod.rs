use crate::cartesian::{
    point::point_i64,
    rect::{rect_i8::RectI8, rect_i16::RectI16, rect_i32::RectI32},
};
use std::ops::RangeInclusive;

pub mod add;
pub mod contains_point;
pub mod contains_rect;
pub mod deflate;
pub mod inflate;
pub mod resize;
pub mod translate;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct RectI64 {
    pub min: point_i64::PointI64,
    pub max: point_i64::PointI64,
}

impl RectI64 {
    pub fn of(x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
        RectI64 { min: point_i64::PointI64::of(x1, y1), max: point_i64::PointI64::of(x2, y2) }
    }

    pub fn largest() -> Self {
        RectI64 { min: point_i64::PointI64::min(), max: point_i64::PointI64::max() }
    }

    pub fn min() -> Self {
        RectI64 { min: point_i64::PointI64::min(), max: point_i64::PointI64::min() }
    }

    pub fn max() -> Self {
        RectI64 { min: point_i64::PointI64::max(), max: point_i64::PointI64::max() }
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

#[cfg(test)]
mod tests {
    use super::{RectI64, delta_x, delta_y, len_x, len_y, max_delta, max_len};
    use crate::cartesian::{
        point::point_i64::PointI64,
        rect::{rect_i8::RectI8, rect_i16::RectI16, rect_i32::RectI32},
    };

    #[test]
    fn rect_i64() {
        assert_eq!(RectI64::largest(), RectI64 { min: PointI64 { x: i64::MIN, y: i64::MIN }, max: PointI64 { x: i64::MAX, y: i64::MAX } });
        assert_eq!(RectI64::min(), RectI64 { min: PointI64 { x: i64::MIN, y: i64::MIN }, max: PointI64 { x: i64::MIN, y: i64::MIN } });
        assert_eq!(RectI64::max(), RectI64 { min: PointI64 { x: i64::MAX, y: i64::MAX }, max: PointI64 { x: i64::MAX, y: i64::MAX } });
        assert_eq!(RectI64::of(i64::MIN, -1, 1, i64::MAX), RectI64 { min: PointI64 { x: i64::MIN, y: -1 }, max: PointI64 { x: 1, y: i64::MAX } });
    }

    #[test]
    fn to_string() {
        assert_eq!(RectI64::largest().to_string(), "((-9223372036854775808, -9223372036854775808), (9223372036854775807, 9223372036854775807))");
        assert_eq!(RectI64::of(i64::MIN, -0, 0, i64::MAX).to_string(), "((-9223372036854775808, 0), (0, 9223372036854775807))");
    }

    #[test]
    fn from() {
        assert_eq!(RectI64::from(RectI8::largest()), RectI64 { min: PointI64 { x: i8::MIN.into(), y: i8::MIN.into() }, max: PointI64 { x: i8::MAX.into(), y: i8::MAX.into() } });
        assert_eq!(RectI64::from(RectI16::largest()), RectI64 { min: PointI64 { x: i16::MIN.into(), y: i16::MIN.into() }, max: PointI64 { x: i16::MAX.into(), y: i16::MAX.into() } });
        assert_eq!(RectI64::from(RectI32::largest()), RectI64 { min: PointI64 { x: i32::MIN.into(), y: i32::MIN.into() }, max: PointI64 { x: i32::MAX.into(), y: i32::MAX.into() } });
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
}
