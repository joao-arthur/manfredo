use crate::matrix::d2::{
    point::point_i32,
    rect::{rect_i8, rect_i16},
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
    pub min: point_i32::Point,
    pub max: point_i32::Point,
}

impl Rect {
    pub fn of(row1: i32, col1: i32, row2: i32, col2: i32) -> Self {
        Rect { min: point_i32::Point::of(row1, col1), max: point_i32::Point::of(row2, col2) }
    }

    pub fn largest() -> Self {
        Rect { min: point_i32::Point::min(), max: point_i32::Point::max() }
    }

    pub fn min() -> Self {
        Rect { min: point_i32::Point::min(), max: point_i32::Point::min() }
    }

    pub fn max() -> Self {
        Rect { min: point_i32::Point::max(), max: point_i32::Point::max() }
    }

    pub fn iter_row(&self) -> RangeInclusive<i32> {
        self.min.row..=self.max.row
    }

    pub fn iter_col(&self) -> RangeInclusive<i32> {
        self.min.col..=self.max.col
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

impl From<rect_i8::Rect> for Rect {
    fn from(r: rect_i8::Rect) -> Self {
        Rect { min: point_i32::Point::from(r.min), max: point_i32::Point::from(r.max) }
    }
}

impl From<rect_i16::Rect> for Rect {
    fn from(r: rect_i16::Rect) -> Self {
        Rect { min: point_i32::Point::from(r.min), max: point_i32::Point::from(r.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Rect;
    use crate::matrix::d2::{
        point::point_i32::Point,
        rect::{rect_i8, rect_i16},
    };

    const MIN: i32 = i32::MIN;
    const MAX: i32 = i32::MAX;

    #[test]
    fn rect() {
        assert_eq!(Rect::largest(), Rect { min: Point { row: MIN, col: MIN }, max: Point { row: MAX, col: MAX } });
        assert_eq!(Rect::min(), Rect { min: Point { row: MIN, col: MIN }, max: Point { row: MIN, col: MIN } });
        assert_eq!(Rect::max(), Rect { min: Point { row: MAX, col: MAX }, max: Point { row: MAX, col: MAX } });
        assert_eq!(Rect::of(MIN, -1, 1, MAX), Rect { min: Point { row: MIN, col: -1 }, max: Point { row: 1, col: MAX } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::largest().to_string(), "((-2147483648, -2147483648), (2147483647, 2147483647))");
        assert_eq!(Rect::of(MIN, -0, 0, MAX).to_string(), "((-2147483648, 0), (0, 2147483647))");
    }

    #[test]
    fn from() {
        assert_eq!(Rect::from(rect_i8::Rect::largest()), Rect { min: Point { row: i8::MIN.into(), col: i8::MIN.into() }, max: Point { row: i8::MAX.into(), col: i8::MAX.into() } });
        assert_eq!(Rect::from(rect_i16::Rect::largest()), Rect { min: Point { row: i16::MIN.into(), col: i16::MIN.into() }, max: Point { row: i16::MAX.into(), col: i16::MAX.into() } });
    }

    #[test]
    fn iter_row() {
        assert_eq!(Rect::of(-6, -8, -7, -6).iter_row().collect::<Vec<i32>>(), []);
        assert_eq!(Rect::of(-6, -8, -6, -6).iter_row().collect::<Vec<i32>>(), [-6]);
        assert_eq!(Rect::of(-6, -8, -5, -6).iter_row().collect::<Vec<i32>>(), [-6, -5]);
        assert_eq!(Rect::of(-6, -8, -4, -6).iter_row().collect::<Vec<i32>>(), [-6, -5, -4]);
        assert_eq!(Rect::of(-6, -8, -3, -6).iter_row().collect::<Vec<i32>>(), [-6, -5, -4, -3]);
        assert_eq!(Rect::of(-6, -8, -3, -6).iter_row().rev().collect::<Vec<i32>>(), [-3, -4, -5, -6]);
        assert_eq!(Rect::of(-6, -8, -4, -6).iter_row().rev().collect::<Vec<i32>>(), [-4, -5, -6]);
        assert_eq!(Rect::of(-6, -8, -5, -6).iter_row().rev().collect::<Vec<i32>>(), [-5, -6]);
        assert_eq!(Rect::of(-6, -8, -6, -6).iter_row().rev().collect::<Vec<i32>>(), [-6]);
        assert_eq!(Rect::of(-6, -8, -7, -6).iter_row().rev().collect::<Vec<i32>>(), []);
    }

    #[test]
    fn iter_col() {
        assert_eq!(Rect::of(-6, -8, -4, -9).iter_col().collect::<Vec<i32>>(), []);
        assert_eq!(Rect::of(-6, -8, -4, -8).iter_col().collect::<Vec<i32>>(), [-8]);
        assert_eq!(Rect::of(-6, -8, -4, -7).iter_col().collect::<Vec<i32>>(), [-8, -7]);
        assert_eq!(Rect::of(-6, -8, -4, -6).iter_col().collect::<Vec<i32>>(), [-8, -7, -6]);
        assert_eq!(Rect::of(-6, -8, -4, -5).iter_col().collect::<Vec<i32>>(), [-8, -7, -6, -5]);
        assert_eq!(Rect::of(-6, -8, -4, -5).iter_col().rev().collect::<Vec<i32>>(), [-5, -6, -7, -8]);
        assert_eq!(Rect::of(-6, -8, -4, -6).iter_col().rev().collect::<Vec<i32>>(), [-6, -7, -8]);
        assert_eq!(Rect::of(-6, -8, -4, -7).iter_col().rev().collect::<Vec<i32>>(), [-7, -8]);
        assert_eq!(Rect::of(-6, -8, -4, -8).iter_col().rev().collect::<Vec<i32>>(), [-8]);
        assert_eq!(Rect::of(-6, -8, -4, -9).iter_col().rev().collect::<Vec<i32>>(), []);
    }
}
