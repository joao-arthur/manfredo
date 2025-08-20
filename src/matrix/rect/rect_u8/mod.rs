use crate::matrix::point::point_u8;
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
    pub fn of(row1: u8, col1: u8, row2: u8, col2: u8) -> Self {
        RectU8 { min: point_u8::PointU8::of(row1, col1), max: point_u8::PointU8::of(row2, col2) }
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

    pub fn iter_row(&self) -> RangeInclusive<u8> {
        self.min.row..=self.max.row
    }

    pub fn iter_col(&self) -> RangeInclusive<u8> {
        self.min.col..=self.max.col
    }
}

impl std::fmt::Display for RectU8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_row(r: &RectU8) -> u8 {
    point_u8::delta_row(&r.min, &r.max)
}

pub fn delta_col(r: &RectU8) -> u8 {
    point_u8::delta_col(&r.min, &r.max)
}

pub fn max_delta(r: &RectU8) -> u8 {
    std::cmp::max(delta_row(r), delta_col(r))
}

pub fn len_row(r: &RectU8) -> u8 {
    delta_row(r) + 1
}

pub fn len_col(r: &RectU8) -> u8 {
    delta_col(r) + 1
}

pub fn max_len(r: &RectU8) -> u8 {
    std::cmp::max(len_row(r), len_col(r))
}

#[cfg(test)]
mod tests {
    use crate::matrix::point::point_u8::PointU8;

    use super::{RectU8, delta_col, delta_row, len_col, len_row, max_delta, max_len};

    #[test]
    fn rect_u8() {
        assert_eq!(RectU8::largest(), RectU8 { min: PointU8 { row: 0, col: 0 }, max: PointU8 { row: u8::MAX, col: u8::MAX } });
        assert_eq!(RectU8::min(), RectU8 { min: PointU8 { row: 0, col: 0 }, max: PointU8 { row: 0, col: 0 } });
        assert_eq!(RectU8::max(), RectU8 { min: PointU8 { row: u8::MAX, col: u8::MAX }, max: PointU8 { row: u8::MAX, col: u8::MAX } });
        assert_eq!(RectU8::of(0, 2, 4, 8), RectU8 { min: PointU8 { row: 0, col: 2 }, max: PointU8 { row: 4, col: 8 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(RectU8::of(0, 2, 4, 8).to_string(), "((0, 2), (4, 8))");
        assert_eq!(RectU8::of(u8::MAX, 0, 0, u8::MAX).to_string(), "((255, 0), (0, 255))");
    }

    #[test]
    fn iter_row() {
        assert_eq!(RectU8::of(3, 6, 2, 8).iter_row().collect::<Vec<u8>>(), []);
        assert_eq!(RectU8::of(3, 6, 3, 8).iter_row().collect::<Vec<u8>>(), [3]);
        assert_eq!(RectU8::of(3, 6, 4, 8).iter_row().collect::<Vec<u8>>(), [3, 4]);
        assert_eq!(RectU8::of(3, 6, 5, 8).iter_row().collect::<Vec<u8>>(), [3, 4, 5]);
        assert_eq!(RectU8::of(3, 6, 6, 8).iter_row().collect::<Vec<u8>>(), [3, 4, 5, 6]);
        assert_eq!(RectU8::of(3, 6, 6, 8).iter_row().rev().collect::<Vec<u8>>(), [6, 5, 4, 3]);
        assert_eq!(RectU8::of(3, 6, 5, 8).iter_row().rev().collect::<Vec<u8>>(), [5, 4, 3]);
        assert_eq!(RectU8::of(3, 6, 4, 8).iter_row().rev().collect::<Vec<u8>>(), [4, 3]);
        assert_eq!(RectU8::of(3, 6, 3, 8).iter_row().rev().collect::<Vec<u8>>(), [3]);
        assert_eq!(RectU8::of(3, 6, 2, 8).iter_row().rev().collect::<Vec<u8>>(), []);
    }

    #[test]
    fn iter_col() {
        assert_eq!(RectU8::of(3, 6, 4, 5).iter_col().collect::<Vec<u8>>(), []);
        assert_eq!(RectU8::of(3, 6, 4, 6).iter_col().collect::<Vec<u8>>(), [6]);
        assert_eq!(RectU8::of(3, 6, 4, 7).iter_col().collect::<Vec<u8>>(), [6, 7]);
        assert_eq!(RectU8::of(3, 6, 4, 8).iter_col().collect::<Vec<u8>>(), [6, 7, 8]);
        assert_eq!(RectU8::of(3, 6, 4, 9).iter_col().collect::<Vec<u8>>(), [6, 7, 8, 9]);
        assert_eq!(RectU8::of(3, 6, 4, 9).iter_col().rev().collect::<Vec<u8>>(), [9, 8, 7, 6]);
        assert_eq!(RectU8::of(3, 6, 4, 8).iter_col().rev().collect::<Vec<u8>>(), [8, 7, 6]);
        assert_eq!(RectU8::of(3, 6, 4, 7).iter_col().rev().collect::<Vec<u8>>(), [7, 6]);
        assert_eq!(RectU8::of(3, 6, 4, 6).iter_col().rev().collect::<Vec<u8>>(), [6]);
        assert_eq!(RectU8::of(3, 6, 4, 5).iter_col().rev().collect::<Vec<u8>>(), []);
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&RectU8::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_row(&RectU8::of(0, 0, u8::MAX, 0)), u8::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&RectU8::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_col(&RectU8::of(0, 0, 0, u8::MAX)), u8::MAX);
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
    fn test_len_row() {
        assert_eq!(len_row(&RectU8::of(0, 0, 0, 0)), 1);
        assert_eq!(len_row(&RectU8::of(0, 0, u8::MAX - 1, 0)), u8::MAX);
    }

    #[test]
    fn test_len_col() {
        assert_eq!(len_col(&RectU8::of(0, 0, 0, 0)), 1);
        assert_eq!(len_col(&RectU8::of(0, 0, 0, u8::MAX - 1)), u8::MAX);
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
