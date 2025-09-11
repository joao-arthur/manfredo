use crate::matrix::point::point_u8;
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
    pub min: point_u8::PointU8,
    pub max: point_u8::PointU8,
}

impl Rect {
    pub fn of(row1: u8, col1: u8, row2: u8, col2: u8) -> Self {
        Rect { min: point_u8::PointU8::of(row1, col1), max: point_u8::PointU8::of(row2, col2) }
    }

    pub fn largest() -> Self {
        Rect { min: point_u8::PointU8::min(), max: point_u8::PointU8::max() }
    }

    pub fn min() -> Self {
        Rect { min: point_u8::PointU8::min(), max: point_u8::PointU8::min() }
    }

    pub fn max() -> Self {
        Rect { min: point_u8::PointU8::max(), max: point_u8::PointU8::max() }
    }

    pub fn iter_row(&self) -> RangeInclusive<u8> {
        self.min.row..=self.max.row
    }

    pub fn iter_col(&self) -> RangeInclusive<u8> {
        self.min.col..=self.max.col
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_row(r: &Rect) -> u8 {
    point_u8::delta_row(&r.min, &r.max)
}

pub fn delta_col(r: &Rect) -> u8 {
    point_u8::delta_col(&r.min, &r.max)
}

pub fn max_delta(r: &Rect) -> u8 {
    std::cmp::max(delta_row(r), delta_col(r))
}

pub fn len_row(r: &Rect) -> u8 {
    delta_row(r) + 1
}

pub fn len_col(r: &Rect) -> u8 {
    delta_col(r) + 1
}

pub fn max_len(r: &Rect) -> u8 {
    std::cmp::max(len_row(r), len_col(r))
}

#[cfg(test)]
mod tests {
    use super::{Rect, delta_col, delta_row, len_col, len_row, max_delta, max_len};
    use crate::matrix::point::point_u8::PointU8;

    #[test]
    fn rect_u8() {
        assert_eq!(Rect::largest(), Rect { min: PointU8 { row: 0, col: 0 }, max: PointU8 { row: u8::MAX, col: u8::MAX } });
        assert_eq!(Rect::min(), Rect { min: PointU8 { row: 0, col: 0 }, max: PointU8 { row: 0, col: 0 } });
        assert_eq!(Rect::max(), Rect { min: PointU8 { row: u8::MAX, col: u8::MAX }, max: PointU8 { row: u8::MAX, col: u8::MAX } });
        assert_eq!(Rect::of(0, 2, 4, 8), Rect { min: PointU8 { row: 0, col: 2 }, max: PointU8 { row: 4, col: 8 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::of(0, 2, 4, 8).to_string(), "((0, 2), (4, 8))");
        assert_eq!(Rect::of(u8::MAX, 0, 0, u8::MAX).to_string(), "((255, 0), (0, 255))");
    }

    #[test]
    fn iter_row() {
        assert_eq!(Rect::of(3, 6, 2, 8).iter_row().collect::<Vec<u8>>(), []);
        assert_eq!(Rect::of(3, 6, 3, 8).iter_row().collect::<Vec<u8>>(), [3]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_row().collect::<Vec<u8>>(), [3, 4]);
        assert_eq!(Rect::of(3, 6, 5, 8).iter_row().collect::<Vec<u8>>(), [3, 4, 5]);
        assert_eq!(Rect::of(3, 6, 6, 8).iter_row().collect::<Vec<u8>>(), [3, 4, 5, 6]);
        assert_eq!(Rect::of(3, 6, 6, 8).iter_row().rev().collect::<Vec<u8>>(), [6, 5, 4, 3]);
        assert_eq!(Rect::of(3, 6, 5, 8).iter_row().rev().collect::<Vec<u8>>(), [5, 4, 3]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_row().rev().collect::<Vec<u8>>(), [4, 3]);
        assert_eq!(Rect::of(3, 6, 3, 8).iter_row().rev().collect::<Vec<u8>>(), [3]);
        assert_eq!(Rect::of(3, 6, 2, 8).iter_row().rev().collect::<Vec<u8>>(), []);
    }

    #[test]
    fn iter_col() {
        assert_eq!(Rect::of(3, 6, 4, 5).iter_col().collect::<Vec<u8>>(), []);
        assert_eq!(Rect::of(3, 6, 4, 6).iter_col().collect::<Vec<u8>>(), [6]);
        assert_eq!(Rect::of(3, 6, 4, 7).iter_col().collect::<Vec<u8>>(), [6, 7]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_col().collect::<Vec<u8>>(), [6, 7, 8]);
        assert_eq!(Rect::of(3, 6, 4, 9).iter_col().collect::<Vec<u8>>(), [6, 7, 8, 9]);
        assert_eq!(Rect::of(3, 6, 4, 9).iter_col().rev().collect::<Vec<u8>>(), [9, 8, 7, 6]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_col().rev().collect::<Vec<u8>>(), [8, 7, 6]);
        assert_eq!(Rect::of(3, 6, 4, 7).iter_col().rev().collect::<Vec<u8>>(), [7, 6]);
        assert_eq!(Rect::of(3, 6, 4, 6).iter_col().rev().collect::<Vec<u8>>(), [6]);
        assert_eq!(Rect::of(3, 6, 4, 5).iter_col().rev().collect::<Vec<u8>>(), []);
    }

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&Rect::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_row(&Rect::of(0, 0, u8::MAX, 0)), u8::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&Rect::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_col(&Rect::of(0, 0, 0, u8::MAX)), u8::MAX);
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
        assert_eq!(max_delta(&Rect::of(0, 0, u8::MAX, u8::MAX - 1)), u8::MAX);
        assert_eq!(max_delta(&Rect::of(0, 0, u8::MAX - 1, u8::MAX)), u8::MAX);
    }

    #[test]
    fn test_len_row() {
        assert_eq!(len_row(&Rect::of(0, 0, 0, 0)), 1);
        assert_eq!(len_row(&Rect::of(0, 0, u8::MAX - 1, 0)), u8::MAX);
    }

    #[test]
    fn test_len_col() {
        assert_eq!(len_col(&Rect::of(0, 0, 0, 0)), 1);
        assert_eq!(len_col(&Rect::of(0, 0, 0, u8::MAX - 1)), u8::MAX);
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
        assert_eq!(max_len(&Rect::of(1, 0, u8::MAX - 1, u8::MAX - 1)), u8::MAX);
        assert_eq!(max_len(&Rect::of(0, 1, u8::MAX - 1, u8::MAX - 1)), u8::MAX);
        assert_eq!(max_len(&Rect::of(0, 0, u8::MAX - 2, u8::MAX - 1)), u8::MAX);
        assert_eq!(max_len(&Rect::of(0, 0, u8::MAX - 1, u8::MAX - 2)), u8::MAX);
    }
}
