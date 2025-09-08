use crate::matrix::{
    point::point_u64,
    rect::{rect_u8::RectU8, rect_u16::RectU16, rect_u32::RectU32},
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
pub struct RectU64 {
    pub min: point_u64::PointU64,
    pub max: point_u64::PointU64,
}

impl RectU64 {
    pub fn of(row1: u64, col1: u64, row2: u64, col2: u64) -> Self {
        RectU64 { min: point_u64::PointU64::of(row1, col1), max: point_u64::PointU64::of(row2, col2) }
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

    pub fn iter_row(&self) -> RangeInclusive<u64> {
        self.min.row..=self.max.row
    }

    pub fn iter_col(&self) -> RangeInclusive<u64> {
        self.min.col..=self.max.col
    }
}

impl From<RectU8> for RectU64 {
    fn from(r: RectU8) -> Self {
        RectU64 { min: point_u64::PointU64::of(r.min.row.into(), r.min.col.into()), max: point_u64::PointU64::of(r.max.row.into(), r.max.col.into()) }
    }
}

impl From<RectU16> for RectU64 {
    fn from(r: RectU16) -> Self {
        RectU64 { min: point_u64::PointU64::of(r.min.row.into(), r.min.col.into()), max: point_u64::PointU64::of(r.max.row.into(), r.max.col.into()) }
    }
}

impl From<RectU32> for RectU64 {
    fn from(r: RectU32) -> Self {
        RectU64 { min: point_u64::PointU64::of(r.min.row.into(), r.min.col.into()), max: point_u64::PointU64::of(r.max.row.into(), r.max.col.into()) }
    }
}

impl std::fmt::Display for RectU64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_row(r: &RectU64) -> u64 {
    point_u64::delta_row(&r.min, &r.max)
}

pub fn delta_col(r: &RectU64) -> u64 {
    point_u64::delta_col(&r.min, &r.max)
}

pub fn max_delta(r: &RectU64) -> u64 {
    std::cmp::max(delta_row(r), delta_col(r))
}

pub fn len_row(r: &RectU64) -> u64 {
    delta_row(r) + 1
}

pub fn len_col(r: &RectU64) -> u64 {
    delta_col(r) + 1
}

pub fn max_len(r: &RectU64) -> u64 {
    std::cmp::max(len_row(r), len_col(r))
}

#[cfg(test)]
mod tests {
    use super::{RectU64, delta_col, delta_row, len_col, len_row, max_delta, max_len};
    use crate::matrix::{
        point::point_u64::PointU64,
        rect::{rect_u8::RectU8, rect_u16::RectU16, rect_u32::RectU32},
    };

    #[test]
    fn rect_u64() {
        assert_eq!(RectU64::largest(), RectU64 { min: PointU64 { row: 0, col: 0 }, max: PointU64 { row: u64::MAX, col: u64::MAX } });
        assert_eq!(RectU64::min(), RectU64 { min: PointU64 { row: 0, col: 0 }, max: PointU64 { row: 0, col: 0 } });
        assert_eq!(RectU64::max(), RectU64 { min: PointU64 { row: u64::MAX, col: u64::MAX }, max: PointU64 { row: u64::MAX, col: u64::MAX } });
        assert_eq!(RectU64::of(4096, 8192, 16384, 32768), RectU64 { min: PointU64 { row: 4096, col: 8192 }, max: PointU64 { row: 16384, col: 32768 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(RectU64::of(4096, 8192, 16384, 32768).to_string(), "((4096, 8192), (16384, 32768))");
        assert_eq!(RectU64::of(u64::MAX, 0, 0, u64::MAX).to_string(), "((18446744073709551615, 0), (0, 18446744073709551615))");
    }

    #[test]
    fn from() {
        assert_eq!(RectU64::from(RectU8::largest()), RectU64 { min: PointU64 { row: 0, col: 0 }, max: PointU64 { row: u8::MAX.into(), col: u8::MAX.into() } });
        assert_eq!(RectU64::from(RectU16::largest()), RectU64 { min: PointU64 { row: 0, col: 0 }, max: PointU64 { row: u16::MAX.into(), col: u16::MAX.into() } });
        assert_eq!(RectU64::from(RectU32::largest()), RectU64 { min: PointU64 { row: 0, col: 0 }, max: PointU64 { row: u32::MAX.into(), col: u32::MAX.into() } });
    }

    #[test]
    fn iter_row() {
        assert_eq!(RectU64::of(3, 6, 2, 8).iter_row().collect::<Vec<u64>>(), []);
        assert_eq!(RectU64::of(3, 6, 3, 8).iter_row().collect::<Vec<u64>>(), [3]);
        assert_eq!(RectU64::of(3, 6, 4, 8).iter_row().collect::<Vec<u64>>(), [3, 4]);
        assert_eq!(RectU64::of(3, 6, 5, 8).iter_row().collect::<Vec<u64>>(), [3, 4, 5]);
        assert_eq!(RectU64::of(3, 6, 6, 8).iter_row().collect::<Vec<u64>>(), [3, 4, 5, 6]);
        assert_eq!(RectU64::of(3, 6, 6, 8).iter_row().rev().collect::<Vec<u64>>(), [6, 5, 4, 3]);
        assert_eq!(RectU64::of(3, 6, 5, 8).iter_row().rev().collect::<Vec<u64>>(), [5, 4, 3]);
        assert_eq!(RectU64::of(3, 6, 4, 8).iter_row().rev().collect::<Vec<u64>>(), [4, 3]);
        assert_eq!(RectU64::of(3, 6, 3, 8).iter_row().rev().collect::<Vec<u64>>(), [3]);
        assert_eq!(RectU64::of(3, 6, 2, 8).iter_row().rev().collect::<Vec<u64>>(), []);
    }

    #[test]
    fn iter_col() {
        assert_eq!(RectU64::of(3, 6, 4, 5).iter_col().collect::<Vec<u64>>(), []);
        assert_eq!(RectU64::of(3, 6, 4, 6).iter_col().collect::<Vec<u64>>(), [6]);
        assert_eq!(RectU64::of(3, 6, 4, 7).iter_col().collect::<Vec<u64>>(), [6, 7]);
        assert_eq!(RectU64::of(3, 6, 4, 8).iter_col().collect::<Vec<u64>>(), [6, 7, 8]);
        assert_eq!(RectU64::of(3, 6, 4, 9).iter_col().collect::<Vec<u64>>(), [6, 7, 8, 9]);
        assert_eq!(RectU64::of(3, 6, 4, 9).iter_col().rev().collect::<Vec<u64>>(), [9, 8, 7, 6]);
        assert_eq!(RectU64::of(3, 6, 4, 8).iter_col().rev().collect::<Vec<u64>>(), [8, 7, 6]);
        assert_eq!(RectU64::of(3, 6, 4, 7).iter_col().rev().collect::<Vec<u64>>(), [7, 6]);
        assert_eq!(RectU64::of(3, 6, 4, 6).iter_col().rev().collect::<Vec<u64>>(), [6]);
        assert_eq!(RectU64::of(3, 6, 4, 5).iter_col().rev().collect::<Vec<u64>>(), []);
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&RectU64::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_row(&RectU64::of(0, 0, u64::MAX, 0)), u64::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&RectU64::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_col(&RectU64::of(0, 0, 0, u64::MAX)), u64::MAX);
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
    fn test_len_row() {
        assert_eq!(len_row(&RectU64::of(0, 0, 0, 0)), 1);
        assert_eq!(len_row(&RectU64::of(0, 0, u64::MAX - 1, 0)), u64::MAX);
    }

    #[test]
    fn test_len_col() {
        assert_eq!(len_col(&RectU64::of(0, 0, 0, 0)), 1);
        assert_eq!(len_col(&RectU64::of(0, 0, 0, u64::MAX - 1)), u64::MAX);
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
