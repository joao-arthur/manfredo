use crate::matrix::d2::{point::point_u16, rect::rect_u8};
use std::ops::RangeInclusive;

mod add;
mod contains_point;
mod contains_rect;
mod deflate;
mod delta;
mod inflate;
mod len;
mod resize;
mod translate;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::contains_point::contains_point;
pub use self::contains_rect::contains_rect;
pub use self::deflate::{deflate, deflate_assign, try_deflate, try_deflate_assign};
pub use self::delta::{delta_col, delta_row, max_delta};
pub use self::inflate::{
    checked_inflate, checked_inflate_assign, saturating_inflate, saturating_inflate_assign, try_checked_inflate, try_checked_inflate_assign, try_saturating_inflate, try_saturating_inflate_assign,
    wrapping_inflate, wrapping_inflate_assign,
};
pub use self::len::{len_col, len_row, max_len};
pub use self::resize::{
    checked_resize, checked_resize_assign, saturating_resize, saturating_resize_assign, try_checked_resize, try_checked_resize_assign, try_saturating_resize, try_saturating_resize_assign,
    try_wrapping_resize, try_wrapping_resize_assign, wrapping_resize, wrapping_resize_assign,
};
pub use self::translate::{
    checked_translate, checked_translate_assign, saturating_translate, saturating_translate_assign, try_checked_translate, try_checked_translate_assign, wrapping_translate, wrapping_translate_assign,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Rect {
    pub min: point_u16::Point,
    pub max: point_u16::Point,
}

impl Rect {
    pub fn of(row1: u16, col1: u16, row2: u16, col2: u16) -> Self {
        Rect { min: point_u16::Point::of(row1, col1), max: point_u16::Point::of(row2, col2) }
    }

    pub fn largest() -> Self {
        Rect { min: point_u16::Point::min(), max: point_u16::Point::max() }
    }

    pub fn min() -> Self {
        Rect { min: point_u16::Point::min(), max: point_u16::Point::min() }
    }

    pub fn max() -> Self {
        Rect { min: point_u16::Point::max(), max: point_u16::Point::max() }
    }

    pub fn iter_row(&self) -> RangeInclusive<u16> {
        self.min.row..=self.max.row
    }

    pub fn iter_col(&self) -> RangeInclusive<u16> {
        self.min.col..=self.max.col
    }
}

impl From<rect_u8::Rect> for Rect {
    fn from(r: rect_u8::Rect) -> Self {
        Rect { min: point_u16::Point::from(r.min), max: point_u16::Point::from(r.max) }
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

#[cfg(test)]
mod tests {
    use super::Rect;
    use crate::matrix::d2::{point::point_u16::Point, rect::rect_u8};

    #[test]
    fn rect_u16() {
        assert_eq!(Rect::largest(), Rect { min: Point { row: 0, col: 0 }, max: Point { row: u16::MAX, col: u16::MAX } });
        assert_eq!(Rect::min(), Rect { min: Point { row: 0, col: 0 }, max: Point { row: 0, col: 0 } });
        assert_eq!(Rect::max(), Rect { min: Point { row: u16::MAX, col: u16::MAX }, max: Point { row: u16::MAX, col: u16::MAX } });
        assert_eq!(Rect::of(16, 32, 64, 128), Rect { min: Point { row: 16, col: 32 }, max: Point { row: 64, col: 128 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::of(16, 32, 64, 128).to_string(), "((16, 32), (64, 128))");
        assert_eq!(Rect::of(u16::MAX, 0, 0, u16::MAX).to_string(), "((65535, 0), (0, 65535))");
    }

    #[test]
    fn from() {
        assert_eq!(Rect::from(rect_u8::Rect::largest()), Rect { min: Point { row: 0, col: 0 }, max: Point { row: u8::MAX.into(), col: u8::MAX.into() } });
    }

    #[test]
    fn iter_row() {
        assert_eq!(Rect::of(3, 6, 2, 8).iter_row().collect::<Vec<u16>>(), []);
        assert_eq!(Rect::of(3, 6, 3, 8).iter_row().collect::<Vec<u16>>(), [3]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_row().collect::<Vec<u16>>(), [3, 4]);
        assert_eq!(Rect::of(3, 6, 5, 8).iter_row().collect::<Vec<u16>>(), [3, 4, 5]);
        assert_eq!(Rect::of(3, 6, 6, 8).iter_row().collect::<Vec<u16>>(), [3, 4, 5, 6]);
        assert_eq!(Rect::of(3, 6, 6, 8).iter_row().rev().collect::<Vec<u16>>(), [6, 5, 4, 3]);
        assert_eq!(Rect::of(3, 6, 5, 8).iter_row().rev().collect::<Vec<u16>>(), [5, 4, 3]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_row().rev().collect::<Vec<u16>>(), [4, 3]);
        assert_eq!(Rect::of(3, 6, 3, 8).iter_row().rev().collect::<Vec<u16>>(), [3]);
        assert_eq!(Rect::of(3, 6, 2, 8).iter_row().rev().collect::<Vec<u16>>(), []);
    }

    #[test]
    fn iter_col() {
        assert_eq!(Rect::of(3, 6, 4, 5).iter_col().collect::<Vec<u16>>(), []);
        assert_eq!(Rect::of(3, 6, 4, 6).iter_col().collect::<Vec<u16>>(), [6]);
        assert_eq!(Rect::of(3, 6, 4, 7).iter_col().collect::<Vec<u16>>(), [6, 7]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_col().collect::<Vec<u16>>(), [6, 7, 8]);
        assert_eq!(Rect::of(3, 6, 4, 9).iter_col().collect::<Vec<u16>>(), [6, 7, 8, 9]);
        assert_eq!(Rect::of(3, 6, 4, 9).iter_col().rev().collect::<Vec<u16>>(), [9, 8, 7, 6]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_col().rev().collect::<Vec<u16>>(), [8, 7, 6]);
        assert_eq!(Rect::of(3, 6, 4, 7).iter_col().rev().collect::<Vec<u16>>(), [7, 6]);
        assert_eq!(Rect::of(3, 6, 4, 6).iter_col().rev().collect::<Vec<u16>>(), [6]);
        assert_eq!(Rect::of(3, 6, 4, 5).iter_col().rev().collect::<Vec<u16>>(), []);
    }
}
