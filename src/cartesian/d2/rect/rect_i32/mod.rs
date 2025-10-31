use crate::cartesian::d2::{
    point::point_i32::Point,
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
pub use self::delta::{delta_max, delta_min, delta_x, delta_y};
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
    pub min: Point,
    pub max: Point,
}

impl Rect {
    pub fn of(min: (i32, i32), max: (i32, i32)) -> Self {
        Rect { min: Point { x: min.0, y: min.1 }, max: Point { x: max.0, y: max.1 } }
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

    pub fn zero() -> Self {
        Rect { min: Point::zero(), max: Point::zero() }
    }

    pub fn iter_x(&self) -> RangeInclusive<i32> {
        self.min.x..=self.max.x
    }

    pub fn iter_y(&self) -> RangeInclusive<i32> {
        self.min.y..=self.max.y
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

impl From<rect_i8::Rect> for Rect {
    fn from(r: rect_i8::Rect) -> Self {
        Rect { min: Point::from(r.min), max: Point::from(r.max) }
    }
}

impl From<rect_i16::Rect> for Rect {
    fn from(r: rect_i16::Rect) -> Self {
        Rect { min: Point::from(r.min), max: Point::from(r.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Rect;
    use crate::cartesian::{
        d1::point::point_i32::{MAX, MIN},
        d2::{
            point::point_i32::Point,
            rect::{rect_i8, rect_i16},
        },
    };

    #[test]
    fn rect() {
        assert_eq!(Rect::largest(), Rect { min: Point { x: MIN, y: MIN }, max: Point { x: MAX, y: MAX } });
        assert_eq!(Rect::min(), Rect { min: Point { x: MIN, y: MIN }, max: Point { x: MIN, y: MIN } });
        assert_eq!(Rect::max(), Rect { min: Point { x: MAX, y: MAX }, max: Point { x: MAX, y: MAX } });
        assert_eq!(Rect::zero(), Rect { min: Point { x: 0, y: 0 }, max: Point { x: 0, y: 0 } });
        assert_eq!(Rect::of((MIN, -1), (1, MAX)), Rect { min: Point { x: MIN, y: -1 }, max: Point { x: 1, y: MAX } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::largest().to_string(), "((-2147483648, -2147483648), (2147483647, 2147483647))");
        assert_eq!(Rect::min().to_string(), "((-2147483648, -2147483648), (-2147483648, -2147483648))");
        assert_eq!(Rect::max().to_string(), "((2147483647, 2147483647), (2147483647, 2147483647))");
        assert_eq!(Rect::zero().to_string(), "((0, 0), (0, 0))");
        assert_eq!(Rect::of((MIN, -0), (0, MAX)).to_string(), "((-2147483648, 0), (0, 2147483647))");
    }

    #[test]
    fn from() {
        assert_eq!(Rect::from(rect_i8::Rect::largest()), Rect { min: Point { x: i8::MIN.into(), y: i8::MIN.into() }, max: Point { x: i8::MAX.into(), y: i8::MAX.into() } });
        assert_eq!(Rect::from(rect_i16::Rect::largest()), Rect { min: Point { x: i16::MIN.into(), y: i16::MIN.into() }, max: Point { x: i16::MAX.into(), y: i16::MAX.into() } });
    }

    #[test]
    fn iter_x() {
        assert_eq!(Rect::of((-6, -8), (-7, -6)).iter_x().collect::<Vec<i32>>(), []);
        assert_eq!(Rect::of((-6, -8), (-6, -6)).iter_x().collect::<Vec<i32>>(), [-6]);
        assert_eq!(Rect::of((-6, -8), (-5, -6)).iter_x().collect::<Vec<i32>>(), [-6, -5]);
        assert_eq!(Rect::of((-6, -8), (-4, -6)).iter_x().collect::<Vec<i32>>(), [-6, -5, -4]);
        assert_eq!(Rect::of((-6, -8), (-3, -6)).iter_x().collect::<Vec<i32>>(), [-6, -5, -4, -3]);
        assert_eq!(Rect::of((-6, -8), (-3, -6)).iter_x().rev().collect::<Vec<i32>>(), [-3, -4, -5, -6]);
        assert_eq!(Rect::of((-6, -8), (-4, -6)).iter_x().rev().collect::<Vec<i32>>(), [-4, -5, -6]);
        assert_eq!(Rect::of((-6, -8), (-5, -6)).iter_x().rev().collect::<Vec<i32>>(), [-5, -6]);
        assert_eq!(Rect::of((-6, -8), (-6, -6)).iter_x().rev().collect::<Vec<i32>>(), [-6]);
        assert_eq!(Rect::of((-6, -8), (-7, -6)).iter_x().rev().collect::<Vec<i32>>(), []);
    }

    #[test]
    fn iter_y() {
        assert_eq!(Rect::of((-6, -8), (-4, -9)).iter_y().collect::<Vec<i32>>(), []);
        assert_eq!(Rect::of((-6, -8), (-4, -8)).iter_y().collect::<Vec<i32>>(), [-8]);
        assert_eq!(Rect::of((-6, -8), (-4, -7)).iter_y().collect::<Vec<i32>>(), [-8, -7]);
        assert_eq!(Rect::of((-6, -8), (-4, -6)).iter_y().collect::<Vec<i32>>(), [-8, -7, -6]);
        assert_eq!(Rect::of((-6, -8), (-4, -5)).iter_y().collect::<Vec<i32>>(), [-8, -7, -6, -5]);
        assert_eq!(Rect::of((-6, -8), (-4, -5)).iter_y().rev().collect::<Vec<i32>>(), [-5, -6, -7, -8]);
        assert_eq!(Rect::of((-6, -8), (-4, -6)).iter_y().rev().collect::<Vec<i32>>(), [-6, -7, -8]);
        assert_eq!(Rect::of((-6, -8), (-4, -7)).iter_y().rev().collect::<Vec<i32>>(), [-7, -8]);
        assert_eq!(Rect::of((-6, -8), (-4, -8)).iter_y().rev().collect::<Vec<i32>>(), [-8]);
        assert_eq!(Rect::of((-6, -8), (-4, -9)).iter_y().rev().collect::<Vec<i32>>(), []);
    }
}
