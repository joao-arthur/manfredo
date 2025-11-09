use crate::cartesian::d2::point::point_u8::Point;
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
    pub fn new(min: (u8, u8), max: (u8, u8)) -> Self {
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

#[cfg(test)]
mod tests {
    use super::Rect;
    use crate::cartesian::{d1::point::point_u8::MAX, d2::point::point_u8::Point};

    #[test]
    fn rect() {
        assert_eq!(Rect::new((0, 2), (4, 8)), Rect { min: Point { x: 0, y: 2 }, max: Point { x: 4, y: 8 } });
        assert_eq!(Rect::largest(), Rect { min: Point { x: 0, y: 0 }, max: Point { x: MAX, y: MAX } });
        assert_eq!(Rect::min(), Rect { min: Point { x: 0, y: 0 }, max: Point { x: 0, y: 0 } });
        assert_eq!(Rect::max(), Rect { min: Point { x: MAX, y: MAX }, max: Point { x: MAX, y: MAX } });
        assert_eq!(Rect::new((1, 2), (4, 3)).iter_x().collect::<Vec<u8>>(), [1, 2, 3, 4]);
        assert_eq!(Rect::new((1, 2), (2, 5)).iter_y().collect::<Vec<u8>>(), [2, 3, 4, 5]);
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::new((0, 2), (4, 8)).to_string(), "((0, 2), (4, 8))");
        assert_eq!(Rect::largest().to_string(), "((0, 0), (255, 255))");
        assert_eq!(Rect::min().to_string(), "((0, 0), (0, 0))");
        assert_eq!(Rect::max().to_string(), "((255, 255), (255, 255))");
    }
}
