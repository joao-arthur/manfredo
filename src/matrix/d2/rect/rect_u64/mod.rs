use crate::matrix::d2::{
    point::point_u64,
    rect::{rect_u8, rect_u16, rect_u32},
};
use std::ops::RangeInclusive;

mod add;
mod area;
mod contains_point;
mod contains_rect;
mod deflate;
mod delta;
mod inflate;
mod len;
mod resize;
mod translate;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use self::area::area;
pub use self::contains_point::contains_point;
pub use self::contains_rect::contains_rect;
pub use self::deflate::{deflate, deflate_assign, try_deflate, try_deflate_assign};
pub use self::delta::{delta_col, delta_max, delta_min, delta_row};
pub use self::inflate::{
    checked_inflate, checked_inflate_assign, saturating_inflate, saturating_inflate_assign, try_checked_inflate, try_checked_inflate_assign, try_saturating_inflate, try_saturating_inflate_assign,
    wrapping_inflate, wrapping_inflate_assign,
};
pub use self::len::{len_col, len_max, len_row};
pub use self::resize::{
    checked_resize, checked_resize_assign, saturating_resize, saturating_resize_assign, try_checked_resize, try_checked_resize_assign, try_saturating_resize, try_saturating_resize_assign,
    try_wrapping_resize, try_wrapping_resize_assign, wrapping_resize, wrapping_resize_assign,
};
pub use self::translate::{
    checked_translate, checked_translate_assign, saturating_translate, saturating_translate_assign, try_checked_translate, try_checked_translate_assign, wrapping_translate, wrapping_translate_assign,
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Rect {
    pub min: point_u64::Point,
    pub max: point_u64::Point,
}

impl Rect {
    pub fn of(row1: u64, col1: u64, row2: u64, col2: u64) -> Self {
        Rect { min: point_u64::Point::of(row1, col1), max: point_u64::Point::of(row2, col2) }
    }

    pub fn largest() -> Self {
        Rect { min: point_u64::Point::min(), max: point_u64::Point::max() }
    }

    pub fn min() -> Self {
        Rect { min: point_u64::Point::min(), max: point_u64::Point::min() }
    }

    pub fn max() -> Self {
        Rect { min: point_u64::Point::max(), max: point_u64::Point::max() }
    }

    pub fn iter_row(&self) -> RangeInclusive<u64> {
        self.min.row..=self.max.row
    }

    pub fn iter_col(&self) -> RangeInclusive<u64> {
        self.min.col..=self.max.col
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

impl From<rect_u8::Rect> for Rect {
    fn from(r: rect_u8::Rect) -> Self {
        Rect { min: point_u64::Point::from(r.min), max: point_u64::Point::from(r.max) }
    }
}

impl From<rect_u16::Rect> for Rect {
    fn from(r: rect_u16::Rect) -> Self {
        Rect { min: point_u64::Point::from(r.min), max: point_u64::Point::from(r.max) }
    }
}

impl From<rect_u32::Rect> for Rect {
    fn from(r: rect_u32::Rect) -> Self {
        Rect { min: point_u64::Point::from(r.min), max: point_u64::Point::from(r.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Rect;
    use crate::matrix::{
        d1::point::point_u64::MAX,
        d2::{
            point::point_u64::Point,
            rect::{rect_u8, rect_u16, rect_u32},
        },
    };

    #[test]
    fn rect() {
        assert_eq!(Rect::largest(), Rect { min: Point { row: 0, col: 0 }, max: Point { row: MAX, col: MAX } });
        assert_eq!(Rect::min(), Rect { min: Point { row: 0, col: 0 }, max: Point { row: 0, col: 0 } });
        assert_eq!(Rect::max(), Rect { min: Point { row: MAX, col: MAX }, max: Point { row: MAX, col: MAX } });
        assert_eq!(Rect::of(4096, 8192, 16384, 32768), Rect { min: Point { row: 4096, col: 8192 }, max: Point { row: 16384, col: 32768 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::largest().to_string(), "((0, 0), (18446744073709551615, 18446744073709551615))");
        assert_eq!(Rect::of(4096, 8192, 16384, 32768).to_string(), "((4096, 8192), (16384, 32768))");
    }

    #[test]
    fn from() {
        assert_eq!(Rect::from(rect_u8::Rect::largest()), Rect { min: Point { row: 0, col: 0 }, max: Point { row: u8::MAX.into(), col: u8::MAX.into() } });
        assert_eq!(Rect::from(rect_u16::Rect::largest()), Rect { min: Point { row: 0, col: 0 }, max: Point { row: u16::MAX.into(), col: u16::MAX.into() } });
        assert_eq!(Rect::from(rect_u32::Rect::largest()), Rect { min: Point { row: 0, col: 0 }, max: Point { row: u32::MAX.into(), col: u32::MAX.into() } });
    }

    #[test]
    fn iter_row() {
        assert_eq!(Rect::of(3, 6, 2, 8).iter_row().collect::<Vec<u64>>(), []);
        assert_eq!(Rect::of(3, 6, 3, 8).iter_row().collect::<Vec<u64>>(), [3]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_row().collect::<Vec<u64>>(), [3, 4]);
        assert_eq!(Rect::of(3, 6, 5, 8).iter_row().collect::<Vec<u64>>(), [3, 4, 5]);
        assert_eq!(Rect::of(3, 6, 6, 8).iter_row().collect::<Vec<u64>>(), [3, 4, 5, 6]);
        assert_eq!(Rect::of(3, 6, 6, 8).iter_row().rev().collect::<Vec<u64>>(), [6, 5, 4, 3]);
        assert_eq!(Rect::of(3, 6, 5, 8).iter_row().rev().collect::<Vec<u64>>(), [5, 4, 3]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_row().rev().collect::<Vec<u64>>(), [4, 3]);
        assert_eq!(Rect::of(3, 6, 3, 8).iter_row().rev().collect::<Vec<u64>>(), [3]);
        assert_eq!(Rect::of(3, 6, 2, 8).iter_row().rev().collect::<Vec<u64>>(), []);
    }

    #[test]
    fn iter_col() {
        assert_eq!(Rect::of(3, 6, 4, 5).iter_col().collect::<Vec<u64>>(), []);
        assert_eq!(Rect::of(3, 6, 4, 6).iter_col().collect::<Vec<u64>>(), [6]);
        assert_eq!(Rect::of(3, 6, 4, 7).iter_col().collect::<Vec<u64>>(), [6, 7]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_col().collect::<Vec<u64>>(), [6, 7, 8]);
        assert_eq!(Rect::of(3, 6, 4, 9).iter_col().collect::<Vec<u64>>(), [6, 7, 8, 9]);
        assert_eq!(Rect::of(3, 6, 4, 9).iter_col().rev().collect::<Vec<u64>>(), [9, 8, 7, 6]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_col().rev().collect::<Vec<u64>>(), [8, 7, 6]);
        assert_eq!(Rect::of(3, 6, 4, 7).iter_col().rev().collect::<Vec<u64>>(), [7, 6]);
        assert_eq!(Rect::of(3, 6, 4, 6).iter_col().rev().collect::<Vec<u64>>(), [6]);
        assert_eq!(Rect::of(3, 6, 4, 5).iter_col().rev().collect::<Vec<u64>>(), []);
    }
}
