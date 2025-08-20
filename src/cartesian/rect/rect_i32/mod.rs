use crate::cartesian::{
    point::point_i32,
    rect::{rect_i8::RectI8, rect_i16::RectI16},
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
pub struct RectI32 {
    pub min: point_i32::PointI32,
    pub max: point_i32::PointI32,
}

impl RectI32 {
    pub fn of(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        RectI32 { min: point_i32::PointI32::of(x1, y1), max: point_i32::PointI32::of(x2, y2) }
    }

    pub fn largest() -> Self {
        RectI32 { min: point_i32::PointI32::min(), max: point_i32::PointI32::max() }
    }

    pub fn min() -> Self {
        RectI32 { min: point_i32::PointI32::min(), max: point_i32::PointI32::min() }
    }

    pub fn max() -> Self {
        RectI32 { min: point_i32::PointI32::max(), max: point_i32::PointI32::max() }
    }

    pub fn iter_x(&self) -> RangeInclusive<i32> {
        self.min.x..=self.max.x
    }

    pub fn iter_y(&self) -> RangeInclusive<i32> {
        self.min.y..=self.max.y
    }
}

impl From<RectI8> for RectI32 {
    fn from(r: RectI8) -> Self {
        RectI32 { min: point_i32::PointI32::of(r.min.x.into(), r.min.y.into()), max: point_i32::PointI32::of(r.max.x.into(), r.max.y.into()) }
    }
}

impl From<RectI16> for RectI32 {
    fn from(r: RectI16) -> Self {
        RectI32 { min: point_i32::PointI32::of(r.min.x.into(), r.min.y.into()), max: point_i32::PointI32::of(r.max.x.into(), r.max.y.into()) }
    }
}

impl std::fmt::Display for RectI32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_x(r: &RectI32) -> u32 {
    point_i32::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectI32) -> u32 {
    point_i32::delta_y(&r.min, &r.max)
}

pub fn max_delta(r: &RectI32) -> u32 {
    std::cmp::max(delta_x(r), delta_y(r))
}

pub fn len_x(r: &RectI32) -> u32 {
    delta_x(r) + 1
}

pub fn len_y(r: &RectI32) -> u32 {
    delta_y(r) + 1
}

pub fn max_len(r: &RectI32) -> u32 {
    std::cmp::max(len_x(r), len_y(r))
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{
        point::point_i32::PointI32,
        rect::{rect_i8::RectI8, rect_i16::RectI16},
    };

    use super::{RectI32, delta_x, delta_y, len_x, len_y, max_delta, max_len};

    #[test]
    fn rect_i32() {
        assert_eq!(RectI32::largest(), RectI32 { min: PointI32 { x: i32::MIN, y: i32::MIN }, max: PointI32 { x: i32::MAX, y: i32::MAX } });
        assert_eq!(RectI32::min(), RectI32 { min: PointI32 { x: i32::MIN, y: i32::MIN }, max: PointI32 { x: i32::MIN, y: i32::MIN } });
        assert_eq!(RectI32::max(), RectI32 { min: PointI32 { x: i32::MAX, y: i32::MAX }, max: PointI32 { x: i32::MAX, y: i32::MAX } });
        assert_eq!(RectI32::of(i32::MIN, -1, 1, i32::MAX), RectI32 { min: PointI32 { x: i32::MIN, y: -1 }, max: PointI32 { x: 1, y: i32::MAX } });
    }

    #[test]
    fn to_string() {
        assert_eq!(RectI32::largest().to_string(), "((-2147483648, -2147483648), (2147483647, 2147483647))");
        assert_eq!(RectI32::of(i32::MIN, -0, 0, i32::MAX).to_string(), "((-2147483648, 0), (0, 2147483647))");
    }

    #[test]
    fn from() {
        assert_eq!(
            RectI32::from(RectI8::largest()),
            RectI32 { min: PointI32 { x: i8::MIN.into(), y: i8::MIN.into() }, max: PointI32 { x: i8::MAX.into(), y: i8::MAX.into() } }
        );
        assert_eq!(
            RectI32::from(RectI16::largest()),
            RectI32 { min: PointI32 { x: i16::MIN.into(), y: i16::MIN.into() }, max: PointI32 { x: i16::MAX.into(), y: i16::MAX.into() } }
        );
    }

    #[test]
    fn iter_x() {
        assert_eq!(RectI32::of(-6, -8, -7, -6).iter_x().collect::<Vec<i32>>(), []);
        assert_eq!(RectI32::of(-6, -8, -6, -6).iter_x().collect::<Vec<i32>>(), [-6]);
        assert_eq!(RectI32::of(-6, -8, -5, -6).iter_x().collect::<Vec<i32>>(), [-6, -5]);
        assert_eq!(RectI32::of(-6, -8, -4, -6).iter_x().collect::<Vec<i32>>(), [-6, -5, -4]);
        assert_eq!(RectI32::of(-6, -8, -3, -6).iter_x().collect::<Vec<i32>>(), [-6, -5, -4, -3]);
        assert_eq!(RectI32::of(-6, -8, -3, -6).iter_x().rev().collect::<Vec<i32>>(), [-3, -4, -5, -6]);
        assert_eq!(RectI32::of(-6, -8, -4, -6).iter_x().rev().collect::<Vec<i32>>(), [-4, -5, -6]);
        assert_eq!(RectI32::of(-6, -8, -5, -6).iter_x().rev().collect::<Vec<i32>>(), [-5, -6]);
        assert_eq!(RectI32::of(-6, -8, -6, -6).iter_x().rev().collect::<Vec<i32>>(), [-6]);
        assert_eq!(RectI32::of(-6, -8, -7, -6).iter_x().rev().collect::<Vec<i32>>(), []);
    }

    #[test]
    fn iter_y() {
        assert_eq!(RectI32::of(-6, -8, -4, -9).iter_y().collect::<Vec<i32>>(), []);
        assert_eq!(RectI32::of(-6, -8, -4, -8).iter_y().collect::<Vec<i32>>(), [-8]);
        assert_eq!(RectI32::of(-6, -8, -4, -7).iter_y().collect::<Vec<i32>>(), [-8, -7]);
        assert_eq!(RectI32::of(-6, -8, -4, -6).iter_y().collect::<Vec<i32>>(), [-8, -7, -6]);
        assert_eq!(RectI32::of(-6, -8, -4, -5).iter_y().collect::<Vec<i32>>(), [-8, -7, -6, -5]);
        assert_eq!(RectI32::of(-6, -8, -4, -5).iter_y().rev().collect::<Vec<i32>>(), [-5, -6, -7, -8]);
        assert_eq!(RectI32::of(-6, -8, -4, -6).iter_y().rev().collect::<Vec<i32>>(), [-6, -7, -8]);
        assert_eq!(RectI32::of(-6, -8, -4, -7).iter_y().rev().collect::<Vec<i32>>(), [-7, -8]);
        assert_eq!(RectI32::of(-6, -8, -4, -8).iter_y().rev().collect::<Vec<i32>>(), [-8]);
        assert_eq!(RectI32::of(-6, -8, -4, -9).iter_y().rev().collect::<Vec<i32>>(), []);
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectI32::of(0, i32::MIN, 0, i32::MAX)), 0);
        assert_eq!(delta_x(&RectI32::of(i32::MIN, 0, i32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectI32::of(i32::MIN, 0, i32::MAX, 0)), 0);
        assert_eq!(delta_y(&RectI32::of(0, i32::MIN, 0, i32::MAX)), u32::MAX);
    }

    #[test]
    fn test_max_delta() {
        assert_eq!(max_delta(&RectI32::of(0, 5, 10, 10)), 10);
        assert_eq!(max_delta(&RectI32::of(-10, -10, -5, 0)), 10);
        assert_eq!(max_delta(&RectI32::of(-5, 0, 5, 5)), 10);
    }

    #[test]
    fn max_delta_0() {
        assert_eq!(max_delta(&RectI32::of(0, 0, 0, 0)), 0);
        assert_eq!(max_delta(&RectI32::of(1, 1, 1, 1)), 0);
        assert_eq!(max_delta(&RectI32::of(-1, -1, -1, -1)), 0);
        assert_eq!(max_delta(&RectI32::of(5, 10, 5, 10)), 0);
    }

    #[test]
    fn max_delta_1() {
        assert_eq!(max_delta(&RectI32::of(0, 0, 1, 1)), 1);
        assert_eq!(max_delta(&RectI32::of(5, 5, 6, 6)), 1);
        assert_eq!(max_delta(&RectI32::of(-6, -6, -5, -5)), 1);
        assert_eq!(max_delta(&RectI32::of(0, 0, 0, 1)), 1);
        assert_eq!(max_delta(&RectI32::of(5, 9, 5, 10)), 1);
    }

    #[test]
    fn max_delta_bounds() {
        assert_eq!(max_delta(&RectI32::of(i32::MIN + 1, i32::MIN, i32::MAX, i32::MAX)), u32::MAX);
        assert_eq!(max_delta(&RectI32::of(i32::MIN, i32::MIN + 1, i32::MAX, i32::MAX)), u32::MAX);
        assert_eq!(max_delta(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX)), u32::MAX);
        assert_eq!(max_delta(&RectI32::of(i32::MIN, i32::MIN, i32::MAX, i32::MAX - 1)), u32::MAX);
    }

    #[test]
    fn test_len_x() {
        assert_eq!(len_x(&RectI32::of(0, i32::MIN, 0, i32::MAX)), 1);
        assert_eq!(len_x(&RectI32::of(i32::MIN, 0, i32::MAX - 1, 0)), u32::MAX);
    }

    #[test]
    fn test_len_y() {
        assert_eq!(len_y(&RectI32::of(i32::MIN, 0, i32::MAX, 0)), 1);
        assert_eq!(len_y(&RectI32::of(0, i32::MIN, 0, i32::MAX - 1)), u32::MAX);
    }

    #[test]
    fn test_max_len() {
        assert_eq!(max_len(&RectI32::of(0, 5, 10, 10)), 11);
        assert_eq!(max_len(&RectI32::of(-10, -10, -5, 0)), 11);
        assert_eq!(max_len(&RectI32::of(-5, 0, 5, 5)), 11);
    }

    #[test]
    fn max_len_1() {
        assert_eq!(max_len(&RectI32::of(0, 0, 0, 0)), 1);
        assert_eq!(max_len(&RectI32::of(1, 1, 1, 1)), 1);
        assert_eq!(max_len(&RectI32::of(-1, -1, -1, -1)), 1);
        assert_eq!(max_len(&RectI32::of(5, 10, 5, 10)), 1);
    }

    #[test]
    fn max_len_2() {
        assert_eq!(max_len(&RectI32::of(0, 0, 1, 1)), 2);
        assert_eq!(max_len(&RectI32::of(5, 5, 6, 6)), 2);
        assert_eq!(max_len(&RectI32::of(-6, -6, -5, -5)), 2);
        assert_eq!(max_len(&RectI32::of(0, 0, 0, 1)), 2);
        assert_eq!(max_len(&RectI32::of(5, 9, 5, 10)), 2);
    }

    #[test]
    fn max_len_bounds() {
        assert_eq!(max_len(&RectI32::of(i32::MIN + 1, i32::MIN, i32::MAX - 1, i32::MAX - 1)), u32::MAX);
        assert_eq!(max_len(&RectI32::of(i32::MIN, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1)), u32::MAX);
        assert_eq!(max_len(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 1)), u32::MAX);
        assert_eq!(max_len(&RectI32::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 2)), u32::MAX);
    }
}
