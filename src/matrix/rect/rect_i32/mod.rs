use crate::matrix::{
    point::point_i32,
    rect::{rect_i8::RectI8, rect_i16::RectI16},
};
use std::ops::RangeInclusive;

mod add;
mod contains_point;
mod contains_rect;
mod deflate;
mod inflate;
mod resize;
mod translate;

pub use add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use contains_point::contains_point;
pub use contains_rect::contains_rect;
pub use deflate::{assign_deflate, deflate, try_assign_deflate, try_deflate};
pub use inflate::{
    checked_inflate, checked_inflate_assign, saturating_inflate, saturating_inflate_assign, try_checked_inflate, try_checked_inflate_assign, try_saturating_inflate, try_saturating_inflate_assign,
    wrapping_inflate, wrapping_inflate_assign,
};
pub use resize::{
    checked_resize, checked_resize_assign, saturating_resize, saturating_resize_assign, try_checked_resize, try_checked_resize_assign, try_saturating_resize, try_saturating_resize_assign,
    try_wrapping_resize, try_wrapping_resize_assign, wrapping_resize, wrapping_resize_assign,
};
pub use translate::{
    checked_translate, checked_translate_assign, saturating_translate, saturating_translate_assign, try_checked_translate, try_checked_translate_assign, wrapping_translate, wrapping_translate_assign,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct RectI32 {
    pub min: point_i32::PointI32,
    pub max: point_i32::PointI32,
}

impl RectI32 {
    pub fn of(row1: i32, col1: i32, row2: i32, col2: i32) -> Self {
        RectI32 { min: point_i32::PointI32::of(row1, col1), max: point_i32::PointI32::of(row2, col2) }
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

    pub fn iter_row(&self) -> RangeInclusive<i32> {
        self.min.row..=self.max.row
    }

    pub fn iter_col(&self) -> RangeInclusive<i32> {
        self.min.col..=self.max.col
    }
}

impl From<RectI8> for RectI32 {
    fn from(r: RectI8) -> Self {
        RectI32 { min: point_i32::PointI32::of(r.min.row.into(), r.min.col.into()), max: point_i32::PointI32::of(r.max.row.into(), r.max.col.into()) }
    }
}

impl From<RectI16> for RectI32 {
    fn from(r: RectI16) -> Self {
        RectI32 { min: point_i32::PointI32::of(r.min.row.into(), r.min.col.into()), max: point_i32::PointI32::of(r.max.row.into(), r.max.col.into()) }
    }
}

impl std::fmt::Display for RectI32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_row(r: &RectI32) -> u32 {
    point_i32::delta_row(&r.min, &r.max)
}

pub fn delta_col(r: &RectI32) -> u32 {
    point_i32::delta_col(&r.min, &r.max)
}

pub fn max_delta(r: &RectI32) -> u32 {
    std::cmp::max(delta_row(r), delta_col(r))
}

pub fn len_row(r: &RectI32) -> u32 {
    delta_row(r) + 1
}

pub fn len_col(r: &RectI32) -> u32 {
    delta_col(r) + 1
}

pub fn max_len(r: &RectI32) -> u32 {
    std::cmp::max(len_row(r), len_col(r))
}

#[cfg(test)]
mod tests {
    use super::{RectI32, delta_col, delta_row, len_col, len_row, max_delta, max_len};
    use crate::matrix::{
        point::point_i32::PointI32,
        rect::{rect_i8::RectI8, rect_i16::RectI16},
    };

    #[test]
    fn rect_i32() {
        assert_eq!(RectI32::largest(), RectI32 { min: PointI32 { row: i32::MIN, col: i32::MIN }, max: PointI32 { row: i32::MAX, col: i32::MAX } });
        assert_eq!(RectI32::min(), RectI32 { min: PointI32 { row: i32::MIN, col: i32::MIN }, max: PointI32 { row: i32::MIN, col: i32::MIN } });
        assert_eq!(RectI32::max(), RectI32 { min: PointI32 { row: i32::MAX, col: i32::MAX }, max: PointI32 { row: i32::MAX, col: i32::MAX } });
        assert_eq!(RectI32::of(i32::MIN, -1, 1, i32::MAX), RectI32 { min: PointI32 { row: i32::MIN, col: -1 }, max: PointI32 { row: 1, col: i32::MAX } });
    }

    #[test]
    fn to_string() {
        assert_eq!(RectI32::largest().to_string(), "((-2147483648, -2147483648), (2147483647, 2147483647))");
        assert_eq!(RectI32::of(i32::MIN, -0, 0, i32::MAX).to_string(), "((-2147483648, 0), (0, 2147483647))");
    }

    #[test]
    fn from() {
        assert_eq!(RectI32::from(RectI8::largest()), RectI32 { min: PointI32 { row: i8::MIN.into(), col: i8::MIN.into() }, max: PointI32 { row: i8::MAX.into(), col: i8::MAX.into() } });
        assert_eq!(RectI32::from(RectI16::largest()), RectI32 { min: PointI32 { row: i16::MIN.into(), col: i16::MIN.into() }, max: PointI32 { row: i16::MAX.into(), col: i16::MAX.into() } });
    }

    #[test]
    fn iter_row() {
        assert_eq!(RectI32::of(-6, -8, -7, -6).iter_row().collect::<Vec<i32>>(), []);
        assert_eq!(RectI32::of(-6, -8, -6, -6).iter_row().collect::<Vec<i32>>(), [-6]);
        assert_eq!(RectI32::of(-6, -8, -5, -6).iter_row().collect::<Vec<i32>>(), [-6, -5]);
        assert_eq!(RectI32::of(-6, -8, -4, -6).iter_row().collect::<Vec<i32>>(), [-6, -5, -4]);
        assert_eq!(RectI32::of(-6, -8, -3, -6).iter_row().collect::<Vec<i32>>(), [-6, -5, -4, -3]);
        assert_eq!(RectI32::of(-6, -8, -3, -6).iter_row().rev().collect::<Vec<i32>>(), [-3, -4, -5, -6]);
        assert_eq!(RectI32::of(-6, -8, -4, -6).iter_row().rev().collect::<Vec<i32>>(), [-4, -5, -6]);
        assert_eq!(RectI32::of(-6, -8, -5, -6).iter_row().rev().collect::<Vec<i32>>(), [-5, -6]);
        assert_eq!(RectI32::of(-6, -8, -6, -6).iter_row().rev().collect::<Vec<i32>>(), [-6]);
        assert_eq!(RectI32::of(-6, -8, -7, -6).iter_row().rev().collect::<Vec<i32>>(), []);
    }

    #[test]
    fn iter_col() {
        assert_eq!(RectI32::of(-6, -8, -4, -9).iter_col().collect::<Vec<i32>>(), []);
        assert_eq!(RectI32::of(-6, -8, -4, -8).iter_col().collect::<Vec<i32>>(), [-8]);
        assert_eq!(RectI32::of(-6, -8, -4, -7).iter_col().collect::<Vec<i32>>(), [-8, -7]);
        assert_eq!(RectI32::of(-6, -8, -4, -6).iter_col().collect::<Vec<i32>>(), [-8, -7, -6]);
        assert_eq!(RectI32::of(-6, -8, -4, -5).iter_col().collect::<Vec<i32>>(), [-8, -7, -6, -5]);
        assert_eq!(RectI32::of(-6, -8, -4, -5).iter_col().rev().collect::<Vec<i32>>(), [-5, -6, -7, -8]);
        assert_eq!(RectI32::of(-6, -8, -4, -6).iter_col().rev().collect::<Vec<i32>>(), [-6, -7, -8]);
        assert_eq!(RectI32::of(-6, -8, -4, -7).iter_col().rev().collect::<Vec<i32>>(), [-7, -8]);
        assert_eq!(RectI32::of(-6, -8, -4, -8).iter_col().rev().collect::<Vec<i32>>(), [-8]);
        assert_eq!(RectI32::of(-6, -8, -4, -9).iter_col().rev().collect::<Vec<i32>>(), []);
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&RectI32::of(0, i32::MIN, 0, i32::MAX)), 0);
        assert_eq!(delta_row(&RectI32::of(i32::MIN, 0, i32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&RectI32::of(i32::MIN, 0, i32::MAX, 0)), 0);
        assert_eq!(delta_col(&RectI32::of(0, i32::MIN, 0, i32::MAX)), u32::MAX);
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
    fn test_len_row() {
        assert_eq!(len_row(&RectI32::of(0, i32::MIN, 0, i32::MAX)), 1);
        assert_eq!(len_row(&RectI32::of(i32::MIN, 0, i32::MAX - 1, 0)), u32::MAX);
    }

    #[test]
    fn test_len_col() {
        assert_eq!(len_col(&RectI32::of(i32::MIN, 0, i32::MAX, 0)), 1);
        assert_eq!(len_col(&RectI32::of(0, i32::MIN, 0, i32::MAX - 1)), u32::MAX);
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
