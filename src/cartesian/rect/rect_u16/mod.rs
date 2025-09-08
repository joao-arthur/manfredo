use crate::cartesian::{point::point_u16, rect::rect_u8::RectU8};
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
pub struct RectU16 {
    pub min: point_u16::PointU16,
    pub max: point_u16::PointU16,
}

impl RectU16 {
    pub fn of(x1: u16, y1: u16, x2: u16, y2: u16) -> Self {
        RectU16 { min: point_u16::PointU16::of(x1, y1), max: point_u16::PointU16::of(x2, y2) }
    }

    pub fn largest() -> Self {
        RectU16 { min: point_u16::PointU16::min(), max: point_u16::PointU16::max() }
    }

    pub fn min() -> Self {
        RectU16 { min: point_u16::PointU16::min(), max: point_u16::PointU16::min() }
    }

    pub fn max() -> Self {
        RectU16 { min: point_u16::PointU16::max(), max: point_u16::PointU16::max() }
    }

    pub fn iter_x(&self) -> RangeInclusive<u16> {
        self.min.x..=self.max.x
    }

    pub fn iter_y(&self) -> RangeInclusive<u16> {
        self.min.y..=self.max.y
    }
}

impl From<RectU8> for RectU16 {
    fn from(r: RectU8) -> Self {
        RectU16 { min: point_u16::PointU16::of(r.min.x.into(), r.min.y.into()), max: point_u16::PointU16::of(r.max.x.into(), r.max.y.into()) }
    }
}

impl std::fmt::Display for RectU16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_x(r: &RectU16) -> u16 {
    point_u16::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectU16) -> u16 {
    point_u16::delta_y(&r.min, &r.max)
}

pub fn max_delta(r: &RectU16) -> u16 {
    std::cmp::max(delta_x(r), delta_y(r))
}

pub fn len_x(r: &RectU16) -> u16 {
    delta_x(r) + 1
}

pub fn len_y(r: &RectU16) -> u16 {
    delta_y(r) + 1
}

pub fn max_len(r: &RectU16) -> u16 {
    std::cmp::max(len_x(r), len_y(r))
}

#[cfg(test)]
mod tests {
    use super::{RectU16, delta_x, delta_y, len_x, len_y, max_delta, max_len};
    use crate::cartesian::{point::point_u16::PointU16, rect::rect_u8::RectU8};

    #[test]
    fn rect_u16() {
        assert_eq!(RectU16::largest(), RectU16 { min: PointU16 { x: 0, y: 0 }, max: PointU16 { x: u16::MAX, y: u16::MAX } });
        assert_eq!(RectU16::min(), RectU16 { min: PointU16 { x: 0, y: 0 }, max: PointU16 { x: 0, y: 0 } });
        assert_eq!(RectU16::max(), RectU16 { min: PointU16 { x: u16::MAX, y: u16::MAX }, max: PointU16 { x: u16::MAX, y: u16::MAX } });
        assert_eq!(RectU16::of(16, 32, 16, 32), RectU16 { min: PointU16 { x: 16, y: 32 }, max: PointU16 { x: 16, y: 32 } });
    }

    #[test]
    fn to_string() {
        assert_eq!(RectU16::of(16, 32, 16, 32).to_string(), "((16, 32), (16, 32))");
        assert_eq!(RectU16::of(u16::MAX, 0, 0, u16::MAX).to_string(), "((65535, 0), (0, 65535))");
    }

    #[test]
    fn from() {
        assert_eq!(RectU16::from(RectU8::largest()), RectU16 { min: PointU16 { x: 0, y: 0 }, max: PointU16 { x: u8::MAX.into(), y: u8::MAX.into() } });
    }

    #[test]
    fn iter_x() {
        assert_eq!(RectU16::of(3, 6, 2, 8).iter_x().collect::<Vec<u16>>(), []);
        assert_eq!(RectU16::of(3, 6, 3, 8).iter_x().collect::<Vec<u16>>(), [3]);
        assert_eq!(RectU16::of(3, 6, 4, 8).iter_x().collect::<Vec<u16>>(), [3, 4]);
        assert_eq!(RectU16::of(3, 6, 5, 8).iter_x().collect::<Vec<u16>>(), [3, 4, 5]);
        assert_eq!(RectU16::of(3, 6, 6, 8).iter_x().collect::<Vec<u16>>(), [3, 4, 5, 6]);
        assert_eq!(RectU16::of(3, 6, 6, 8).iter_x().rev().collect::<Vec<u16>>(), [6, 5, 4, 3]);
        assert_eq!(RectU16::of(3, 6, 5, 8).iter_x().rev().collect::<Vec<u16>>(), [5, 4, 3]);
        assert_eq!(RectU16::of(3, 6, 4, 8).iter_x().rev().collect::<Vec<u16>>(), [4, 3]);
        assert_eq!(RectU16::of(3, 6, 3, 8).iter_x().rev().collect::<Vec<u16>>(), [3]);
        assert_eq!(RectU16::of(3, 6, 2, 8).iter_x().rev().collect::<Vec<u16>>(), []);
    }

    #[test]
    fn iter_y() {
        assert_eq!(RectU16::of(3, 6, 4, 5).iter_y().collect::<Vec<u16>>(), []);
        assert_eq!(RectU16::of(3, 6, 4, 6).iter_y().collect::<Vec<u16>>(), [6]);
        assert_eq!(RectU16::of(3, 6, 4, 7).iter_y().collect::<Vec<u16>>(), [6, 7]);
        assert_eq!(RectU16::of(3, 6, 4, 8).iter_y().collect::<Vec<u16>>(), [6, 7, 8]);
        assert_eq!(RectU16::of(3, 6, 4, 9).iter_y().collect::<Vec<u16>>(), [6, 7, 8, 9]);
        assert_eq!(RectU16::of(3, 6, 4, 9).iter_y().rev().collect::<Vec<u16>>(), [9, 8, 7, 6]);
        assert_eq!(RectU16::of(3, 6, 4, 8).iter_y().rev().collect::<Vec<u16>>(), [8, 7, 6]);
        assert_eq!(RectU16::of(3, 6, 4, 7).iter_y().rev().collect::<Vec<u16>>(), [7, 6]);
        assert_eq!(RectU16::of(3, 6, 4, 6).iter_y().rev().collect::<Vec<u16>>(), [6]);
        assert_eq!(RectU16::of(3, 6, 4, 5).iter_y().rev().collect::<Vec<u16>>(), []);
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectU16::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_x(&RectU16::of(0, 0, u16::MAX, 0)), u16::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectU16::of(0, 0, 0, 0)), 0);
        assert_eq!(delta_y(&RectU16::of(0, 0, 0, u16::MAX)), u16::MAX);
    }

    #[test]
    fn test_max_delta() {
        assert_eq!(max_delta(&RectU16::of(0, 5, 10, 10)), 10);
        assert_eq!(max_delta(&RectU16::of(5, 0, 9, 9)), 9);
    }

    #[test]
    fn max_delta_0() {
        assert_eq!(max_delta(&RectU16::of(0, 0, 0, 0)), 0);
        assert_eq!(max_delta(&RectU16::of(1, 1, 1, 1)), 0);
        assert_eq!(max_delta(&RectU16::of(5, 10, 5, 10)), 0);
    }

    #[test]
    fn max_delta_1() {
        assert_eq!(max_delta(&RectU16::of(0, 0, 1, 1)), 1);
        assert_eq!(max_delta(&RectU16::of(5, 5, 6, 6)), 1);
        assert_eq!(max_delta(&RectU16::of(0, 0, 0, 1)), 1);
        assert_eq!(max_delta(&RectU16::of(5, 9, 5, 10)), 1);
    }

    #[test]
    fn max_delta_bounds() {
        assert_eq!(max_delta(&RectU16::of(0, 0, u16::MAX, u16::MAX - 1)), u16::MAX);
        assert_eq!(max_delta(&RectU16::of(0, 0, u16::MAX - 1, u16::MAX)), u16::MAX);
    }

    #[test]
    fn test_len_x() {
        assert_eq!(len_x(&RectU16::of(0, 0, 0, 0)), 1);
        assert_eq!(len_x(&RectU16::of(0, 0, u16::MAX - 1, 0)), u16::MAX);
    }

    #[test]
    fn test_len_y() {
        assert_eq!(len_y(&RectU16::of(0, 0, 0, 0)), 1);
        assert_eq!(len_y(&RectU16::of(0, 0, 0, u16::MAX - 1)), u16::MAX);
    }

    #[test]
    fn test_max_len() {
        assert_eq!(max_len(&RectU16::of(0, 5, 10, 10)), 11);
        assert_eq!(max_len(&RectU16::of(5, 0, 9, 9)), 10);
    }

    #[test]
    fn max_len_1() {
        assert_eq!(max_len(&RectU16::of(0, 0, 0, 0)), 1);
        assert_eq!(max_len(&RectU16::of(1, 1, 1, 1)), 1);
        assert_eq!(max_len(&RectU16::of(5, 10, 5, 10)), 1);
    }

    #[test]
    fn max_len_2() {
        assert_eq!(max_len(&RectU16::of(0, 0, 1, 1)), 2);
        assert_eq!(max_len(&RectU16::of(5, 5, 6, 6)), 2);
        assert_eq!(max_len(&RectU16::of(0, 0, 0, 1)), 2);
        assert_eq!(max_len(&RectU16::of(5, 9, 5, 10)), 2);
    }

    #[test]
    fn max_len_bounds() {
        assert_eq!(max_len(&RectU16::of(1, 0, u16::MAX - 1, u16::MAX - 1)), u16::MAX);
        assert_eq!(max_len(&RectU16::of(0, 1, u16::MAX - 1, u16::MAX - 1)), u16::MAX);
        assert_eq!(max_len(&RectU16::of(0, 0, u16::MAX - 2, u16::MAX - 1)), u16::MAX);
        assert_eq!(max_len(&RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 2)), u16::MAX);
    }
}
