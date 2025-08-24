use std::ops::RangeInclusive;

pub mod add;
pub mod contains_point;
pub mod contains_rect;
pub mod deflate;
pub mod inflate;
pub mod resize;
pub mod translate;

use crate::cartesian::{
    point::point_u64,
    rect::{rect_u8::RectU8, rect_u16::RectU16, rect_u32::RectU32},
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct RectU64 {
    pub min: point_u64::PointU64,
    pub max: point_u64::PointU64,
}

impl RectU64 {
    pub fn of(x1: u64, y1: u64, x2: u64, y2: u64) -> Self {
        RectU64 { min: point_u64::PointU64::of(x1, y1), max: point_u64::PointU64::of(x2, y2) }
    }

    pub fn largest() -> Self {
        RectU64 { min: point_u64::PointU64::min(), max: point_u64::PointU64::max() }
    }

    pub fn min() -> Self {
        RectU64 { min: point_u64::PointU64::min(), max: point_u64::PointU64::min() }
    }

    pub fn max() -> Self {
        RectU64 { min: point_u64::PointU64::max(), max: point_u64::PointU64::max() }
    }

    pub fn iter_x(&self) -> RangeInclusive<u64> {
        self.min.x..=self.max.x
    }

    pub fn iter_y(&self) -> RangeInclusive<u64> {
        self.min.y..=self.max.y
    }
}

impl From<RectU8> for RectU64 {
    fn from(r: RectU8) -> Self {
        RectU64 { min: point_u64::PointU64::of(r.min.x.into(), r.min.y.into()), max: point_u64::PointU64::of(r.max.x.into(), r.max.y.into()) }
    }
}

impl From<RectU16> for RectU64 {
    fn from(r: RectU16) -> Self {
        RectU64 { min: point_u64::PointU64::of(r.min.x.into(), r.min.y.into()), max: point_u64::PointU64::of(r.max.x.into(), r.max.y.into()) }
    }
}

impl From<RectU32> for RectU64 {
    fn from(r: RectU32) -> Self {
        RectU64 { min: point_u64::PointU64::of(r.min.x.into(), r.min.y.into()), max: point_u64::PointU64::of(r.max.x.into(), r.max.y.into()) }
    }
}

impl std::fmt::Display for RectU64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_x(r: &RectU64) -> u64 {
    point_u64::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectU64) -> u64 {
    point_u64::delta_y(&r.min, &r.max)
}

pub fn max_delta(r: &RectU64) -> u64 {
    std::cmp::max(delta_x(r), delta_y(r))
}

pub fn len_x(r: &RectU64) -> u64 {
    delta_x(r) + 1
}

pub fn len_y(r: &RectU64) -> u64 {
    delta_y(r) + 1
}

pub fn max_len(r: &RectU64) -> u64 {
    std::cmp::max(len_x(r), len_y(r))
}

#[cfg(test)]
mod tests {
    use super::{RectU64, delta_x, delta_y, len_x, len_y, max_delta, max_len};
    use crate::cartesian::{
        point::point_u64::PointU64,
        rect::{rect_u8::RectU8, rect_u16::RectU16, rect_u32::RectU32},
    };

    #[test]
    fn rect_u64() {
        assert_eq!(RectU64::largest(), RectU64 { min: PointU64 { x: 0, y: 0 }, max: PointU64 { x: u64::MAX, y: u64::MAX } });
        assert_eq!(RectU64::min(), RectU64 { min: PointU64 { x: 0, y: 0 }, max: PointU64 { x: 0, y: 0 } });
        assert_eq!(RectU64::max(), RectU64 { min: PointU64 { x: u64::MAX, y: u64::MAX }, max: PointU64 { x: u64::MAX, y: u64::MAX } });
        assert_eq!(RectU64::of(4096, 8192, 16384, 32768), RectU64 { min: PointU64 { x: 4096, y: 8192 }, max: PointU64 { x: 16384, y: 32768 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(RectU64::of(4096, 8192, 16384, 32768).to_string(), "((4096, 8192), (16384, 32768))");
        assert_eq!(RectU64::of(u64::MAX, 0, 0, u64::MAX).to_string(), "((18446744073709551615, 0), (0, 18446744073709551615))");
    }

    #[test]
    fn from() {
        assert_eq!(
            RectU64::from(RectU8::largest()),
            RectU64 { min: PointU64 { x: 0, y: 0 }, max: PointU64 { x: u8::MAX.into(), y: u8::MAX.into() } }
        );
        assert_eq!(
            RectU64::from(RectU16::largest()),
            RectU64 { min: PointU64 { x: 0, y: 0 }, max: PointU64 { x: u16::MAX.into(), y: u16::MAX.into() } }
        );
        assert_eq!(
            RectU64::from(RectU32::largest()),
            RectU64 { min: PointU64 { x: 0, y: 0 }, max: PointU64 { x: u32::MAX.into(), y: u32::MAX.into() } }
        );
    }

    #[test]
    fn iter_x() {
        assert_eq!(RectU64::of(3, 6, 2, 8).iter_x().collect::<Vec<u64>>(), []);
        assert_eq!(RectU64::of(3, 6, 3, 8).iter_x().collect::<Vec<u64>>(), [3]);
        assert_eq!(RectU64::of(3, 6, 4, 8).iter_x().collect::<Vec<u64>>(), [3, 4]);
        assert_eq!(RectU64::of(3, 6, 5, 8).iter_x().collect::<Vec<u64>>(), [3, 4, 5]);
        assert_eq!(RectU64::of(3, 6, 6, 8).iter_x().collect::<Vec<u64>>(), [3, 4, 5, 6]);
        assert_eq!(RectU64::of(3, 6, 6, 8).iter_x().rev().collect::<Vec<u64>>(), [6, 5, 4, 3]);
        assert_eq!(RectU64::of(3, 6, 5, 8).iter_x().rev().collect::<Vec<u64>>(), [5, 4, 3]);
        assert_eq!(RectU64::of(3, 6, 4, 8).iter_x().rev().collect::<Vec<u64>>(), [4, 3]);
        assert_eq!(RectU64::of(3, 6, 3, 8).iter_x().rev().collect::<Vec<u64>>(), [3]);
        assert_eq!(RectU64::of(3, 6, 2, 8).iter_x().rev().collect::<Vec<u64>>(), []);
    }

    #[test]
    fn iter_y() {
        assert_eq!(RectU64::of(3, 6, 4, 5).iter_y().collect::<Vec<u64>>(), []);
        assert_eq!(RectU64::of(3, 6, 4, 6).iter_y().collect::<Vec<u64>>(), [6]);
        assert_eq!(RectU64::of(3, 6, 4, 7).iter_y().collect::<Vec<u64>>(), [6, 7]);
        assert_eq!(RectU64::of(3, 6, 4, 8).iter_y().collect::<Vec<u64>>(), [6, 7, 8]);
        assert_eq!(RectU64::of(3, 6, 4, 9).iter_y().collect::<Vec<u64>>(), [6, 7, 8, 9]);
        assert_eq!(RectU64::of(3, 6, 4, 9).iter_y().rev().collect::<Vec<u64>>(), [9, 8, 7, 6]);
        assert_eq!(RectU64::of(3, 6, 4, 8).iter_y().rev().collect::<Vec<u64>>(), [8, 7, 6]);
        assert_eq!(RectU64::of(3, 6, 4, 7).iter_y().rev().collect::<Vec<u64>>(), [7, 6]);
        assert_eq!(RectU64::of(3, 6, 4, 6).iter_y().rev().collect::<Vec<u64>>(), [6]);
        assert_eq!(RectU64::of(3, 6, 4, 5).iter_y().rev().collect::<Vec<u64>>(), []);
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectU64::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_x(&RectU64::of(0, 0, u64::MAX, 0)), u64::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectU64::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_y(&RectU64::of(0, 0, 0, u64::MAX)), u64::MAX);
    }

    #[test]
    fn test_max_delta() {
        assert_eq!(max_delta(&RectU64::of(0, 5, 10, 10)), 10);
        assert_eq!(max_delta(&RectU64::of(5, 0, 9, 9)), 9);
    }

    #[test]
    fn max_delta_0() {
        assert_eq!(max_delta(&RectU64::of(0, 0, 0, 0)), 0);
        assert_eq!(max_delta(&RectU64::of(1, 1, 1, 1)), 0);
        assert_eq!(max_delta(&RectU64::of(5, 10, 5, 10)), 0);
    }

    #[test]
    fn max_delta_1() {
        assert_eq!(max_delta(&RectU64::of(0, 0, 1, 1)), 1);
        assert_eq!(max_delta(&RectU64::of(5, 5, 6, 6)), 1);
        assert_eq!(max_delta(&RectU64::of(0, 0, 0, 1)), 1);
        assert_eq!(max_delta(&RectU64::of(5, 9, 5, 10)), 1);
    }

    #[test]
    fn max_delta_bounds() {
        assert_eq!(max_delta(&RectU64::of(0, 0, u64::MAX, u64::MAX - 1)), u64::MAX);
        assert_eq!(max_delta(&RectU64::of(0, 0, u64::MAX - 1, u64::MAX)), u64::MAX);
    }

    #[test]
    fn test_len_x() {
        assert_eq!(len_x(&RectU64::of(0, 0, 0, 0)), 1);
        assert_eq!(len_x(&RectU64::of(0, 0, u64::MAX - 1, 0)), u64::MAX);
    }

    #[test]
    fn test_len_y() {
        assert_eq!(len_y(&RectU64::of(0, 0, 0, 0)), 1);
        assert_eq!(len_y(&RectU64::of(0, 0, 0, u64::MAX - 1)), u64::MAX);
    }

    #[test]
    fn test_max_len() {
        assert_eq!(max_len(&RectU64::of(0, 5, 10, 10)), 11);
        assert_eq!(max_len(&RectU64::of(5, 0, 9, 9)), 10);
    }

    #[test]
    fn max_len_1() {
        assert_eq!(max_len(&RectU64::of(0, 0, 0, 0)), 1);
        assert_eq!(max_len(&RectU64::of(1, 1, 1, 1)), 1);
        assert_eq!(max_len(&RectU64::of(5, 10, 5, 10)), 1);
    }

    #[test]
    fn max_len_2() {
        assert_eq!(max_len(&RectU64::of(0, 0, 1, 1)), 2);
        assert_eq!(max_len(&RectU64::of(5, 5, 6, 6)), 2);
        assert_eq!(max_len(&RectU64::of(0, 0, 0, 1)), 2);
        assert_eq!(max_len(&RectU64::of(5, 9, 5, 10)), 2);
    }

    #[test]
    fn max_len_bounds() {
        assert_eq!(max_len(&RectU64::of(1, 0, u64::MAX - 1, u64::MAX - 1)), u64::MAX);
        assert_eq!(max_len(&RectU64::of(0, 1, u64::MAX - 1, u64::MAX - 1)), u64::MAX);
        assert_eq!(max_len(&RectU64::of(0, 0, u64::MAX - 2, u64::MAX - 1)), u64::MAX);
        assert_eq!(max_len(&RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 2)), u64::MAX);
    }
}
