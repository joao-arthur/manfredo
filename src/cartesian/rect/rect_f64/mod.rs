use crate::cartesian::{point::point_f64, rect::rect_f32};

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
};
pub use self::resize::{
    checked_resize, checked_resize_assign, saturating_resize, saturating_resize_assign, try_checked_resize, try_checked_resize_assign, try_saturating_resize, try_saturating_resize_assign,
};
pub use self::translate::{
    checked_translate, checked_translate_assign, saturating_translate, saturating_translate_assign, try_checked_translate, try_checked_translate_assign, wrapping_translate, wrapping_translate_assign,
};

#[derive(PartialEq, Debug, Clone)]
pub struct RectF64 {
    pub min: point_f64::PointF64,
    pub max: point_f64::PointF64,
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

impl RectF64 {
    pub fn of(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        RectF64 { min: point_f64::PointF64 { x: x1, y: y1 }, max: point_f64::PointF64 { x: x2, y: y2 } }
    }

    pub fn largest() -> Self {
        RectF64 { min: point_f64::PointF64::min(), max: point_f64::PointF64::max() }
    }

    pub fn min() -> Self {
        RectF64 { min: point_f64::PointF64::min(), max: point_f64::PointF64::min() }
    }

    pub fn max() -> Self {
        RectF64 { min: point_f64::PointF64::max(), max: point_f64::PointF64::max() }
    }

    pub fn iter_x(&self) -> RectF64Iterator {
        RectF64Iterator { begin: self.min.x, end: self.max.x }
    }

    pub fn iter_y(&self) -> RectF64Iterator {
        RectF64Iterator { begin: self.min.y, end: self.max.y }
    }
}

impl From<rect_f32::Rect> for RectF64 {
    fn from(r: rect_f32::Rect) -> Self {
        RectF64 { min: point_f64::PointF64::of(r.min.x.into(), r.min.y.into()), max: point_f64::PointF64::of(r.max.x.into(), r.max.y.into()) }
    }
}

impl std::fmt::Display for RectF64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_x(r: &RectF64) -> f64 {
    point_f64::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectF64) -> f64 {
    point_f64::delta_y(&r.min, &r.max)
}

pub fn max_delta(r: &RectF64) -> f64 {
    delta_x(r).max(delta_y(r))
}

pub fn len_x(r: &RectF64) -> f64 {
    delta_x(r) + 1.0
}

pub fn len_y(r: &RectF64) -> f64 {
    delta_y(r) + 1.0
}

pub fn max_len(r: &RectF64) -> f64 {
    len_x(r).max(len_y(r))
}

#[cfg(test)]
mod tests {
    use super::{RectF64, delta_x, delta_y, len_x, len_y, max_delta, max_len};
    use crate::cartesian::{
        point::{
            point_f32,
            point_f64::{MAX, MIN, PointF64},
        },
        rect::rect_f32::Rect,
    };

    #[test]
    fn rect_f64() {
        assert_eq!(RectF64::largest(), RectF64 { min: PointF64 { x: MIN, y: MIN }, max: PointF64 { x: MAX, y: MAX } });
        assert_eq!(RectF64::min(), RectF64 { min: PointF64 { x: MIN, y: MIN }, max: PointF64 { x: MIN, y: MIN } });
        assert_eq!(RectF64::max(), RectF64 { min: PointF64 { x: MAX, y: MAX }, max: PointF64 { x: MAX, y: MAX } });
        assert_eq!(RectF64::of(MIN, -0.0, 0.0, MAX), RectF64 { min: PointF64 { x: MIN, y: -0.0 }, max: PointF64 { x: 0.0, y: MAX } });
    }

    #[test]
    fn to_string() {
        assert_eq!(RectF64::largest().to_string(), "((-9007199254740992, -9007199254740992), (9007199254740991, 9007199254740991))");
        assert_eq!(RectF64::of(MIN, -0.0, 0.0, MAX).to_string(), "((-9007199254740992, -0), (0, 9007199254740991))");
    }

    #[test]
    fn from() {
        assert_eq!(
            RectF64::from(Rect::largest()),
            RectF64 { min: PointF64 { x: point_f32::MIN.into(), y: point_f32::MIN.into() }, max: PointF64 { x: point_f32::MAX.into(), y: point_f32::MAX.into() } }
        );
    }

    #[test]
    fn iter_x() {
        assert_eq!(RectF64::of(-6.0, -8.0, -7.0, -6.0).iter_x().collect::<Vec<f64>>(), []);
        assert_eq!(RectF64::of(-6.0, -8.0, -6.0, -6.0).iter_x().collect::<Vec<f64>>(), [-6.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -5.0, -6.0).iter_x().collect::<Vec<f64>>(), [-6.0, -5.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -6.0).iter_x().collect::<Vec<f64>>(), [-6.0, -5.0, -4.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -3.0, -6.0).iter_x().collect::<Vec<f64>>(), [-6.0, -5.0, -4.0, -3.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -3.0, -6.0).iter_x().rev().collect::<Vec<f64>>(), [-3.0, -4.0, -5.0, -6.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -6.0).iter_x().rev().collect::<Vec<f64>>(), [-4.0, -5.0, -6.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -5.0, -6.0).iter_x().rev().collect::<Vec<f64>>(), [-5.0, -6.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -6.0, -6.0).iter_x().rev().collect::<Vec<f64>>(), [-6.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -7.0, -6.0).iter_x().rev().collect::<Vec<f64>>(), []);
    }

    #[test]
    fn iter_y() {
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -9.0).iter_y().collect::<Vec<f64>>(), []);
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -8.0).iter_y().collect::<Vec<f64>>(), [-8.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -7.0).iter_y().collect::<Vec<f64>>(), [-8.0, -7.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -6.0).iter_y().collect::<Vec<f64>>(), [-8.0, -7.0, -6.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -5.0).iter_y().collect::<Vec<f64>>(), [-8.0, -7.0, -6.0, -5.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -5.0).iter_y().rev().collect::<Vec<f64>>(), [-5.0, -6.0, -7.0, -8.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -6.0).iter_y().rev().collect::<Vec<f64>>(), [-6.0, -7.0, -8.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -7.0).iter_y().rev().collect::<Vec<f64>>(), [-7.0, -8.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -8.0).iter_y().rev().collect::<Vec<f64>>(), [-8.0]);
        assert_eq!(RectF64::of(-6.0, -8.0, -4.0, -9.0).iter_y().rev().collect::<Vec<f64>>(), []);
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectF64::of(0.0, 0.0, 0.0, MAX)), 0.0);
        assert_eq!(delta_x(&RectF64::of(0.0, -4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0)), 0.0);
        assert_eq!(delta_x(&RectF64::of(0.0, 0.0, MAX, 0.0)), MAX);
        assert_eq!(delta_x(&RectF64::of(-4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0, 0.0)), MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectF64::of(0.0, 0.0, MAX, 0.0)), 0.0);
        assert_eq!(delta_y(&RectF64::of(-4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0, 0.0)), 0.0);
        assert_eq!(delta_y(&RectF64::of(0.0, 0.0, 0.0, MAX)), MAX);
        assert_eq!(delta_y(&RectF64::of(0.0, -4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0)), MAX);
    }

    #[test]
    fn test_max_delta() {
        assert_eq!(max_delta(&RectF64::of(0.0, 5.0, 10.0, 10.0)), 10.0);
        assert_eq!(max_delta(&RectF64::of(-10.0, -10.0, -5.0, 0.0)), 10.0);
        assert_eq!(max_delta(&RectF64::of(-5.0, 0.0, 5.0, 5.0)), 10.0);
    }

    #[test]
    fn max_delta_0() {
        assert_eq!(max_delta(&RectF64::of(0.0, 0.0, 0.0, 0.0)), 0.0);
        assert_eq!(max_delta(&RectF64::of(1.0, 1.0, 1.0, 1.0)), 0.0);
        assert_eq!(max_delta(&RectF64::of(-1.0, -1.0, -1.0, -1.0)), 0.0);
        assert_eq!(max_delta(&RectF64::of(5.0, 10.0, 5.0, 10.0)), 0.0);
    }

    #[test]
    fn max_delta_1() {
        assert_eq!(max_delta(&RectF64::of(0.0, 0.0, 1.0, 1.0)), 1.0);
        assert_eq!(max_delta(&RectF64::of(5.0, 5.0, 6.0, 6.0)), 1.0);
        assert_eq!(max_delta(&RectF64::of(-6.0, -6.0, -5.0, -5.0)), 1.0);
        assert_eq!(max_delta(&RectF64::of(0.0, 0.0, 0.0, 1.0)), 1.0);
        assert_eq!(max_delta(&RectF64::of(5.0, 9.0, 5.0, 10.0)), 1.0);
    }

    #[test]
    fn max_delta_bounds() {
        assert_eq!(max_delta(&RectF64::of(MIN + 1.0, MIN + 2.0, 0.0, 0.0)), MAX);
        assert_eq!(max_delta(&RectF64::of(MIN + 2.0, MIN + 1.0, 0.0, 0.0)), MAX);
        assert_eq!(max_delta(&RectF64::of(0.0, 0.0, MAX - 1.0, MAX)), MAX);
        assert_eq!(max_delta(&RectF64::of(0.0, 0.0, MAX, MAX - 1.0)), MAX);
    }

    #[test]
    fn test_len_x() {
        assert_eq!(len_x(&RectF64::of(0.0, 0.0, 0.0, MAX)), 1.0);
        assert_eq!(len_x(&RectF64::of(0.0, -4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0)), 1.0);
        assert_eq!(len_x(&RectF64::of(0.0, 0.0, MAX - 1.0, 0.0)), MAX);
        assert_eq!(len_x(&RectF64::of(-4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_494.0, 0.0)), MAX);
    }

    #[test]
    fn test_len_y() {
        assert_eq!(len_y(&RectF64::of(0.0, 0.0, MAX, 0.0)), 1.0);
        assert_eq!(len_y(&RectF64::of(-4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0, 0.0)), 1.0);
        assert_eq!(len_y(&RectF64::of(0.0, 0.0, 0.0, MAX - 1.0)), MAX);
        assert_eq!(len_y(&RectF64::of(0.0, -4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_494.0)), MAX);
    }

    #[test]
    fn test_max_len() {
        assert_eq!(max_len(&RectF64::of(0.0, 5.0, 10.0, 10.0)), 11.0);
        assert_eq!(max_len(&RectF64::of(-10.0, -10.0, -5.0, 0.0)), 11.0);
        assert_eq!(max_len(&RectF64::of(-5.0, 0.0, 5.0, 5.0)), 11.0);
    }

    #[test]
    fn max_len_1() {
        assert_eq!(max_len(&RectF64::of(0.0, 0.0, 0.0, 0.0)), 1.0);
        assert_eq!(max_len(&RectF64::of(1.0, 1.0, 1.0, 1.0)), 1.0);
        assert_eq!(max_len(&RectF64::of(-1.0, -1.0, -1.0, -1.0)), 1.0);
        assert_eq!(max_len(&RectF64::of(5.0, 10.0, 5.0, 10.0)), 1.0);
    }

    #[test]
    fn max_len_2() {
        assert_eq!(max_len(&RectF64::of(0.0, 0.0, 1.0, 1.0)), 2.0);
        assert_eq!(max_len(&RectF64::of(5.0, 5.0, 6.0, 6.0)), 2.0);
        assert_eq!(max_len(&RectF64::of(-6.0, -6.0, -5.0, -5.0)), 2.0);
        assert_eq!(max_len(&RectF64::of(0.0, 0.0, 0.0, 1.0)), 2.0);
        assert_eq!(max_len(&RectF64::of(5.0, 9.0, 5.0, 10.0)), 2.0);
    }

    #[test]
    fn max_len_bounds() {
        assert_eq!(max_len(&RectF64::of(MIN + 2.0, MIN + 3.0, 0.0, 0.0)), MAX);
        assert_eq!(max_len(&RectF64::of(MIN + 3.0, MIN + 2.0, 0.0, 0.0)), MAX);
        assert_eq!(max_len(&RectF64::of(0.0, 0.0, MAX - 2.0, MAX - 1.0)), MAX);
        assert_eq!(max_len(&RectF64::of(0.0, 0.0, MAX - 1.0, MAX - 2.0)), MAX);
    }
}
