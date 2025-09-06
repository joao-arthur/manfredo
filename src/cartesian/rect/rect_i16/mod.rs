use crate::cartesian::{point::point_i16, rect::rect_i8::RectI8};
use std::ops::RangeInclusive;

pub mod add;
pub mod contains_point;
pub mod contains_rect;
pub mod deflate;
pub mod inflate;
pub mod resize;
pub mod translate;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct RectI16 {
    pub min: point_i16::PointI16,
    pub max: point_i16::PointI16,
}

impl RectI16 {
    pub fn of(x1: i16, y1: i16, x2: i16, y2: i16) -> Self {
        RectI16 { min: point_i16::PointI16::of(x1, y1), max: point_i16::PointI16::of(x2, y2) }
    }

    pub fn largest() -> Self {
        RectI16 { min: point_i16::PointI16::min(), max: point_i16::PointI16::max() }
    }

    pub fn min() -> Self {
        RectI16 { min: point_i16::PointI16::min(), max: point_i16::PointI16::min() }
    }

    pub fn max() -> Self {
        RectI16 { min: point_i16::PointI16::max(), max: point_i16::PointI16::max() }
    }

    pub fn iter_x(&self) -> RangeInclusive<i16> {
        self.min.x..=self.max.x
    }

    pub fn iter_y(&self) -> RangeInclusive<i16> {
        self.min.y..=self.max.y
    }
}

impl From<RectI8> for RectI16 {
    fn from(r: RectI8) -> Self {
        RectI16 { min: point_i16::PointI16::of(r.min.x.into(), r.min.y.into()), max: point_i16::PointI16::of(r.max.x.into(), r.max.y.into()) }
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

#[cfg(test)]
mod tests {
    use super::{RectI16, delta_x, delta_y, len_x, len_y, max_delta, max_len};
    use crate::cartesian::{point::point_i16::PointI16, rect::rect_i8::RectI8};

    #[test]
    fn rect_i16() {
        assert_eq!(RectI16::largest(), RectI16 { min: PointI16 { x: i16::MIN, y: i16::MIN }, max: PointI16 { x: i16::MAX, y: i16::MAX } });
        assert_eq!(RectI16::min(), RectI16 { min: PointI16 { x: i16::MIN, y: i16::MIN }, max: PointI16 { x: i16::MIN, y: i16::MIN } });
        assert_eq!(RectI16::max(), RectI16 { min: PointI16 { x: i16::MAX, y: i16::MAX }, max: PointI16 { x: i16::MAX, y: i16::MAX } });
        assert_eq!(RectI16::of(i16::MIN, -1, 1, i16::MAX), RectI16 { min: PointI16 { x: i16::MIN, y: -1 }, max: PointI16 { x: 1, y: i16::MAX } });
    }

    #[test]
    fn to_string() {
        assert_eq!(RectI16::largest().to_string(), "((-32768, -32768), (32767, 32767))");
        assert_eq!(RectI16::of(i16::MIN, -0, 0, i16::MAX).to_string(), "((-32768, 0), (0, 32767))");
    }

    #[test]
    fn from() {
        assert_eq!(RectI16::from(RectI8::largest()), RectI16 { min: PointI16 { x: i8::MIN.into(), y: i8::MIN.into() }, max: PointI16 { x: i8::MAX.into(), y: i8::MAX.into() } });
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
}
