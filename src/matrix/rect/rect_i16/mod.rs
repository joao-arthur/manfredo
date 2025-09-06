use crate::matrix::{point::point_i16, rect::rect_i8::RectI8};
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
    pub fn of(row1: i16, col1: i16, row2: i16, col2: i16) -> Self {
        RectI16 { min: point_i16::PointI16::of(row1, col1), max: point_i16::PointI16::of(row2, col2) }
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

    pub fn iter_row(&self) -> RangeInclusive<i16> {
        self.min.row..=self.max.row
    }

    pub fn iter_col(&self) -> RangeInclusive<i16> {
        self.min.col..=self.max.col
    }
}

impl From<RectI8> for RectI16 {
    fn from(r: RectI8) -> Self {
        RectI16 { min: point_i16::PointI16::of(r.min.row.into(), r.min.col.into()), max: point_i16::PointI16::of(r.max.row.into(), r.max.col.into()) }
    }
}

impl std::fmt::Display for RectI16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_row(r: &RectI16) -> u16 {
    point_i16::delta_row(&r.min, &r.max)
}

pub fn delta_col(r: &RectI16) -> u16 {
    point_i16::delta_col(&r.min, &r.max)
}

pub fn max_delta(r: &RectI16) -> u16 {
    std::cmp::max(delta_row(r), delta_col(r))
}

pub fn len_row(r: &RectI16) -> u16 {
    delta_row(r) + 1
}

pub fn len_col(r: &RectI16) -> u16 {
    delta_col(r) + 1
}

pub fn max_len(r: &RectI16) -> u16 {
    std::cmp::max(len_row(r), len_col(r))
}

#[cfg(test)]
mod tests {
    use super::{RectI16, delta_col, delta_row, len_col, len_row, max_delta, max_len};
    use crate::matrix::{point::point_i16::PointI16, rect::rect_i8::RectI8};

    #[test]
    fn rect_i16() {
        assert_eq!(RectI16::largest(), RectI16 { min: PointI16 { row: i16::MIN, col: i16::MIN }, max: PointI16 { row: i16::MAX, col: i16::MAX } });
        assert_eq!(RectI16::min(), RectI16 { min: PointI16 { row: i16::MIN, col: i16::MIN }, max: PointI16 { row: i16::MIN, col: i16::MIN } });
        assert_eq!(RectI16::max(), RectI16 { min: PointI16 { row: i16::MAX, col: i16::MAX }, max: PointI16 { row: i16::MAX, col: i16::MAX } });
        assert_eq!(RectI16::of(i16::MIN, -1, 1, i16::MAX), RectI16 { min: PointI16 { row: i16::MIN, col: -1 }, max: PointI16 { row: 1, col: i16::MAX } });
    }

    #[test]
    fn to_string() {
        assert_eq!(RectI16::largest().to_string(), "((-32768, -32768), (32767, 32767))");
        assert_eq!(RectI16::of(i16::MIN, -0, 0, i16::MAX).to_string(), "((-32768, 0), (0, 32767))");
    }

    #[test]
    fn from() {
        assert_eq!(RectI16::from(RectI8::largest()), RectI16 { min: PointI16 { row: i8::MIN.into(), col: i8::MIN.into() }, max: PointI16 { row: i8::MAX.into(), col: i8::MAX.into() } });
    }

    #[test]
    fn iter_row() {
        assert_eq!(RectI16::of(-6, -8, -7, -6).iter_row().collect::<Vec<i16>>(), []);
        assert_eq!(RectI16::of(-6, -8, -6, -6).iter_row().collect::<Vec<i16>>(), [-6]);
        assert_eq!(RectI16::of(-6, -8, -5, -6).iter_row().collect::<Vec<i16>>(), [-6, -5]);
        assert_eq!(RectI16::of(-6, -8, -4, -6).iter_row().collect::<Vec<i16>>(), [-6, -5, -4]);
        assert_eq!(RectI16::of(-6, -8, -3, -6).iter_row().collect::<Vec<i16>>(), [-6, -5, -4, -3]);
        assert_eq!(RectI16::of(-6, -8, -3, -6).iter_row().rev().collect::<Vec<i16>>(), [-3, -4, -5, -6]);
        assert_eq!(RectI16::of(-6, -8, -4, -6).iter_row().rev().collect::<Vec<i16>>(), [-4, -5, -6]);
        assert_eq!(RectI16::of(-6, -8, -5, -6).iter_row().rev().collect::<Vec<i16>>(), [-5, -6]);
        assert_eq!(RectI16::of(-6, -8, -6, -6).iter_row().rev().collect::<Vec<i16>>(), [-6]);
        assert_eq!(RectI16::of(-6, -8, -7, -6).iter_row().rev().collect::<Vec<i16>>(), []);
    }

    #[test]
    fn iter_col() {
        assert_eq!(RectI16::of(-6, -8, -4, -9).iter_col().collect::<Vec<i16>>(), []);
        assert_eq!(RectI16::of(-6, -8, -4, -8).iter_col().collect::<Vec<i16>>(), [-8]);
        assert_eq!(RectI16::of(-6, -8, -4, -7).iter_col().collect::<Vec<i16>>(), [-8, -7]);
        assert_eq!(RectI16::of(-6, -8, -4, -6).iter_col().collect::<Vec<i16>>(), [-8, -7, -6]);
        assert_eq!(RectI16::of(-6, -8, -4, -5).iter_col().collect::<Vec<i16>>(), [-8, -7, -6, -5]);
        assert_eq!(RectI16::of(-6, -8, -4, -5).iter_col().rev().collect::<Vec<i16>>(), [-5, -6, -7, -8]);
        assert_eq!(RectI16::of(-6, -8, -4, -6).iter_col().rev().collect::<Vec<i16>>(), [-6, -7, -8]);
        assert_eq!(RectI16::of(-6, -8, -4, -7).iter_col().rev().collect::<Vec<i16>>(), [-7, -8]);
        assert_eq!(RectI16::of(-6, -8, -4, -8).iter_col().rev().collect::<Vec<i16>>(), [-8]);
        assert_eq!(RectI16::of(-6, -8, -4, -9).iter_col().rev().collect::<Vec<i16>>(), []);
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&RectI16::of(0, i16::MIN, 0, i16::MAX)), 0);
        assert_eq!(delta_row(&RectI16::of(i16::MIN, 0, i16::MAX, 0)), u16::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&RectI16::of(i16::MIN, 0, i16::MAX, 0)), 0);
        assert_eq!(delta_col(&RectI16::of(0, i16::MIN, 0, i16::MAX)), u16::MAX);
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
    fn test_len_row() {
        assert_eq!(len_row(&RectI16::of(0, i16::MIN, 0, i16::MAX)), 1);
        assert_eq!(len_row(&RectI16::of(i16::MIN, 0, i16::MAX - 1, 0)), u16::MAX);
    }

    #[test]
    fn test_len_col() {
        assert_eq!(len_col(&RectI16::of(i16::MIN, 0, i16::MAX, 0)), 1);
        assert_eq!(len_col(&RectI16::of(0, i16::MIN, 0, i16::MAX - 1)), u16::MAX);
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
