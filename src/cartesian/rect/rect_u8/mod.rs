use crate::cartesian::point::point_u8;
use std::ops::RangeInclusive;

pub mod add;
pub mod contains_point;
pub mod contains_rect;
pub mod deflate;
pub mod inflate;
pub mod resize;
pub mod translate;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct RectU8 {
    pub min: point_u8::PointU8,
    pub max: point_u8::PointU8,
}

impl RectU8 {
    pub fn of(x1: u8, y1: u8, x2: u8, y2: u8) -> Self {
        RectU8 { min: point_u8::PointU8::of(x1, y1), max: point_u8::PointU8::of(x2, y2) }
    }

    pub fn largest() -> Self {
        RectU8 { min: point_u8::PointU8::min(), max: point_u8::PointU8::max() }
    }

    pub fn min() -> Self {
        RectU8 { min: point_u8::PointU8::min(), max: point_u8::PointU8::min() }
    }

    pub fn max() -> Self {
        RectU8 { min: point_u8::PointU8::max(), max: point_u8::PointU8::max() }
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

#[cfg(test)]
mod tests {
    use super::{RectU8, delta_x, delta_y, len_x, len_y, max_delta, max_len};
    use crate::cartesian::point::point_u8::PointU8;

    #[test]
    fn rect_u8() {
        assert_eq!(RectU8::largest(), RectU8 { min: PointU8 { x: 0, y: 0 }, max: PointU8 { x: u8::MAX, y: u8::MAX } });
        assert_eq!(RectU8::min(), RectU8 { min: PointU8 { x: 0, y: 0 }, max: PointU8 { x: 0, y: 0 } });
        assert_eq!(RectU8::max(), RectU8 { min: PointU8 { x: u8::MAX, y: u8::MAX }, max: PointU8 { x: u8::MAX, y: u8::MAX } });
        assert_eq!(RectU8::of(0, 2, 4, 8), RectU8 { min: PointU8 { x: 0, y: 2 }, max: PointU8 { x: 4, y: 8 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(RectU8::largest().to_string(), "((0, 0), (255, 255))");
        assert_eq!(RectU8::of(0, 2, 4, 8).to_string(), "((0, 2), (4, 8))");
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
}
