use crate::cartesian::d2::{point::point_f64::Point, rect::rect_f32};

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
};
pub use self::len::{len_max, len_x, len_y};
pub use self::resize::{
    checked_resize, checked_resize_assign, saturating_resize, saturating_resize_assign, try_checked_resize, try_checked_resize_assign, try_saturating_resize, try_saturating_resize_assign,
};
pub use self::translate::{
    checked_translate, checked_translate_assign, saturating_translate, saturating_translate_assign, try_checked_translate, try_checked_translate_assign, wrapping_translate, wrapping_translate_assign,
};

#[derive(PartialEq, Debug, Clone)]
pub struct Rect {
    pub min: Point,
    pub max: Point,
}

pub struct RectF64Iterator {
    begin: f64,
    end: f64,
}

impl Iterator for RectF64Iterator {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.begin > self.end {
            return None;
        }
        let result = self.begin;
        self.begin += 1.0;
        Some(result)
    }
}

impl DoubleEndedIterator for RectF64Iterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.begin > self.end {
            return None;
        }
        let result = self.end;
        self.end -= 1.0;
        Some(result)
    }
}

impl Rect {
    pub fn of(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Rect { min: Point { x: x1, y: y1 }, max: Point { x: x2, y: y2 } }
    }

    pub fn largest() -> Self {
        Rect { min: Point::min(), max: Point::max() }
    }

    pub fn largest_min() -> Self {
        Rect { min: Point::min(), max: Point::zero() }
    }

    pub fn largest_max() -> Self {
        Rect { min: Point::zero(), max: Point::max() }
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

    pub fn iter_x(&self) -> RectF64Iterator {
        RectF64Iterator { begin: self.min.x, end: self.max.x }
    }

    pub fn iter_y(&self) -> RectF64Iterator {
        RectF64Iterator { begin: self.min.y, end: self.max.y }
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

impl From<rect_f32::Rect> for Rect {
    fn from(r: rect_f32::Rect) -> Self {
        Rect { min: Point::from(r.min), max: Point::from(r.max) }
    }
}

#[cfg(test)]
mod tests {
    use super::Rect;
    use crate::cartesian::{
        d1::point::{
            point_f32,
            point_f64::{MAX, MIN},
        },
        d2::{point::point_f64::Point, rect::rect_f32},
    };

    #[test]
    fn rect() {
        assert_eq!(Rect::largest(), Rect { min: Point { x: MIN, y: MIN }, max: Point { x: MAX, y: MAX } });
        assert_eq!(Rect::largest_min(), Rect { min: Point { x: MIN, y: MIN }, max: Point { x: 0.0, y: 0.0 } });
        assert_eq!(Rect::largest_max(), Rect { min: Point { x: 0.0, y: 0.0 }, max: Point { x: MAX, y: MAX } });
        assert_eq!(Rect::min(), Rect { min: Point { x: MIN, y: MIN }, max: Point { x: MIN, y: MIN } });
        assert_eq!(Rect::max(), Rect { min: Point { x: MAX, y: MAX }, max: Point { x: MAX, y: MAX } });
        assert_eq!(Rect::zero(), Rect { min: Point { x: 0.0, y: 0.0 }, max: Point { x: 0.0, y: 0.0 } });
        assert_eq!(Rect::of(MIN, -0.0, 0.0, MAX), Rect { min: Point { x: MIN, y: -0.0 }, max: Point { x: 0.0, y: MAX } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::largest().to_string(), "((-9007199254740992, -9007199254740992), (9007199254740991, 9007199254740991))");
        assert_eq!(Rect::largest_min().to_string(), "((-9007199254740992, -9007199254740992), (0, 0))");
        assert_eq!(Rect::largest_max().to_string(), "((0, 0), (9007199254740991, 9007199254740991))");
        assert_eq!(Rect::min().to_string(), "((-9007199254740992, -9007199254740992), (-9007199254740992, -9007199254740992))");
        assert_eq!(Rect::max().to_string(), "((9007199254740991, 9007199254740991), (9007199254740991, 9007199254740991))");
        assert_eq!(Rect::zero().to_string(), "((0, 0), (0, 0))");
        assert_eq!(Rect::of(MIN, -0.0, 0.0, MAX).to_string(), "((-9007199254740992, -0), (0, 9007199254740991))");
    }

    #[test]
    fn from() {
        assert_eq!(
            Rect::from(rect_f32::Rect::largest()),
            Rect { min: Point { x: point_f32::MIN.into(), y: point_f32::MIN.into() }, max: Point { x: point_f32::MAX.into(), y: point_f32::MAX.into() } }
        );
    }

    #[test]
    fn iter_x() {
        assert_eq!(Rect::of(-6.0, -8.0, -7.0, -6.0).iter_x().collect::<Vec<f64>>(), []);
        assert_eq!(Rect::of(-6.0, -8.0, -6.0, -6.0).iter_x().collect::<Vec<f64>>(), [-6.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -5.0, -6.0).iter_x().collect::<Vec<f64>>(), [-6.0, -5.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -6.0).iter_x().collect::<Vec<f64>>(), [-6.0, -5.0, -4.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -3.0, -6.0).iter_x().collect::<Vec<f64>>(), [-6.0, -5.0, -4.0, -3.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -3.0, -6.0).iter_x().rev().collect::<Vec<f64>>(), [-3.0, -4.0, -5.0, -6.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -6.0).iter_x().rev().collect::<Vec<f64>>(), [-4.0, -5.0, -6.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -5.0, -6.0).iter_x().rev().collect::<Vec<f64>>(), [-5.0, -6.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -6.0, -6.0).iter_x().rev().collect::<Vec<f64>>(), [-6.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -7.0, -6.0).iter_x().rev().collect::<Vec<f64>>(), []);
    }

    #[test]
    fn iter_y() {
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -9.0).iter_y().collect::<Vec<f64>>(), []);
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -8.0).iter_y().collect::<Vec<f64>>(), [-8.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -7.0).iter_y().collect::<Vec<f64>>(), [-8.0, -7.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -6.0).iter_y().collect::<Vec<f64>>(), [-8.0, -7.0, -6.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -5.0).iter_y().collect::<Vec<f64>>(), [-8.0, -7.0, -6.0, -5.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -5.0).iter_y().rev().collect::<Vec<f64>>(), [-5.0, -6.0, -7.0, -8.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -6.0).iter_y().rev().collect::<Vec<f64>>(), [-6.0, -7.0, -8.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -7.0).iter_y().rev().collect::<Vec<f64>>(), [-7.0, -8.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -8.0).iter_y().rev().collect::<Vec<f64>>(), [-8.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -9.0).iter_y().rev().collect::<Vec<f64>>(), []);
    }
}
