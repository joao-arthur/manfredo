use crate::matrix::d2::{point::point_u16::Point, rect::rect_u8};
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
    pub min: Point,
    pub max: Point,
}

impl Rect {
    pub fn new(min: (u16, u16), max: (u16, u16)) -> Self {
        Rect { min: Point { row: min.0, col: min.1 }, max: Point { row: max.0, col: max.1 } }
    }

    pub fn largest() -> Self {
        Rect { min: Point::min(), max: Point::max() }
    }

    pub fn min() -> Self {
        Rect { min: Point::min(), max: Point::min() }
    }

    pub fn max() -> Self {
        Rect { min: Point::max(), max: Point::max() }
    }

    pub fn iter_row(&self) -> RangeInclusive<u16> {
        self.min.row..=self.max.row
    }

    pub fn iter_col(&self) -> RangeInclusive<u16> {
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
        Rect { min: Point::from(r.min), max: Point::from(r.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Rect;
    use crate::matrix::{
        d1::point::point_u16::MAX,
        d2::{point::point_u16::Point, rect::rect_u8},
    };

    #[test]
    fn rect() {
        assert_eq!(Rect::new((16, 32), (64, 128)), Rect { min: Point { row: 16, col: 32 }, max: Point { row: 64, col: 128 } });
        assert_eq!(Rect::largest(), Rect { min: Point { row: 0, col: 0 }, max: Point { row: MAX, col: MAX } });
        assert_eq!(Rect::min(), Rect { min: Point { row: 0, col: 0 }, max: Point { row: 0, col: 0 } });
        assert_eq!(Rect::max(), Rect { min: Point { row: MAX, col: MAX }, max: Point { row: MAX, col: MAX } });
        assert_eq!(Rect::new((1, 2), (4, 3)).iter_row().collect::<Vec<u16>>(), [1, 2, 3, 4]);
        assert_eq!(Rect::new((1, 2), (2, 5)).iter_col().collect::<Vec<u16>>(), [2, 3, 4, 5]);
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::new((16, 32), (64, 128)).to_string(), "((16, 32), (64, 128))");
        assert_eq!(Rect::largest().to_string(), "((0, 0), (65535, 65535))");
        assert_eq!(Rect::min().to_string(), "((0, 0), (0, 0))");
        assert_eq!(Rect::max().to_string(), "((65535, 65535), (65535, 65535))");
    }

    #[test]
    fn from() {
        assert_eq!(Rect::from(rect_u8::Rect::largest()), Rect { min: Point { row: 0, col: 0 }, max: Point { row: u8::MAX.into(), col: u8::MAX.into() } });
    }
}
