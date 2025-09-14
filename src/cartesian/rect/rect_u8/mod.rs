use crate::cartesian::point::point_u8;
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
pub use self::deflate::{deflate_assign, deflate, try_deflate_assign, try_deflate};
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
    pub min: point_u8::Point,
    pub max: point_u8::Point,
}

impl Rect {
    pub fn of(x1: u8, y1: u8, x2: u8, y2: u8) -> Self {
        Rect { min: point_u8::Point::of(x1, y1), max: point_u8::Point::of(x2, y2) }
    }

    pub fn largest() -> Self {
        Rect { min: point_u8::Point::min(), max: point_u8::Point::max() }
    }

    pub fn min() -> Self {
        Rect { min: point_u8::Point::min(), max: point_u8::Point::min() }
    }

    pub fn max() -> Self {
        Rect { min: point_u8::Point::max(), max: point_u8::Point::max() }
    }

    pub fn iter_x(&self) -> RangeInclusive<u8> {
        self.min.x..=self.max.x
    }

    pub fn iter_y(&self) -> RangeInclusive<u8> {
        self.min.y..=self.max.y
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_x(r: &Rect) -> u8 {
    point_u8::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &Rect) -> u8 {
    point_u8::delta_y(&r.min, &r.max)
}

pub fn max_delta(r: &Rect) -> u8 {
    std::cmp::max(delta_x(r), delta_y(r))
}

pub fn len_x(r: &Rect) -> u8 {
    delta_x(r) + 1
}

pub fn len_y(r: &Rect) -> u8 {
    delta_y(r) + 1
}

pub fn max_len(r: &Rect) -> u8 {
    std::cmp::max(len_x(r), len_y(r))
}

#[cfg(test)]
mod tests {
    use super::{Rect, delta_x, delta_y, len_x, len_y, max_delta, max_len};
    use crate::cartesian::point::point_u8::Point;

    #[test]
    fn rect_u8() {
        assert_eq!(Rect::largest(), Rect { min: Point { x: 0, y: 0 }, max: Point { x: u8::MAX, y: u8::MAX } });
        assert_eq!(Rect::min(), Rect { min: Point { x: 0, y: 0 }, max: Point { x: 0, y: 0 } });
        assert_eq!(Rect::max(), Rect { min: Point { x: u8::MAX, y: u8::MAX }, max: Point { x: u8::MAX, y: u8::MAX } });
        assert_eq!(Rect::of(0, 2, 4, 8), Rect { min: Point { x: 0, y: 2 }, max: Point { x: 4, y: 8 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::largest().to_string(), "((0, 0), (255, 255))");
        assert_eq!(Rect::of(0, 2, 4, 8).to_string(), "((0, 2), (4, 8))");
    }

    #[test]
    fn iter_x() {
        assert_eq!(Rect::of(3, 6, 2, 8).iter_x().collect::<Vec<u8>>(), []);
        assert_eq!(Rect::of(3, 6, 3, 8).iter_x().collect::<Vec<u8>>(), [3]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_x().collect::<Vec<u8>>(), [3, 4]);
        assert_eq!(Rect::of(3, 6, 5, 8).iter_x().collect::<Vec<u8>>(), [3, 4, 5]);
        assert_eq!(Rect::of(3, 6, 6, 8).iter_x().collect::<Vec<u8>>(), [3, 4, 5, 6]);
        assert_eq!(Rect::of(3, 6, 6, 8).iter_x().rev().collect::<Vec<u8>>(), [6, 5, 4, 3]);
        assert_eq!(Rect::of(3, 6, 5, 8).iter_x().rev().collect::<Vec<u8>>(), [5, 4, 3]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_x().rev().collect::<Vec<u8>>(), [4, 3]);
        assert_eq!(Rect::of(3, 6, 3, 8).iter_x().rev().collect::<Vec<u8>>(), [3]);
        assert_eq!(Rect::of(3, 6, 2, 8).iter_x().rev().collect::<Vec<u8>>(), []);
    }

    #[test]
    fn iter_y() {
        assert_eq!(Rect::of(3, 6, 4, 5).iter_y().collect::<Vec<u8>>(), []);
        assert_eq!(Rect::of(3, 6, 4, 6).iter_y().collect::<Vec<u8>>(), [6]);
        assert_eq!(Rect::of(3, 6, 4, 7).iter_y().collect::<Vec<u8>>(), [6, 7]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_y().collect::<Vec<u8>>(), [6, 7, 8]);
        assert_eq!(Rect::of(3, 6, 4, 9).iter_y().collect::<Vec<u8>>(), [6, 7, 8, 9]);
        assert_eq!(Rect::of(3, 6, 4, 9).iter_y().rev().collect::<Vec<u8>>(), [9, 8, 7, 6]);
        assert_eq!(Rect::of(3, 6, 4, 8).iter_y().rev().collect::<Vec<u8>>(), [8, 7, 6]);
        assert_eq!(Rect::of(3, 6, 4, 7).iter_y().rev().collect::<Vec<u8>>(), [7, 6]);
        assert_eq!(Rect::of(3, 6, 4, 6).iter_y().rev().collect::<Vec<u8>>(), [6]);
        assert_eq!(Rect::of(3, 6, 4, 5).iter_y().rev().collect::<Vec<u8>>(), []);
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Rect::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_x(&Rect::of(0, 0, u8::MAX, 0)), u8::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&Rect::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_y(&Rect::of(0, 0, 0, u8::MAX)), u8::MAX);
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
    fn test_len_x() {
        assert_eq!(len_x(&Rect::of(0, 0, 0, 0)), 1);
        assert_eq!(len_x(&Rect::of(0, 0, u8::MAX - 1, 0)), u8::MAX);
    }

    #[test]
    fn test_len_y() {
        assert_eq!(len_y(&Rect::of(0, 0, 0, 0)), 1);
        assert_eq!(len_y(&Rect::of(0, 0, 0, u8::MAX - 1)), u8::MAX);
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
