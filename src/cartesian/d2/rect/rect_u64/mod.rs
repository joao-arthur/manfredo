use crate::cartesian::d2::{
    point::point_u64,
    rect::{rect_u8, rect_u16, rect_u32},
};
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
pub use self::delta::{delta_max, delta_x, delta_y};
pub use self::inflate::{
    checked_inflate, checked_inflate_assign, saturating_inflate, saturating_inflate_assign, try_checked_inflate, try_checked_inflate_assign, try_saturating_inflate, try_saturating_inflate_assign,
    wrapping_inflate, wrapping_inflate_assign,
};
pub use self::len::{len_max, len_x, len_y};
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
    pub fn of(x1: u64, y1: u64, x2: u64, y2: u64) -> Self {
        Rect { min: point_u64::Point::of(x1, y1), max: point_u64::Point::of(x2, y2) }
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

    pub fn iter_x(&self) -> RangeInclusive<u64> {
        self.min.x..=self.max.x
    }

    pub fn iter_y(&self) -> RangeInclusive<u64> {
        self.min.y..=self.max.y
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

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

#[cfg(test)]
mod tests {
    use super::Rect;
    use crate::cartesian::d2::{
        point::point_u64::Point,
        rect::{rect_u8, rect_u16, rect_u32},
    };

    #[test]
    fn rect_u64() {
        assert_eq!(Rect::largest(), Rect { min: Point { x: 0, y: 0 }, max: Point { x: u64::MAX, y: u64::MAX } });
        assert_eq!(Rect::min(), Rect { min: Point { x: 0, y: 0 }, max: Point { x: 0, y: 0 } });
        assert_eq!(Rect::max(), Rect { min: Point { x: u64::MAX, y: u64::MAX }, max: Point { x: u64::MAX, y: u64::MAX } });
        assert_eq!(Rect::of(4096, 8192, 16384, 32768), Rect { min: Point { x: 4096, y: 8192 }, max: Point { x: 16384, y: 32768 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::of(4096, 8192, 16384, 32768).to_string(), "((4096, 8192), (16384, 32768))");
        assert_eq!(Rect::of(u64::MAX, 0, 0, u64::MAX).to_string(), "((18446744073709551615, 0), (0, 18446744073709551615))");
    }

    #[test]
    fn from() {
        assert_eq!(Rect::from(rect_u8::Rect::largest()), Rect { min: Point { x: 0, y: 0 }, max: Point { x: u8::MAX.into(), y: u8::MAX.into() } });
        assert_eq!(Rect::from(rect_u16::Rect::largest()), Rect { min: Point { x: 0, y: 0 }, max: Point { x: u16::MAX.into(), y: u16::MAX.into() } });
        assert_eq!(Rect::from(rect_u32::Rect::largest()), Rect { min: Point { x: 0, y: 0 }, max: Point { x: u32::MAX.into(), y: u32::MAX.into() } });
    }

    #[test]
    fn iter_x() {
        assert_eq!(Rect::of(3, 6, 2, 8).iter_x().collect::<Vec<u64>>(), []);
        assert_eq!(Rect::of(3, 6, 3, 8).iter_x().collect::<Vec<u64>>(), [3]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_x().collect::<Vec<u64>>(), [3, 4]);
        assert_eq!(Rect::of(3, 6, 5, 8).iter_x().collect::<Vec<u64>>(), [3, 4, 5]);
        assert_eq!(Rect::of(3, 6, 6, 8).iter_x().collect::<Vec<u64>>(), [3, 4, 5, 6]);
        assert_eq!(Rect::of(3, 6, 6, 8).iter_x().rev().collect::<Vec<u64>>(), [6, 5, 4, 3]);
        assert_eq!(Rect::of(3, 6, 5, 8).iter_x().rev().collect::<Vec<u64>>(), [5, 4, 3]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_x().rev().collect::<Vec<u64>>(), [4, 3]);
        assert_eq!(Rect::of(3, 6, 3, 8).iter_x().rev().collect::<Vec<u64>>(), [3]);
        assert_eq!(Rect::of(3, 6, 2, 8).iter_x().rev().collect::<Vec<u64>>(), []);
    }

    #[test]
    fn iter_y() {
        assert_eq!(Rect::of(3, 6, 4, 5).iter_y().collect::<Vec<u64>>(), []);
        assert_eq!(Rect::of(3, 6, 4, 6).iter_y().collect::<Vec<u64>>(), [6]);
        assert_eq!(Rect::of(3, 6, 4, 7).iter_y().collect::<Vec<u64>>(), [6, 7]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_y().collect::<Vec<u64>>(), [6, 7, 8]);
        assert_eq!(Rect::of(3, 6, 4, 9).iter_y().collect::<Vec<u64>>(), [6, 7, 8, 9]);
        assert_eq!(Rect::of(3, 6, 4, 9).iter_y().rev().collect::<Vec<u64>>(), [9, 8, 7, 6]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_y().rev().collect::<Vec<u64>>(), [8, 7, 6]);
        assert_eq!(Rect::of(3, 6, 4, 7).iter_y().rev().collect::<Vec<u64>>(), [7, 6]);
        assert_eq!(Rect::of(3, 6, 4, 6).iter_y().rev().collect::<Vec<u64>>(), [6]);
        assert_eq!(Rect::of(3, 6, 4, 5).iter_y().rev().collect::<Vec<u64>>(), []);
    }
}
