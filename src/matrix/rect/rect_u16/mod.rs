use crate::matrix::{point::point_u16, rect::rect_u8::RectU8};
use std::ops::RangeInclusive;

pub mod add;
pub mod contains_point;
pub mod contains_rect;
pub mod deflate;
pub mod inflate;
pub mod resize;
pub mod translate;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct RectU16 {
    pub min: point_u16::PointU16,
    pub max: point_u16::PointU16,
}

impl RectU16 {
    pub fn of(row1: u16, col1: u16, row2: u16, col2: u16) -> Self {
        RectU16 { min: point_u16::PointU16::of(row1, col1), max: point_u16::PointU16::of(row2, col2) }
    }

    pub fn largest() -> Self {
        RectU16 { min: point_u16::PointU16::min(), max: point_u16::PointU16::max() }
    }

    pub fn min() -> Self {
        RectU16 { min: point_u16::PointU16::min(), max: point_u16::PointU16::min() }
    }

    pub fn max() -> Self {
        RectU16 { min: point_u16::PointU16::max(), max: point_u16::PointU16::max() }
    }

    pub fn iter_row(&self) -> RangeInclusive<u16> {
        self.min.row..=self.max.row
    }

    pub fn iter_col(&self) -> RangeInclusive<u16> {
        self.min.col..=self.max.col
    }
}

impl From<RectU8> for RectU16 {
    fn from(r: RectU8) -> Self {
        RectU16 { min: point_u16::PointU16::of(r.min.row.into(), r.min.col.into()), max: point_u16::PointU16::of(r.max.row.into(), r.max.col.into()) }
    }
}

impl std::fmt::Display for RectU16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_row(r: &RectU16) -> u16 {
    point_u16::delta_row(&r.min, &r.max)
}

pub fn delta_col(r: &RectU16) -> u16 {
    point_u16::delta_col(&r.min, &r.max)
}

pub fn max_delta(r: &RectU16) -> u16 {
    std::cmp::max(delta_row(r), delta_col(r))
}

pub fn len_row(r: &RectU16) -> u16 {
    delta_row(r) + 1
}

pub fn len_col(r: &RectU16) -> u16 {
    delta_col(r) + 1
}

pub fn max_len(r: &RectU16) -> u16 {
    std::cmp::max(len_row(r), len_col(r))
}

#[cfg(test)]
mod tests {
    use super::{RectU16, delta_col, delta_row, len_col, len_row, max_delta, max_len};
    use crate::matrix::{point::point_u16::PointU16, rect::rect_u8::RectU8};

    #[test]
    fn rect_u16() {
        assert_eq!(RectU16::largest(), RectU16 { min: PointU16 { row: 0, col: 0 }, max: PointU16 { row: u16::MAX, col: u16::MAX } });
        assert_eq!(RectU16::min(), RectU16 { min: PointU16 { row: 0, col: 0 }, max: PointU16 { row: 0, col: 0 } });
        assert_eq!(RectU16::max(), RectU16 { min: PointU16 { row: u16::MAX, col: u16::MAX }, max: PointU16 { row: u16::MAX, col: u16::MAX } });
        assert_eq!(RectU16::of(16, 32, 64, 128), RectU16 { min: PointU16 { row: 16, col: 32 }, max: PointU16 { row: 64, col: 128 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(RectU16::of(16, 32, 64, 128).to_string(), "((16, 32), (64, 128))");
        assert_eq!(RectU16::of(u16::MAX, 0, 0, u16::MAX).to_string(), "((65535, 0), (0, 65535))");
    }

    #[test]
    fn from() {
        assert_eq!(
            RectU16::from(RectU8::largest()),
            RectU16 { min: PointU16 { row: 0, col: 0 }, max: PointU16 { row: u8::MAX.into(), col: u8::MAX.into() } }
        );
    }

    #[test]
    fn iter_row() {
        assert_eq!(RectU16::of(3, 6, 2, 8).iter_row().collect::<Vec<u16>>(), []);
        assert_eq!(RectU16::of(3, 6, 3, 8).iter_row().collect::<Vec<u16>>(), [3]);
        assert_eq!(RectU16::of(3, 6, 4, 8).iter_row().collect::<Vec<u16>>(), [3, 4]);
        assert_eq!(RectU16::of(3, 6, 5, 8).iter_row().collect::<Vec<u16>>(), [3, 4, 5]);
        assert_eq!(RectU16::of(3, 6, 6, 8).iter_row().collect::<Vec<u16>>(), [3, 4, 5, 6]);
        assert_eq!(RectU16::of(3, 6, 6, 8).iter_row().rev().collect::<Vec<u16>>(), [6, 5, 4, 3]);
        assert_eq!(RectU16::of(3, 6, 5, 8).iter_row().rev().collect::<Vec<u16>>(), [5, 4, 3]);
        assert_eq!(RectU16::of(3, 6, 4, 8).iter_row().rev().collect::<Vec<u16>>(), [4, 3]);
        assert_eq!(RectU16::of(3, 6, 3, 8).iter_row().rev().collect::<Vec<u16>>(), [3]);
        assert_eq!(RectU16::of(3, 6, 2, 8).iter_row().rev().collect::<Vec<u16>>(), []);
    }

    #[test]
    fn iter_col() {
        assert_eq!(RectU16::of(3, 6, 4, 5).iter_col().collect::<Vec<u16>>(), []);
        assert_eq!(RectU16::of(3, 6, 4, 6).iter_col().collect::<Vec<u16>>(), [6]);
        assert_eq!(RectU16::of(3, 6, 4, 7).iter_col().collect::<Vec<u16>>(), [6, 7]);
        assert_eq!(RectU16::of(3, 6, 4, 8).iter_col().collect::<Vec<u16>>(), [6, 7, 8]);
        assert_eq!(RectU16::of(3, 6, 4, 9).iter_col().collect::<Vec<u16>>(), [6, 7, 8, 9]);
        assert_eq!(RectU16::of(3, 6, 4, 9).iter_col().rev().collect::<Vec<u16>>(), [9, 8, 7, 6]);
        assert_eq!(RectU16::of(3, 6, 4, 8).iter_col().rev().collect::<Vec<u16>>(), [8, 7, 6]);
        assert_eq!(RectU16::of(3, 6, 4, 7).iter_col().rev().collect::<Vec<u16>>(), [7, 6]);
        assert_eq!(RectU16::of(3, 6, 4, 6).iter_col().rev().collect::<Vec<u16>>(), [6]);
        assert_eq!(RectU16::of(3, 6, 4, 5).iter_col().rev().collect::<Vec<u16>>(), []);
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&RectU16::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_row(&RectU16::of(0, 0, u16::MAX, 0)), u16::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&RectU16::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_col(&RectU16::of(0, 0, 0, u16::MAX)), u16::MAX);
    }

    #[test]
    fn test_max_delta() {
        assert_eq!(max_delta(&RectU16::of(0, 5, 10, 10)), 10);
        assert_eq!(max_delta(&RectU16::of(5, 0, 9, 9)), 9);
    }

    #[test]
    fn max_delta_0() {
        assert_eq!(max_delta(&RectU16::of(0, 0, 0, 0)), 0);
        assert_eq!(max_delta(&RectU16::of(1, 1, 1, 1)), 0);
        assert_eq!(max_delta(&RectU16::of(5, 10, 5, 10)), 0);
    }

    #[test]
    fn max_delta_1() {
        assert_eq!(max_delta(&RectU16::of(0, 0, 1, 1)), 1);
        assert_eq!(max_delta(&RectU16::of(5, 5, 6, 6)), 1);
        assert_eq!(max_delta(&RectU16::of(0, 0, 0, 1)), 1);
        assert_eq!(max_delta(&RectU16::of(5, 9, 5, 10)), 1);
    }

    #[test]
    fn max_delta_bounds() {
        assert_eq!(max_delta(&RectU16::of(0, 0, u16::MAX, u16::MAX - 1)), u16::MAX);
        assert_eq!(max_delta(&RectU16::of(0, 0, u16::MAX - 1, u16::MAX)), u16::MAX);
    }

    #[test]
    fn test_len_row() {
        assert_eq!(len_row(&RectU16::of(0, 0, 0, 0)), 1);
        assert_eq!(len_row(&RectU16::of(0, 0, u16::MAX - 1, 0)), u16::MAX);
    }

    #[test]
    fn test_len_col() {
        assert_eq!(len_col(&RectU16::of(0, 0, 0, 0)), 1);
        assert_eq!(len_col(&RectU16::of(0, 0, 0, u16::MAX - 1)), u16::MAX);
    }

    #[test]
    fn test_max_len() {
        assert_eq!(max_len(&RectU16::of(0, 5, 10, 10)), 11);
        assert_eq!(max_len(&RectU16::of(5, 0, 9, 9)), 10);
    }

    #[test]
    fn max_len_1() {
        assert_eq!(max_len(&RectU16::of(0, 0, 0, 0)), 1);
        assert_eq!(max_len(&RectU16::of(1, 1, 1, 1)), 1);
        assert_eq!(max_len(&RectU16::of(5, 10, 5, 10)), 1);
    }

    #[test]
    fn max_len_2() {
        assert_eq!(max_len(&RectU16::of(0, 0, 1, 1)), 2);
        assert_eq!(max_len(&RectU16::of(5, 5, 6, 6)), 2);
        assert_eq!(max_len(&RectU16::of(0, 0, 0, 1)), 2);
        assert_eq!(max_len(&RectU16::of(5, 9, 5, 10)), 2);
    }

    #[test]
    fn max_len_bounds() {
        assert_eq!(max_len(&RectU16::of(1, 0, u16::MAX - 1, u16::MAX - 1)), u16::MAX);
        assert_eq!(max_len(&RectU16::of(0, 1, u16::MAX - 1, u16::MAX - 1)), u16::MAX);
        assert_eq!(max_len(&RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 1)), u16::MAX);
        assert_eq!(max_len(&RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 2)), u16::MAX);
    }
}
