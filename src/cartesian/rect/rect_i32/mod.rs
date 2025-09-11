use crate::cartesian::{
    point::point_i32,
    rect::{rect_i8, rect_i16},
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
    pub min: point_i32::Point,
    pub max: point_i32::Point,
}

impl Rect {
    pub fn of(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Rect { min: point_i32::Point::of(x1, y1), max: point_i32::Point::of(x2, y2) }
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

    pub fn iter_x(&self) -> RangeInclusive<i32> {
        self.min.x..=self.max.x
    }

    pub fn iter_y(&self) -> RangeInclusive<i32> {
        self.min.y..=self.max.y
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

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_x(r: &Rect) -> u32 {
    point_i32::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &Rect) -> u32 {
    point_i32::delta_y(&r.min, &r.max)
}

pub fn max_delta(r: &Rect) -> u32 {
    std::cmp::max(delta_x(r), delta_y(r))
}

pub fn len_x(r: &Rect) -> u32 {
    delta_x(r) + 1
}

pub fn len_y(r: &Rect) -> u32 {
    delta_y(r) + 1
}

pub fn max_len(r: &Rect) -> u32 {
    std::cmp::max(len_x(r), len_y(r))
}

#[cfg(test)]
mod tests {
    use super::{Rect, delta_x, delta_y, len_x, len_y, max_delta, max_len};
    use crate::cartesian::{
        point::point_i32::Point,
        rect::{rect_i8, rect_i16},
    };

    #[test]
    fn rect_i32() {
        assert_eq!(Rect::largest(), Rect { min: Point { x: i32::MIN, y: i32::MIN }, max: Point { x: i32::MAX, y: i32::MAX } });
        assert_eq!(Rect::min(), Rect { min: Point { x: i32::MIN, y: i32::MIN }, max: Point { x: i32::MIN, y: i32::MIN } });
        assert_eq!(Rect::max(), Rect { min: Point { x: i32::MAX, y: i32::MAX }, max: Point { x: i32::MAX, y: i32::MAX } });
        assert_eq!(Rect::of(i32::MIN, -1, 1, i32::MAX), Rect { min: Point { x: i32::MIN, y: -1 }, max: Point { x: 1, y: i32::MAX } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::largest().to_string(), "((-2147483648, -2147483648), (2147483647, 2147483647))");
        assert_eq!(Rect::of(i32::MIN, -0, 0, i32::MAX).to_string(), "((-2147483648, 0), (0, 2147483647))");
    }

    #[test]
    fn from() {
        assert_eq!(Rect::from(rect_i8::Rect::largest()), Rect { min: Point { x: i8::MIN.into(), y: i8::MIN.into() }, max: Point { x: i8::MAX.into(), y: i8::MAX.into() } });
        assert_eq!(Rect::from(rect_i16::Rect::largest()), Rect { min: Point { x: i16::MIN.into(), y: i16::MIN.into() }, max: Point { x: i16::MAX.into(), y: i16::MAX.into() } });
    }

    #[test]
    fn iter_x() {
        assert_eq!(Rect::of(-6, -8, -7, -6).iter_x().collect::<Vec<i32>>(), []);
        assert_eq!(Rect::of(-6, -8, -6, -6).iter_x().collect::<Vec<i32>>(), [-6]);
        assert_eq!(Rect::of(-6, -8, -5, -6).iter_x().collect::<Vec<i32>>(), [-6, -5]);
        assert_eq!(Rect::of(-6, -8, -4, -6).iter_x().collect::<Vec<i32>>(), [-6, -5, -4]);
        assert_eq!(Rect::of(-6, -8, -3, -6).iter_x().collect::<Vec<i32>>(), [-6, -5, -4, -3]);
        assert_eq!(Rect::of(-6, -8, -3, -6).iter_x().rev().collect::<Vec<i32>>(), [-3, -4, -5, -6]);
        assert_eq!(Rect::of(-6, -8, -4, -6).iter_x().rev().collect::<Vec<i32>>(), [-4, -5, -6]);
        assert_eq!(Rect::of(-6, -8, -5, -6).iter_x().rev().collect::<Vec<i32>>(), [-5, -6]);
        assert_eq!(Rect::of(-6, -8, -6, -6).iter_x().rev().collect::<Vec<i32>>(), [-6]);
        assert_eq!(Rect::of(-6, -8, -7, -6).iter_x().rev().collect::<Vec<i32>>(), []);
    }

    #[test]
    fn iter_y() {
        assert_eq!(Rect::of(-6, -8, -4, -9).iter_y().collect::<Vec<i32>>(), []);
        assert_eq!(Rect::of(-6, -8, -4, -8).iter_y().collect::<Vec<i32>>(), [-8]);
        assert_eq!(Rect::of(-6, -8, -4, -7).iter_y().collect::<Vec<i32>>(), [-8, -7]);
        assert_eq!(Rect::of(-6, -8, -4, -6).iter_y().collect::<Vec<i32>>(), [-8, -7, -6]);
        assert_eq!(Rect::of(-6, -8, -4, -5).iter_y().collect::<Vec<i32>>(), [-8, -7, -6, -5]);
        assert_eq!(Rect::of(-6, -8, -4, -5).iter_y().rev().collect::<Vec<i32>>(), [-5, -6, -7, -8]);
        assert_eq!(Rect::of(-6, -8, -4, -6).iter_y().rev().collect::<Vec<i32>>(), [-6, -7, -8]);
        assert_eq!(Rect::of(-6, -8, -4, -7).iter_y().rev().collect::<Vec<i32>>(), [-7, -8]);
        assert_eq!(Rect::of(-6, -8, -4, -8).iter_y().rev().collect::<Vec<i32>>(), [-8]);
        assert_eq!(Rect::of(-6, -8, -4, -9).iter_y().rev().collect::<Vec<i32>>(), []);
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Rect::of(0, i32::MIN, 0, i32::MAX)), 0);
        assert_eq!(delta_x(&Rect::of(i32::MIN, 0, i32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&Rect::of(i32::MIN, 0, i32::MAX, 0)), 0);
        assert_eq!(delta_y(&Rect::of(0, i32::MIN, 0, i32::MAX)), u32::MAX);
    }

    #[test]
    fn test_max_delta() {
        assert_eq!(max_delta(&Rect::of(0, 5, 10, 10)), 10);
        assert_eq!(max_delta(&Rect::of(-10, -10, -5, 0)), 10);
        assert_eq!(max_delta(&Rect::of(-5, 0, 5, 5)), 10);
    }

    #[test]
    fn max_delta_0() {
        assert_eq!(max_delta(&Rect::of(0, 0, 0, 0)), 0);
        assert_eq!(max_delta(&Rect::of(1, 1, 1, 1)), 0);
        assert_eq!(max_delta(&Rect::of(-1, -1, -1, -1)), 0);
        assert_eq!(max_delta(&Rect::of(5, 10, 5, 10)), 0);
    }

    #[test]
    fn max_delta_1() {
        assert_eq!(max_delta(&Rect::of(0, 0, 1, 1)), 1);
        assert_eq!(max_delta(&Rect::of(5, 5, 6, 6)), 1);
        assert_eq!(max_delta(&Rect::of(-6, -6, -5, -5)), 1);
        assert_eq!(max_delta(&Rect::of(0, 0, 0, 1)), 1);
        assert_eq!(max_delta(&Rect::of(5, 9, 5, 10)), 1);
    }

    #[test]
    fn max_delta_bounds() {
        assert_eq!(max_delta(&Rect::of(i32::MIN + 1, i32::MIN, i32::MAX, i32::MAX)), u32::MAX);
        assert_eq!(max_delta(&Rect::of(i32::MIN, i32::MIN + 1, i32::MAX, i32::MAX)), u32::MAX);
        assert_eq!(max_delta(&Rect::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX)), u32::MAX);
        assert_eq!(max_delta(&Rect::of(i32::MIN, i32::MIN, i32::MAX, i32::MAX - 1)), u32::MAX);
    }

    #[test]
    fn test_len_x() {
        assert_eq!(len_x(&Rect::of(0, i32::MIN, 0, i32::MAX)), 1);
        assert_eq!(len_x(&Rect::of(i32::MIN, 0, i32::MAX - 1, 0)), u32::MAX);
    }

    #[test]
    fn test_len_y() {
        assert_eq!(len_y(&Rect::of(i32::MIN, 0, i32::MAX, 0)), 1);
        assert_eq!(len_y(&Rect::of(0, i32::MIN, 0, i32::MAX - 1)), u32::MAX);
    }

    #[test]
    fn test_max_len() {
        assert_eq!(max_len(&Rect::of(0, 5, 10, 10)), 11);
        assert_eq!(max_len(&Rect::of(-10, -10, -5, 0)), 11);
        assert_eq!(max_len(&Rect::of(-5, 0, 5, 5)), 11);
    }

    #[test]
    fn max_len_1() {
        assert_eq!(max_len(&Rect::of(0, 0, 0, 0)), 1);
        assert_eq!(max_len(&Rect::of(1, 1, 1, 1)), 1);
        assert_eq!(max_len(&Rect::of(-1, -1, -1, -1)), 1);
        assert_eq!(max_len(&Rect::of(5, 10, 5, 10)), 1);
    }

    #[test]
    fn max_len_2() {
        assert_eq!(max_len(&Rect::of(0, 0, 1, 1)), 2);
        assert_eq!(max_len(&Rect::of(5, 5, 6, 6)), 2);
        assert_eq!(max_len(&Rect::of(-6, -6, -5, -5)), 2);
        assert_eq!(max_len(&Rect::of(0, 0, 0, 1)), 2);
        assert_eq!(max_len(&Rect::of(5, 9, 5, 10)), 2);
    }

    #[test]
    fn max_len_bounds() {
        assert_eq!(max_len(&Rect::of(i32::MIN + 1, i32::MIN, i32::MAX - 1, i32::MAX - 1)), u32::MAX);
        assert_eq!(max_len(&Rect::of(i32::MIN, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1)), u32::MAX);
        assert_eq!(max_len(&Rect::of(i32::MIN, i32::MIN, i32::MAX - 2, i32::MAX - 1)), u32::MAX);
        assert_eq!(max_len(&Rect::of(i32::MIN, i32::MIN, i32::MAX - 1, i32::MAX - 2)), u32::MAX);
    }
}
