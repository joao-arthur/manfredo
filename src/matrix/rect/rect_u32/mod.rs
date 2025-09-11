use crate::matrix::{
    point::point_u32,
    rect::{rect_u8, rect_u16},
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
pub struct Rect {
    pub min: point_u32::Point,
    pub max: point_u32::Point,
}

impl Rect {
    pub fn of(row1: u32, col1: u32, row2: u32, col2: u32) -> Self {
        Rect { min: point_u32::Point::of(row1, col1), max: point_u32::Point::of(row2, col2) }
    }

    pub fn largest() -> Self {
        Rect { min: point_u32::Point::min(), max: point_u32::Point::max() }
    }

    pub fn min() -> Self {
        Rect { min: point_u32::Point::min(), max: point_u32::Point::min() }
    }

    pub fn max() -> Self {
        Rect { min: point_u32::Point::max(), max: point_u32::Point::max() }
    }

    pub fn iter_row(&self) -> RangeInclusive<u32> {
        self.min.row..=self.max.row
    }

    pub fn iter_col(&self) -> RangeInclusive<u32> {
        self.min.col..=self.max.col
    }
}

impl From<rect_u8::Rect> for Rect {
    fn from(r: rect_u8::Rect) -> Self {
        Rect { min: point_u32::Point::from(r.min), max: point_u32::Point::from(r.max) }
    }
}

impl From<rect_u16::Rect> for Rect {
    fn from(r: rect_u16::Rect) -> Self {
        Rect { min: point_u32::Point::from(r.min), max: point_u32::Point::from(r.max) }
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_row(r: &Rect) -> u32 {
    point_u32::delta_row(&r.min, &r.max)
}

pub fn delta_col(r: &Rect) -> u32 {
    point_u32::delta_col(&r.min, &r.max)
}

pub fn max_delta(r: &Rect) -> u32 {
    std::cmp::max(delta_row(r), delta_col(r))
}

pub fn len_row(r: &Rect) -> u32 {
    delta_row(r) + 1
}

pub fn len_col(r: &Rect) -> u32 {
    delta_col(r) + 1
}

pub fn max_len(r: &Rect) -> u32 {
    std::cmp::max(len_row(r), len_col(r))
}

#[cfg(test)]
mod tests {
    use super::{Rect, delta_col, delta_row, len_col, len_row, max_delta, max_len};
    use crate::matrix::{
        point::point_u32::Point,
        rect::{rect_u8, rect_u16},
    };

    #[test]
    fn rect_u32() {
        assert_eq!(Rect::largest(), Rect { min: Point { row: 0, col: 0 }, max: Point { row: u32::MAX, col: u32::MAX } });
        assert_eq!(Rect::min(), Rect { min: Point { row: 0, col: 0 }, max: Point { row: 0, col: 0 } });
        assert_eq!(Rect::max(), Rect { min: Point { row: u32::MAX, col: u32::MAX }, max: Point { row: u32::MAX, col: u32::MAX } });
        assert_eq!(Rect::of(256, 512, 1024, 2048), Rect { min: Point { row: 256, col: 512 }, max: Point { row: 1024, col: 2048 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::of(256, 512, 1024, 2048).to_string(), "((256, 512), (1024, 2048))");
        assert_eq!(Rect::of(u32::MAX, 0, 0, u32::MAX).to_string(), "((4294967295, 0), (0, 4294967295))");
    }

    #[test]
    fn from() {
        assert_eq!(Rect::from(rect_u8::Rect::largest()), Rect { min: Point { row: 0, col: 0 }, max: Point { row: u8::MAX.into(), col: u8::MAX.into() } });
        assert_eq!(Rect::from(rect_u16::Rect::largest()), Rect { min: Point { row: 0, col: 0 }, max: Point { row: u16::MAX.into(), col: u16::MAX.into() } });
    }

    #[test]
    fn iter_row() {
        assert_eq!(Rect::of(3, 6, 2, 8).iter_row().collect::<Vec<u32>>(), []);
        assert_eq!(Rect::of(3, 6, 3, 8).iter_row().collect::<Vec<u32>>(), [3]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_row().collect::<Vec<u32>>(), [3, 4]);
        assert_eq!(Rect::of(3, 6, 5, 8).iter_row().collect::<Vec<u32>>(), [3, 4, 5]);
        assert_eq!(Rect::of(3, 6, 6, 8).iter_row().collect::<Vec<u32>>(), [3, 4, 5, 6]);
        assert_eq!(Rect::of(3, 6, 6, 8).iter_row().rev().collect::<Vec<u32>>(), [6, 5, 4, 3]);
        assert_eq!(Rect::of(3, 6, 5, 8).iter_row().rev().collect::<Vec<u32>>(), [5, 4, 3]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_row().rev().collect::<Vec<u32>>(), [4, 3]);
        assert_eq!(Rect::of(3, 6, 3, 8).iter_row().rev().collect::<Vec<u32>>(), [3]);
        assert_eq!(Rect::of(3, 6, 2, 8).iter_row().rev().collect::<Vec<u32>>(), []);
    }

    #[test]
    fn iter_col() {
        assert_eq!(Rect::of(3, 6, 4, 5).iter_col().collect::<Vec<u32>>(), []);
        assert_eq!(Rect::of(3, 6, 4, 6).iter_col().collect::<Vec<u32>>(), [6]);
        assert_eq!(Rect::of(3, 6, 4, 7).iter_col().collect::<Vec<u32>>(), [6, 7]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_col().collect::<Vec<u32>>(), [6, 7, 8]);
        assert_eq!(Rect::of(3, 6, 4, 9).iter_col().collect::<Vec<u32>>(), [6, 7, 8, 9]);
        assert_eq!(Rect::of(3, 6, 4, 9).iter_col().rev().collect::<Vec<u32>>(), [9, 8, 7, 6]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_col().rev().collect::<Vec<u32>>(), [8, 7, 6]);
        assert_eq!(Rect::of(3, 6, 4, 7).iter_col().rev().collect::<Vec<u32>>(), [7, 6]);
        assert_eq!(Rect::of(3, 6, 4, 6).iter_col().rev().collect::<Vec<u32>>(), [6]);
        assert_eq!(Rect::of(3, 6, 4, 5).iter_col().rev().collect::<Vec<u32>>(), []);
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&Rect::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_row(&Rect::of(0, 0, u32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&Rect::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_col(&Rect::of(0, 0, 0, u32::MAX)), u32::MAX);
    }

    #[test]
    fn test_max_delta() {
        assert_eq!(max_delta(&Rect::of(0, 5, 10, 10)), 10);
        assert_eq!(max_delta(&Rect::of(5, 0, 9, 9)), 9);
    }

    #[test]
    fn max_delta_0() {
        assert_eq!(max_delta(&Rect::of(0, 0, 0, 0)), 0);
        assert_eq!(max_delta(&Rect::of(1, 1, 1, 1)), 0);
        assert_eq!(max_delta(&Rect::of(5, 10, 5, 10)), 0);
    }

    #[test]
    fn max_delta_1() {
        assert_eq!(max_delta(&Rect::of(0, 0, 1, 1)), 1);
        assert_eq!(max_delta(&Rect::of(5, 5, 6, 6)), 1);
        assert_eq!(max_delta(&Rect::of(0, 0, 0, 1)), 1);
        assert_eq!(max_delta(&Rect::of(5, 9, 5, 10)), 1);
    }

    #[test]
    fn max_delta_bounds() {
        assert_eq!(max_delta(&Rect::of(0, 0, u32::MAX, u32::MAX - 1)), u32::MAX);
        assert_eq!(max_delta(&Rect::of(0, 0, u32::MAX - 1, u32::MAX)), u32::MAX);
    }

    #[test]
    fn test_len_row() {
        assert_eq!(len_row(&Rect::of(0, 0, 0, 0)), 1);
        assert_eq!(len_row(&Rect::of(0, 0, u32::MAX - 1, 0)), u32::MAX);
    }

    #[test]
    fn test_len_col() {
        assert_eq!(len_col(&Rect::of(0, 0, 0, 0)), 1);
        assert_eq!(len_col(&Rect::of(0, 0, 0, u32::MAX - 1)), u32::MAX);
    }

    #[test]
    fn test_max_len() {
        assert_eq!(max_len(&Rect::of(0, 5, 10, 10)), 11);
        assert_eq!(max_len(&Rect::of(5, 0, 9, 9)), 10);
    }

    #[test]
    fn max_len_1() {
        assert_eq!(max_len(&Rect::of(0, 0, 0, 0)), 1);
        assert_eq!(max_len(&Rect::of(1, 1, 1, 1)), 1);
        assert_eq!(max_len(&Rect::of(5, 10, 5, 10)), 1);
    }

    #[test]
    fn max_len_2() {
        assert_eq!(max_len(&Rect::of(0, 0, 1, 1)), 2);
        assert_eq!(max_len(&Rect::of(5, 5, 6, 6)), 2);
        assert_eq!(max_len(&Rect::of(0, 0, 0, 1)), 2);
        assert_eq!(max_len(&Rect::of(5, 9, 5, 10)), 2);
    }

    #[test]
    fn max_len_bounds() {
        assert_eq!(max_len(&Rect::of(1, 0, u32::MAX - 1, u32::MAX - 1)), u32::MAX);
        assert_eq!(max_len(&Rect::of(0, 1, u32::MAX - 1, u32::MAX - 1)), u32::MAX);
        assert_eq!(max_len(&Rect::of(0, 0, u32::MAX - 2, u32::MAX - 1)), u32::MAX);
        assert_eq!(max_len(&Rect::of(0, 0, u32::MAX - 1, u32::MAX - 2)), u32::MAX);
    }
}
