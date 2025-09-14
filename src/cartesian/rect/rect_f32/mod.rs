use crate::cartesian::point::point_f32;

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
};
pub use self::resize::{
    checked_resize, checked_resize_assign, saturating_resize, saturating_resize_assign, try_checked_resize, try_checked_resize_assign, try_saturating_resize, try_saturating_resize_assign,
};
pub use self::translate::{
    checked_translate, checked_translate_assign, saturating_translate, saturating_translate_assign, try_checked_translate, try_checked_translate_assign, wrapping_translate, wrapping_translate_assign,
};

#[derive(PartialEq, Debug, Clone)]
pub struct Rect {
    pub min: point_f32::Point,
    pub max: point_f32::Point,
}

pub struct RectF32Iterator {
    begin: f32,
    end: f32,
}

impl Iterator for RectF32Iterator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.begin > self.end {
            return None;
        }
        let result = self.begin;
        self.begin += 1.0;
        Some(result)
    }
}

impl DoubleEndedIterator for RectF32Iterator {
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
    pub fn of(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Rect { min: point_f32::Point { x: x1, y: y1 }, max: point_f32::Point { x: x2, y: y2 } }
    }

    pub fn largest() -> Self {
        Rect { min: point_f32::Point::min(), max: point_f32::Point::max() }
    }

    pub fn largest_min() -> Self {
        Rect { min: point_f32::Point::min(), max: point_f32::Point::of(0.0, 0.0) }
    }

    pub fn largest_max() -> Self {
        Rect { min: point_f32::Point::of(0.0, 0.0), max: point_f32::Point::max() }
    }

    pub fn min() -> Self {
        Rect { min: point_f32::Point::min(), max: point_f32::Point::min() }
    }

    pub fn max() -> Self {
        Rect { min: point_f32::Point::max(), max: point_f32::Point::max() }
    }

    pub fn iter_x(&self) -> RectF32Iterator {
        RectF32Iterator { begin: self.min.x, end: self.max.x }
    }

    pub fn iter_y(&self) -> RectF32Iterator {
        RectF32Iterator { begin: self.min.y, end: self.max.y }
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_x(r: &Rect) -> f32 {
    point_f32::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &Rect) -> f32 {
    point_f32::delta_y(&r.min, &r.max)
}

pub fn max_delta(r: &Rect) -> f32 {
    delta_x(r).max(delta_y(r))
}

pub fn len_x(r: &Rect) -> f32 {
    delta_x(r) + 1.0
}

pub fn len_y(r: &Rect) -> f32 {
    delta_y(r) + 1.0
}

pub fn max_len(r: &Rect) -> f32 {
    len_x(r).max(len_y(r))
}

#[cfg(test)]
mod tests {
    use super::{Rect, delta_x, delta_y, len_x, len_y, max_delta, max_len};
    use crate::cartesian::point::point_f32::{MAX, MIN, Point};

    #[test]
    fn rect_f32() {
        assert_eq!(Rect::largest(), Rect { min: Point { x: MIN, y: MIN }, max: Point { x: MAX, y: MAX } });
        assert_eq!(Rect::largest_min(), Rect { min: Point { x: MIN, y: MIN }, max: Point { x: 0.0, y: 0.0 } });
        assert_eq!(Rect::largest_max(), Rect { min: Point { x: 0.0, y: 0.0 }, max: Point { x: MAX, y: MAX } });
        assert_eq!(Rect::min(), Rect { min: Point { x: MIN, y: MIN }, max: Point { x: MIN, y: MIN } });
        assert_eq!(Rect::max(), Rect { min: Point { x: MAX, y: MAX }, max: Point { x: MAX, y: MAX } });
        assert_eq!(Rect::of(MIN, -0.0, 0.0, MAX), Rect { min: Point { x: MIN, y: -0.0 }, max: Point { x: 0.0, y: MAX } });
    }

    #[test]
    fn to_string() {
        assert_eq!(Rect::largest().to_string(), "((-16777216, -16777216), (16777215, 16777215))");
        assert_eq!(Rect::largest_min().to_string(), "((-16777216, -16777216), (0, 0))");
        assert_eq!(Rect::largest_max().to_string(), "((0, 0), (16777215, 16777215))");
        assert_eq!(Rect::of(MIN, -0.0, 0.0, MAX).to_string(), "((-16777216, -0), (0, 16777215))");
    }

    #[test]
    fn iter_x() {
        assert_eq!(Rect::of(-6.0, -8.0, -7.0, -6.0).iter_x().collect::<Vec<f32>>(), []);
        assert_eq!(Rect::of(-6.0, -8.0, -6.0, -6.0).iter_x().collect::<Vec<f32>>(), [-6.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -5.0, -6.0).iter_x().collect::<Vec<f32>>(), [-6.0, -5.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -6.0).iter_x().collect::<Vec<f32>>(), [-6.0, -5.0, -4.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -3.0, -6.0).iter_x().collect::<Vec<f32>>(), [-6.0, -5.0, -4.0, -3.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -3.0, -6.0).iter_x().rev().collect::<Vec<f32>>(), [-3.0, -4.0, -5.0, -6.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -6.0).iter_x().rev().collect::<Vec<f32>>(), [-4.0, -5.0, -6.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -5.0, -6.0).iter_x().rev().collect::<Vec<f32>>(), [-5.0, -6.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -6.0, -6.0).iter_x().rev().collect::<Vec<f32>>(), [-6.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -7.0, -6.0).iter_x().rev().collect::<Vec<f32>>(), []);
    }

    #[test]
    fn iter_y() {
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -9.0).iter_y().collect::<Vec<f32>>(), []);
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -8.0).iter_y().collect::<Vec<f32>>(), [-8.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -7.0).iter_y().collect::<Vec<f32>>(), [-8.0, -7.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -6.0).iter_y().collect::<Vec<f32>>(), [-8.0, -7.0, -6.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -5.0).iter_y().collect::<Vec<f32>>(), [-8.0, -7.0, -6.0, -5.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -5.0).iter_y().rev().collect::<Vec<f32>>(), [-5.0, -6.0, -7.0, -8.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -6.0).iter_y().rev().collect::<Vec<f32>>(), [-6.0, -7.0, -8.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -7.0).iter_y().rev().collect::<Vec<f32>>(), [-7.0, -8.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -8.0).iter_y().rev().collect::<Vec<f32>>(), [-8.0]);
        assert_eq!(Rect::of(-6.0, -8.0, -4.0, -9.0).iter_y().rev().collect::<Vec<f32>>(), []);
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Rect::of(0.0, 0.0, 0.0, MAX)), 0.0);
        assert_eq!(delta_x(&Rect::of(0.0, -8_388_608.0, 0.0, 8_388_607.0)), 0.0);
        assert_eq!(delta_x(&Rect::of(0.0, 0.0, MAX, 0.0)), MAX);
        assert_eq!(delta_x(&Rect::of(-8_388_608.0, 0.0, 8_388_607.0, 0.0)), MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&Rect::of(0.0, 0.0, MAX, 0.0)), 0.0);
        assert_eq!(delta_y(&Rect::of(-8_388_608.0, 0.0, 8_388_607.0, 0.0)), 0.0);
        assert_eq!(delta_y(&Rect::of(0.0, 0.0, 0.0, MAX)), MAX);
        assert_eq!(delta_y(&Rect::of(0.0, -8_388_608.0, 0.0, 8_388_607.0)), MAX);
    }

    #[test]
    fn test_max_delta() {
        assert_eq!(max_delta(&Rect::of(0.0, 5.0, 10.0, 10.0)), 10.0);
        assert_eq!(max_delta(&Rect::of(-10.0, -10.0, -5.0, 0.0)), 10.0);
        assert_eq!(max_delta(&Rect::of(-5.0, 0.0, 5.0, 5.0)), 10.0);
    }

    #[test]
    fn max_delta_0() {
        assert_eq!(max_delta(&Rect::of(0.0, 0.0, 0.0, 0.0)), 0.0);
        assert_eq!(max_delta(&Rect::of(1.0, 1.0, 1.0, 1.0)), 0.0);
        assert_eq!(max_delta(&Rect::of(-1.0, -1.0, -1.0, -1.0)), 0.0);
        assert_eq!(max_delta(&Rect::of(5.0, 10.0, 5.0, 10.0)), 0.0);
    }

    #[test]
    fn max_delta_1() {
        assert_eq!(max_delta(&Rect::of(0.0, 0.0, 1.0, 1.0)), 1.0);
        assert_eq!(max_delta(&Rect::of(5.0, 5.0, 6.0, 6.0)), 1.0);
        assert_eq!(max_delta(&Rect::of(-6.0, -6.0, -5.0, -5.0)), 1.0);
        assert_eq!(max_delta(&Rect::of(0.0, 0.0, 0.0, 1.0)), 1.0);
        assert_eq!(max_delta(&Rect::of(5.0, 9.0, 5.0, 10.0)), 1.0);
    }

    #[test]
    fn max_delta_bounds() {
        assert_eq!(max_delta(&Rect::of(MIN + 1.0, MIN + 2.0, 0.0, 0.0)), MAX);
        assert_eq!(max_delta(&Rect::of(MIN + 2.0, MIN + 1.0, 0.0, 0.0)), MAX);
        assert_eq!(max_delta(&Rect::of(0.0, 0.0, MAX - 1.0, MAX)), MAX);
        assert_eq!(max_delta(&Rect::of(0.0, 0.0, MAX, MAX - 1.0)), MAX);
    }

    #[test]
    fn test_len_x() {
        assert_eq!(len_x(&Rect::of(0.0, 0.0, 0.0, MAX)), 1.0);
        assert_eq!(len_x(&Rect::of(0.0, -8_388_608.0, 0.0, 8_388_607.0)), 1.0);
        assert_eq!(len_x(&Rect::of(0.0, 0.0, MAX - 1.0, 0.0)), MAX);
        assert_eq!(len_x(&Rect::of(-8_388_608.0, 0.0, 8_388_606.0, 0.0)), MAX);
    }

    #[test]
    fn test_len_y() {
        assert_eq!(len_y(&Rect::of(0.0, 0.0, MAX, 0.0)), 1.0);
        assert_eq!(len_y(&Rect::of(-8_388_608.0, 0.0, 8_388_607.0, 0.0)), 1.0);
        assert_eq!(len_y(&Rect::of(0.0, 0.0, 0.0, MAX - 1.0)), MAX);
        assert_eq!(len_y(&Rect::of(0.0, -8_388_608.0, 0.0, 8_388_606.0)), MAX);
    }

    #[test]
    fn test_max_len() {
        assert_eq!(max_len(&Rect::of(0.0, 5.0, 10.0, 10.0)), 11.0);
        assert_eq!(max_len(&Rect::of(-10.0, -10.0, -5.0, 0.0)), 11.0);
        assert_eq!(max_len(&Rect::of(-5.0, 0.0, 5.0, 5.0)), 11.0);
    }

    #[test]
    fn max_len_1() {
        assert_eq!(max_len(&Rect::of(0.0, 0.0, 0.0, 0.0)), 1.0);
        assert_eq!(max_len(&Rect::of(1.0, 1.0, 1.0, 1.0)), 1.0);
        assert_eq!(max_len(&Rect::of(-1.0, -1.0, -1.0, -1.0)), 1.0);
        assert_eq!(max_len(&Rect::of(5.0, 10.0, 5.0, 10.0)), 1.0);
    }

    #[test]
    fn max_len_2() {
        assert_eq!(max_len(&Rect::of(0.0, 0.0, 1.0, 1.0)), 2.0);
        assert_eq!(max_len(&Rect::of(5.0, 5.0, 6.0, 6.0)), 2.0);
        assert_eq!(max_len(&Rect::of(-6.0, -6.0, -5.0, -5.0)), 2.0);
        assert_eq!(max_len(&Rect::of(0.0, 0.0, 0.0, 1.0)), 2.0);
        assert_eq!(max_len(&Rect::of(5.0, 9.0, 5.0, 10.0)), 2.0);
    }

    #[test]
    fn max_len_bounds() {
        assert_eq!(max_len(&Rect::of(MIN + 2.0, MIN + 3.0, 0.0, 0.0)), MAX);
        assert_eq!(max_len(&Rect::of(MIN + 3.0, MIN + 2.0, 0.0, 0.0)), MAX);
        assert_eq!(max_len(&Rect::of(0.0, 0.0, MAX - 2.0, MAX - 1.0)), MAX);
        assert_eq!(max_len(&Rect::of(0.0, 0.0, MAX - 1.0, MAX - 2.0)), MAX);
    }
}
