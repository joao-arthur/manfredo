use crate::matrix::{
    point::point_u32,
    rect::{rect_u8::RectU8, rect_u16::RectU16},
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
pub struct RectU32 {
    pub min: point_u32::PointU32,
    pub max: point_u32::PointU32,
}

impl RectU32 {
    pub fn of(row1: u32, col1: u32, row2: u32, col2: u32) -> Self {
        RectU32 { min: point_u32::PointU32::of(row1, col1), max: point_u32::PointU32::of(row2, col2) }
    }

    pub fn largest() -> Self {
        RectU32 { min: point_u32::PointU32::min(), max: point_u32::PointU32::max() }
    }

    pub fn min() -> Self {
        RectU32 { min: point_u32::PointU32::min(), max: point_u32::PointU32::min() }
    }

    pub fn max() -> Self {
        RectU32 { min: point_u32::PointU32::max(), max: point_u32::PointU32::max() }
    }

    pub fn iter_row(&self) -> RangeInclusive<u32> {
        self.min.row..=self.max.row
    }

    pub fn iter_col(&self) -> RangeInclusive<u32> {
        self.min.col..=self.max.col
    }
}

impl From<RectU8> for RectU32 {
    fn from(r: RectU8) -> Self {
        RectU32 { min: point_u32::PointU32::of(r.min.row.into(), r.min.col.into()), max: point_u32::PointU32::of(r.max.row.into(), r.max.col.into()) }
    }
}

impl From<RectU16> for RectU32 {
    fn from(r: RectU16) -> Self {
        RectU32 { min: point_u32::PointU32::of(r.min.row.into(), r.min.col.into()), max: point_u32::PointU32::of(r.max.row.into(), r.max.col.into()) }
    }
}

impl std::fmt::Display for RectU32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_row(r: &RectU32) -> u32 {
    point_u32::delta_row(&r.min, &r.max)
}

pub fn delta_col(r: &RectU32) -> u32 {
    point_u32::delta_col(&r.min, &r.max)
}

pub fn max_delta(r: &RectU32) -> u32 {
    std::cmp::max(delta_row(r), delta_col(r))
}

pub fn len_row(r: &RectU32) -> u32 {
    delta_row(r) + 1
}

pub fn len_col(r: &RectU32) -> u32 {
    delta_col(r) + 1
}

pub fn max_len(r: &RectU32) -> u32 {
    std::cmp::max(len_row(r), len_col(r))
}

#[cfg(test)]
mod tests {
    use super::{RectU32, delta_col, delta_row, len_col, len_row, max_delta, max_len};
    use crate::matrix::{
        point::point_u32::PointU32,
        rect::{rect_u8::RectU8, rect_u16::RectU16},
    };

    #[test]
    fn rect_u32() {
        assert_eq!(RectU32::largest(), RectU32 { min: PointU32 { row: 0, col: 0 }, max: PointU32 { row: u32::MAX, col: u32::MAX } });
        assert_eq!(RectU32::min(), RectU32 { min: PointU32 { row: 0, col: 0 }, max: PointU32 { row: 0, col: 0 } });
        assert_eq!(RectU32::max(), RectU32 { min: PointU32 { row: u32::MAX, col: u32::MAX }, max: PointU32 { row: u32::MAX, col: u32::MAX } });
        assert_eq!(RectU32::of(256, 512, 1024, 2048), RectU32 { min: PointU32 { row: 256, col: 512 }, max: PointU32 { row: 1024, col: 2048 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(RectU32::of(256, 512, 1024, 2048).to_string(), "((256, 512), (1024, 2048))");
        assert_eq!(RectU32::of(u32::MAX, 0, 0, u32::MAX).to_string(), "((4294967295, 0), (0, 4294967295))");
    }

    #[test]
    fn from() {
        assert_eq!(RectU32::from(RectU8::largest()), RectU32 { min: PointU32 { row: 0, col: 0 }, max: PointU32 { row: u8::MAX.into(), col: u8::MAX.into() } });
        assert_eq!(RectU32::from(RectU16::largest()), RectU32 { min: PointU32 { row: 0, col: 0 }, max: PointU32 { row: u16::MAX.into(), col: u16::MAX.into() } });
    }

    #[test]
    fn iter_row() {
        assert_eq!(RectU32::of(3, 6, 2, 8).iter_row().collect::<Vec<u32>>(), []);
        assert_eq!(RectU32::of(3, 6, 3, 8).iter_row().collect::<Vec<u32>>(), [3]);
        assert_eq!(RectU32::of(3, 6, 4, 8).iter_row().collect::<Vec<u32>>(), [3, 4]);
        assert_eq!(RectU32::of(3, 6, 5, 8).iter_row().collect::<Vec<u32>>(), [3, 4, 5]);
        assert_eq!(RectU32::of(3, 6, 6, 8).iter_row().collect::<Vec<u32>>(), [3, 4, 5, 6]);
        assert_eq!(RectU32::of(3, 6, 6, 8).iter_row().rev().collect::<Vec<u32>>(), [6, 5, 4, 3]);
        assert_eq!(RectU32::of(3, 6, 5, 8).iter_row().rev().collect::<Vec<u32>>(), [5, 4, 3]);
        assert_eq!(RectU32::of(3, 6, 4, 8).iter_row().rev().collect::<Vec<u32>>(), [4, 3]);
        assert_eq!(RectU32::of(3, 6, 3, 8).iter_row().rev().collect::<Vec<u32>>(), [3]);
        assert_eq!(RectU32::of(3, 6, 2, 8).iter_row().rev().collect::<Vec<u32>>(), []);
    }

    #[test]
    fn iter_col() {
        assert_eq!(RectU32::of(3, 6, 4, 5).iter_col().collect::<Vec<u32>>(), []);
        assert_eq!(RectU32::of(3, 6, 4, 6).iter_col().collect::<Vec<u32>>(), [6]);
        assert_eq!(RectU32::of(3, 6, 4, 7).iter_col().collect::<Vec<u32>>(), [6, 7]);
        assert_eq!(RectU32::of(3, 6, 4, 8).iter_col().collect::<Vec<u32>>(), [6, 7, 8]);
        assert_eq!(RectU32::of(3, 6, 4, 9).iter_col().collect::<Vec<u32>>(), [6, 7, 8, 9]);
        assert_eq!(RectU32::of(3, 6, 4, 9).iter_col().rev().collect::<Vec<u32>>(), [9, 8, 7, 6]);
        assert_eq!(RectU32::of(3, 6, 4, 8).iter_col().rev().collect::<Vec<u32>>(), [8, 7, 6]);
        assert_eq!(RectU32::of(3, 6, 4, 7).iter_col().rev().collect::<Vec<u32>>(), [7, 6]);
        assert_eq!(RectU32::of(3, 6, 4, 6).iter_col().rev().collect::<Vec<u32>>(), [6]);
        assert_eq!(RectU32::of(3, 6, 4, 5).iter_col().rev().collect::<Vec<u32>>(), []);
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&RectU32::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_row(&RectU32::of(0, 0, u32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&RectU32::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_col(&RectU32::of(0, 0, 0, u32::MAX)), u32::MAX);
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
    fn max_delta_bounds() {
        assert_eq!(max_delta(&RectU32::of(0, 0, u32::MAX, u32::MAX - 1)), u32::MAX);
        assert_eq!(max_delta(&RectU32::of(0, 0, u32::MAX - 1, u32::MAX)), u32::MAX);
    }

    #[test]
    fn test_len_row() {
        assert_eq!(len_row(&RectU32::of(0, 0, 0, 0)), 1);
        assert_eq!(len_row(&RectU32::of(0, 0, u32::MAX - 1, 0)), u32::MAX);
    }

    #[test]
    fn test_len_col() {
        assert_eq!(len_col(&RectU32::of(0, 0, 0, 0)), 1);
        assert_eq!(len_col(&RectU32::of(0, 0, 0, u32::MAX - 1)), u32::MAX);
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
    fn max_len_bounds() {
        assert_eq!(max_len(&RectU32::of(1, 0, u32::MAX - 1, u32::MAX - 1)), u32::MAX);
        assert_eq!(max_len(&RectU32::of(0, 1, u32::MAX - 1, u32::MAX - 1)), u32::MAX);
        assert_eq!(max_len(&RectU32::of(0, 0, u32::MAX - 2, u32::MAX - 1)), u32::MAX);
        assert_eq!(max_len(&RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 2)), u32::MAX);
    }
}
