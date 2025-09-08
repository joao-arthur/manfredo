use crate::matrix::{
    point::point_i64,
    rect::{rect_i8::RectI8, rect_i16::RectI16, rect_i32::RectI32},
};
use std::ops::RangeInclusive;

mod add;
mod contains_point;
mod contains_rect;
mod deflate;
mod inflate;
mod resize;
mod translate;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::contains_point::contains_point;
pub use self::contains_rect::contains_rect;
pub use self::deflate::{assign_deflate, deflate, try_assign_deflate, try_deflate};
pub use self::inflate::{
    checked_inflate, checked_inflate_assign, saturating_inflate, saturating_inflate_assign, try_checked_inflate, try_checked_inflate_assign, try_saturating_inflate, try_saturating_inflate_assign,
    wrapping_inflate, wrapping_inflate_assign,
};
pub use self::resize::{
    checked_resize, checked_resize_assign, saturating_resize, saturating_resize_assign, try_checked_resize, try_checked_resize_assign, try_saturating_resize, try_saturating_resize_assign,
    try_wrapping_resize, try_wrapping_resize_assign, wrapping_resize, wrapping_resize_assign,
};
pub use self::translate::{
    checked_translate, checked_translate_assign, saturating_translate, saturating_translate_assign, try_checked_translate, try_checked_translate_assign, wrapping_translate, wrapping_translate_assign,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct RectI64 {
    pub min: point_i64::PointI64,
    pub max: point_i64::PointI64,
}

impl RectI64 {
    pub fn of(row1: i64, col1: i64, row2: i64, col2: i64) -> Self {
        RectI64 { min: point_i64::PointI64::of(row1, col1), max: point_i64::PointI64::of(row2, col2) }
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

    pub fn iter_row(&self) -> RangeInclusive<i64> {
        self.min.row..=self.max.row
    }

    pub fn iter_col(&self) -> RangeInclusive<i64> {
        self.min.col..=self.max.col
    }
}

impl From<RectI8> for RectI64 {
    fn from(r: RectI8) -> Self {
        RectI64 { min: point_i64::PointI64::of(r.min.row.into(), r.min.col.into()), max: point_i64::PointI64::of(r.max.row.into(), r.max.col.into()) }
    }
}

impl From<RectI16> for RectI64 {
    fn from(r: RectI16) -> Self {
        RectI64 { min: point_i64::PointI64::of(r.min.row.into(), r.min.col.into()), max: point_i64::PointI64::of(r.max.row.into(), r.max.col.into()) }
    }
}

impl From<RectI32> for RectI64 {
    fn from(r: RectI32) -> Self {
        RectI64 { min: point_i64::PointI64::of(r.min.row.into(), r.min.col.into()), max: point_i64::PointI64::of(r.max.row.into(), r.max.col.into()) }
    }
}

impl std::fmt::Display for RectI64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_row(r: &RectI64) -> u64 {
    point_i64::delta_row(&r.min, &r.max)
}

pub fn delta_col(r: &RectI64) -> u64 {
    point_i64::delta_col(&r.min, &r.max)
}

pub fn max_delta(r: &RectI64) -> u64 {
    std::cmp::max(delta_row(r), delta_col(r))
}

pub fn len_row(r: &RectI64) -> u64 {
    delta_row(r) + 1
}

pub fn len_col(r: &RectI64) -> u64 {
    delta_col(r) + 1
}

pub fn max_len(r: &RectI64) -> u64 {
    std::cmp::max(len_row(r), len_col(r))
}

#[cfg(test)]
mod tests {
    use super::{RectI64, delta_col, delta_row, len_col, len_row, max_delta, max_len};
    use crate::matrix::{
        point::point_i64::PointI64,
        rect::{rect_i8::RectI8, rect_i16::RectI16, rect_i32::RectI32},
    };

    #[test]
    fn rect_i64() {
        assert_eq!(RectI64::largest(), RectI64 { min: PointI64 { row: i64::MIN, col: i64::MIN }, max: PointI64 { row: i64::MAX, col: i64::MAX } });
        assert_eq!(RectI64::min(), RectI64 { min: PointI64 { row: i64::MIN, col: i64::MIN }, max: PointI64 { row: i64::MIN, col: i64::MIN } });
        assert_eq!(RectI64::max(), RectI64 { min: PointI64 { row: i64::MAX, col: i64::MAX }, max: PointI64 { row: i64::MAX, col: i64::MAX } });
        assert_eq!(RectI64::of(i64::MIN, -1, 1, i64::MAX), RectI64 { min: PointI64 { row: i64::MIN, col: -1 }, max: PointI64 { row: 1, col: i64::MAX } });
    }

    #[test]
    fn to_string() {
        assert_eq!(RectI64::largest().to_string(), "((-9223372036854775808, -9223372036854775808), (9223372036854775807, 9223372036854775807))");
        assert_eq!(RectI64::of(i64::MIN, -0, 0, i64::MAX).to_string(), "((-9223372036854775808, 0), (0, 9223372036854775807))");
    }

    #[test]
    fn from() {
        assert_eq!(RectI64::from(RectI8::largest()), RectI64 { min: PointI64 { row: i8::MIN.into(), col: i8::MIN.into() }, max: PointI64 { row: i8::MAX.into(), col: i8::MAX.into() } });
        assert_eq!(RectI64::from(RectI16::largest()), RectI64 { min: PointI64 { row: i16::MIN.into(), col: i16::MIN.into() }, max: PointI64 { row: i16::MAX.into(), col: i16::MAX.into() } });
        assert_eq!(RectI64::from(RectI32::largest()), RectI64 { min: PointI64 { row: i32::MIN.into(), col: i32::MIN.into() }, max: PointI64 { row: i32::MAX.into(), col: i32::MAX.into() } });
    }

    #[test]
    fn iter_row() {
        assert_eq!(RectI64::of(-6, -8, -7, -6).iter_row().collect::<Vec<i64>>(), []);
        assert_eq!(RectI64::of(-6, -8, -6, -6).iter_row().collect::<Vec<i64>>(), [-6]);
        assert_eq!(RectI64::of(-6, -8, -5, -6).iter_row().collect::<Vec<i64>>(), [-6, -5]);
        assert_eq!(RectI64::of(-6, -8, -4, -6).iter_row().collect::<Vec<i64>>(), [-6, -5, -4]);
        assert_eq!(RectI64::of(-6, -8, -3, -6).iter_row().collect::<Vec<i64>>(), [-6, -5, -4, -3]);
        assert_eq!(RectI64::of(-6, -8, -3, -6).iter_row().rev().collect::<Vec<i64>>(), [-3, -4, -5, -6]);
        assert_eq!(RectI64::of(-6, -8, -4, -6).iter_row().rev().collect::<Vec<i64>>(), [-4, -5, -6]);
        assert_eq!(RectI64::of(-6, -8, -5, -6).iter_row().rev().collect::<Vec<i64>>(), [-5, -6]);
        assert_eq!(RectI64::of(-6, -8, -6, -6).iter_row().rev().collect::<Vec<i64>>(), [-6]);
        assert_eq!(RectI64::of(-6, -8, -7, -6).iter_row().rev().collect::<Vec<i64>>(), []);
    }

    #[test]
    fn iter_col() {
        assert_eq!(RectI64::of(-6, -8, -4, -9).iter_col().collect::<Vec<i64>>(), []);
        assert_eq!(RectI64::of(-6, -8, -4, -8).iter_col().collect::<Vec<i64>>(), [-8]);
        assert_eq!(RectI64::of(-6, -8, -4, -7).iter_col().collect::<Vec<i64>>(), [-8, -7]);
        assert_eq!(RectI64::of(-6, -8, -4, -6).iter_col().collect::<Vec<i64>>(), [-8, -7, -6]);
        assert_eq!(RectI64::of(-6, -8, -4, -5).iter_col().collect::<Vec<i64>>(), [-8, -7, -6, -5]);
        assert_eq!(RectI64::of(-6, -8, -4, -5).iter_col().rev().collect::<Vec<i64>>(), [-5, -6, -7, -8]);
        assert_eq!(RectI64::of(-6, -8, -4, -6).iter_col().rev().collect::<Vec<i64>>(), [-6, -7, -8]);
        assert_eq!(RectI64::of(-6, -8, -4, -7).iter_col().rev().collect::<Vec<i64>>(), [-7, -8]);
        assert_eq!(RectI64::of(-6, -8, -4, -8).iter_col().rev().collect::<Vec<i64>>(), [-8]);
        assert_eq!(RectI64::of(-6, -8, -4, -9).iter_col().rev().collect::<Vec<i64>>(), []);
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&RectI64::of(0, i64::MIN, 0, i64::MAX)), 0);
        assert_eq!(delta_row(&RectI64::of(i64::MIN, 0, i64::MAX, 0)), u64::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&RectI64::of(i64::MIN, 0, i64::MAX, 0)), 0);
        assert_eq!(delta_col(&RectI64::of(0, i64::MIN, 0, i64::MAX)), u64::MAX);
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
    fn test_len_row() {
        assert_eq!(len_row(&RectI64::of(0, i64::MIN, 0, i64::MAX)), 1);
        assert_eq!(len_row(&RectI64::of(i64::MIN, 0, i64::MAX - 1, 0)), u64::MAX);
    }

    #[test]
    fn test_len_col() {
        assert_eq!(len_col(&RectI64::of(i64::MIN, 0, i64::MAX, 0)), 1);
        assert_eq!(len_col(&RectI64::of(0, i64::MIN, 0, i64::MAX - 1)), u64::MAX);
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
