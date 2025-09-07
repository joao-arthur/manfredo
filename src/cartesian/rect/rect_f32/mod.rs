use crate::cartesian::point::point_f32;

mod add;
mod contains_point;
mod contains_rect;
mod deflate;
mod inflate;
mod resize;
mod translate;

pub use add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};
pub use contains_point::contains_point;
pub use contains_rect::contains_rect;
pub use deflate::{assign_deflate, deflate, try_assign_deflate, try_deflate};
pub use inflate::{
    checked_inflate, checked_inflate_assign, saturating_inflate, saturating_inflate_assign, try_checked_inflate, try_checked_inflate_assign, try_saturating_inflate, try_saturating_inflate_assign,
};
pub use resize::{
    checked_resize, checked_resize_assign, saturating_resize, saturating_resize_assign, try_checked_resize, try_checked_resize_assign, try_saturating_resize, try_saturating_resize_assign,
};
pub use translate::{
    checked_translate, checked_translate_assign, saturating_translate, saturating_translate_assign, try_checked_translate, try_checked_translate_assign, wrapping_translate, wrapping_translate_assign,
};

#[derive(PartialEq, Debug, Clone)]
pub struct RectF32 {
    pub min: point_f32::PointF32,
    pub max: point_f32::PointF32,
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

impl RectF32 {
    pub fn of(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        RectF32 { min: point_f32::PointF32 { x: x1, y: y1 }, max: point_f32::PointF32 { x: x2, y: y2 } }
    }

    pub fn largest() -> Self {
        RectF32 { min: point_f32::PointF32::min(), max: point_f32::PointF32::max() }
    }

    pub fn largest_min() -> Self {
        RectF32 { min: point_f32::PointF32::min(), max: point_f32::PointF32::of(0.0, 0.0) }
    }

    pub fn largest_max() -> Self {
        RectF32 { min: point_f32::PointF32::of(0.0, 0.0), max: point_f32::PointF32::max() }
    }

    pub fn min() -> Self {
        RectF32 { min: point_f32::PointF32::min(), max: point_f32::PointF32::min() }
    }

    pub fn max() -> Self {
        RectF32 { min: point_f32::PointF32::max(), max: point_f32::PointF32::max() }
    }

    pub fn iter_x(&self) -> RectF32Iterator {
        RectF32Iterator { begin: self.min.x, end: self.max.x }
    }

    pub fn iter_y(&self) -> RectF32Iterator {
        RectF32Iterator { begin: self.min.y, end: self.max.y }
    }
}

impl std::fmt::Display for RectF32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

pub fn delta_x(r: &RectF32) -> f32 {
    point_f32::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectF32) -> f32 {
    point_f32::delta_y(&r.min, &r.max)
}

pub fn max_delta(r: &RectF32) -> f32 {
    delta_x(r).max(delta_y(r))
}

pub fn len_x(r: &RectF32) -> f32 {
    delta_x(r) + 1.0
}

pub fn len_y(r: &RectF32) -> f32 {
    delta_y(r) + 1.0
}

pub fn max_len(r: &RectF32) -> f32 {
    len_x(r).max(len_y(r))
}

#[cfg(test)]
mod tests {
    use super::{RectF32, delta_x, delta_y, len_x, len_y, max_delta, max_len};
    use crate::cartesian::point::point_f32::{MAX, MIN, PointF32};

    #[test]
    fn rect_f32() {
        assert_eq!(RectF32::largest(), RectF32 { min: PointF32 { x: MIN, y: MIN }, max: PointF32 { x: MAX, y: MAX } });
        assert_eq!(RectF32::largest_min(), RectF32 { min: PointF32 { x: MIN, y: MIN }, max: PointF32 { x: 0.0, y: 0.0 } });
        assert_eq!(RectF32::largest_max(), RectF32 { min: PointF32 { x: 0.0, y: 0.0 }, max: PointF32 { x: MAX, y: MAX } });
        assert_eq!(RectF32::min(), RectF32 { min: PointF32 { x: MIN, y: MIN }, max: PointF32 { x: MIN, y: MIN } });
        assert_eq!(RectF32::max(), RectF32 { min: PointF32 { x: MAX, y: MAX }, max: PointF32 { x: MAX, y: MAX } });
        assert_eq!(RectF32::of(MIN, -0.0, 0.0, MAX), RectF32 { min: PointF32 { x: MIN, y: -0.0 }, max: PointF32 { x: 0.0, y: MAX } });
    }

    #[test]
    fn to_string() {
        assert_eq!(RectF32::largest().to_string(), "((-16777216, -16777216), (16777215, 16777215))");
        assert_eq!(RectF32::largest_min().to_string(), "((-16777216, -16777216), (0, 0))");
        assert_eq!(RectF32::largest_max().to_string(), "((0, 0), (16777215, 16777215))");
        assert_eq!(RectF32::of(MIN, -0.0, 0.0, MAX).to_string(), "((-16777216, -0), (0, 16777215))");
    }

    #[test]
    fn iter_x() {
        assert_eq!(RectF32::of(-6.0, -8.0, -7.0, -6.0).iter_x().collect::<Vec<f32>>(), []);
        assert_eq!(RectF32::of(-6.0, -8.0, -6.0, -6.0).iter_x().collect::<Vec<f32>>(), [-6.0]);
        assert_eq!(RectF32::of(-6.0, -8.0, -5.0, -6.0).iter_x().collect::<Vec<f32>>(), [-6.0, -5.0]);
        assert_eq!(RectF32::of(-6.0, -8.0, -4.0, -6.0).iter_x().collect::<Vec<f32>>(), [-6.0, -5.0, -4.0]);
        assert_eq!(RectF32::of(-6.0, -8.0, -3.0, -6.0).iter_x().collect::<Vec<f32>>(), [-6.0, -5.0, -4.0, -3.0]);
        assert_eq!(RectF32::of(-6.0, -8.0, -3.0, -6.0).iter_x().rev().collect::<Vec<f32>>(), [-3.0, -4.0, -5.0, -6.0]);
        assert_eq!(RectF32::of(-6.0, -8.0, -4.0, -6.0).iter_x().rev().collect::<Vec<f32>>(), [-4.0, -5.0, -6.0]);
        assert_eq!(RectF32::of(-6.0, -8.0, -5.0, -6.0).iter_x().rev().collect::<Vec<f32>>(), [-5.0, -6.0]);
        assert_eq!(RectF32::of(-6.0, -8.0, -6.0, -6.0).iter_x().rev().collect::<Vec<f32>>(), [-6.0]);
        assert_eq!(RectF32::of(-6.0, -8.0, -7.0, -6.0).iter_x().rev().collect::<Vec<f32>>(), []);
    }

    #[test]
    fn iter_y() {
        assert_eq!(RectF32::of(-6.0, -8.0, -4.0, -9.0).iter_y().collect::<Vec<f32>>(), []);
        assert_eq!(RectF32::of(-6.0, -8.0, -4.0, -8.0).iter_y().collect::<Vec<f32>>(), [-8.0]);
        assert_eq!(RectF32::of(-6.0, -8.0, -4.0, -7.0).iter_y().collect::<Vec<f32>>(), [-8.0, -7.0]);
        assert_eq!(RectF32::of(-6.0, -8.0, -4.0, -6.0).iter_y().collect::<Vec<f32>>(), [-8.0, -7.0, -6.0]);
        assert_eq!(RectF32::of(-6.0, -8.0, -4.0, -5.0).iter_y().collect::<Vec<f32>>(), [-8.0, -7.0, -6.0, -5.0]);
        assert_eq!(RectF32::of(-6.0, -8.0, -4.0, -5.0).iter_y().rev().collect::<Vec<f32>>(), [-5.0, -6.0, -7.0, -8.0]);
        assert_eq!(RectF32::of(-6.0, -8.0, -4.0, -6.0).iter_y().rev().collect::<Vec<f32>>(), [-6.0, -7.0, -8.0]);
        assert_eq!(RectF32::of(-6.0, -8.0, -4.0, -7.0).iter_y().rev().collect::<Vec<f32>>(), [-7.0, -8.0]);
        assert_eq!(RectF32::of(-6.0, -8.0, -4.0, -8.0).iter_y().rev().collect::<Vec<f32>>(), [-8.0]);
        assert_eq!(RectF32::of(-6.0, -8.0, -4.0, -9.0).iter_y().rev().collect::<Vec<f32>>(), []);
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectF32::of(0.0, 0.0, 0.0, MAX)), 0.0);
        assert_eq!(delta_x(&RectF32::of(0.0, -8_388_608.0, 0.0, 8_388_607.0)), 0.0);
        assert_eq!(delta_x(&RectF32::of(0.0, 0.0, MAX, 0.0)), MAX);
        assert_eq!(delta_x(&RectF32::of(-8_388_608.0, 0.0, 8_388_607.0, 0.0)), MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectF32::of(0.0, 0.0, MAX, 0.0)), 0.0);
        assert_eq!(delta_y(&RectF32::of(-8_388_608.0, 0.0, 8_388_607.0, 0.0)), 0.0);
        assert_eq!(delta_y(&RectF32::of(0.0, 0.0, 0.0, MAX)), MAX);
        assert_eq!(delta_y(&RectF32::of(0.0, -8_388_608.0, 0.0, 8_388_607.0)), MAX);
    }

    #[test]
    fn test_max_delta() {
        assert_eq!(max_delta(&RectF32::of(0.0, 5.0, 10.0, 10.0)), 10.0);
        assert_eq!(max_delta(&RectF32::of(-10.0, -10.0, -5.0, 0.0)), 10.0);
        assert_eq!(max_delta(&RectF32::of(-5.0, 0.0, 5.0, 5.0)), 10.0);
    }

    #[test]
    fn max_delta_0() {
        assert_eq!(max_delta(&RectF32::of(0.0, 0.0, 0.0, 0.0)), 0.0);
        assert_eq!(max_delta(&RectF32::of(1.0, 1.0, 1.0, 1.0)), 0.0);
        assert_eq!(max_delta(&RectF32::of(-1.0, -1.0, -1.0, -1.0)), 0.0);
        assert_eq!(max_delta(&RectF32::of(5.0, 10.0, 5.0, 10.0)), 0.0);
    }

    #[test]
    fn max_delta_1() {
        assert_eq!(max_delta(&RectF32::of(0.0, 0.0, 1.0, 1.0)), 1.0);
        assert_eq!(max_delta(&RectF32::of(5.0, 5.0, 6.0, 6.0)), 1.0);
        assert_eq!(max_delta(&RectF32::of(-6.0, -6.0, -5.0, -5.0)), 1.0);
        assert_eq!(max_delta(&RectF32::of(0.0, 0.0, 0.0, 1.0)), 1.0);
        assert_eq!(max_delta(&RectF32::of(5.0, 9.0, 5.0, 10.0)), 1.0);
    }

    #[test]
    fn max_delta_bounds() {
        assert_eq!(max_delta(&RectF32::of(MIN + 1.0, MIN + 2.0, 0.0, 0.0)), MAX);
        assert_eq!(max_delta(&RectF32::of(MIN + 2.0, MIN + 1.0, 0.0, 0.0)), MAX);
        assert_eq!(max_delta(&RectF32::of(0.0, 0.0, MAX - 1.0, MAX)), MAX);
        assert_eq!(max_delta(&RectF32::of(0.0, 0.0, MAX, MAX - 1.0)), MAX);
    }

    #[test]
    fn test_len_x() {
        assert_eq!(len_x(&RectF32::of(0.0, 0.0, 0.0, MAX)), 1.0);
        assert_eq!(len_x(&RectF32::of(0.0, -8_388_608.0, 0.0, 8_388_607.0)), 1.0);
        assert_eq!(len_x(&RectF32::of(0.0, 0.0, MAX - 1.0, 0.0)), MAX);
        assert_eq!(len_x(&RectF32::of(-8_388_608.0, 0.0, 8_388_606.0, 0.0)), MAX);
    }

    #[test]
    fn test_len_y() {
        assert_eq!(len_y(&RectF32::of(0.0, 0.0, MAX, 0.0)), 1.0);
        assert_eq!(len_y(&RectF32::of(-8_388_608.0, 0.0, 8_388_607.0, 0.0)), 1.0);
        assert_eq!(len_y(&RectF32::of(0.0, 0.0, 0.0, MAX - 1.0)), MAX);
        assert_eq!(len_y(&RectF32::of(0.0, -8_388_608.0, 0.0, 8_388_606.0)), MAX);
    }

    #[test]
    fn test_max_len() {
        assert_eq!(max_len(&RectF32::of(0.0, 5.0, 10.0, 10.0)), 11.0);
        assert_eq!(max_len(&RectF32::of(-10.0, -10.0, -5.0, 0.0)), 11.0);
        assert_eq!(max_len(&RectF32::of(-5.0, 0.0, 5.0, 5.0)), 11.0);
    }

    #[test]
    fn max_len_1() {
        assert_eq!(max_len(&RectF32::of(0.0, 0.0, 0.0, 0.0)), 1.0);
        assert_eq!(max_len(&RectF32::of(1.0, 1.0, 1.0, 1.0)), 1.0);
        assert_eq!(max_len(&RectF32::of(-1.0, -1.0, -1.0, -1.0)), 1.0);
        assert_eq!(max_len(&RectF32::of(5.0, 10.0, 5.0, 10.0)), 1.0);
    }

    #[test]
    fn max_len_2() {
        assert_eq!(max_len(&RectF32::of(0.0, 0.0, 1.0, 1.0)), 2.0);
        assert_eq!(max_len(&RectF32::of(5.0, 5.0, 6.0, 6.0)), 2.0);
        assert_eq!(max_len(&RectF32::of(-6.0, -6.0, -5.0, -5.0)), 2.0);
        assert_eq!(max_len(&RectF32::of(0.0, 0.0, 0.0, 1.0)), 2.0);
        assert_eq!(max_len(&RectF32::of(5.0, 9.0, 5.0, 10.0)), 2.0);
    }

    #[test]
    fn max_len_bounds() {
        assert_eq!(max_len(&RectF32::of(MIN + 2.0, MIN + 3.0, 0.0, 0.0)), MAX);
        assert_eq!(max_len(&RectF32::of(MIN + 3.0, MIN + 2.0, 0.0, 0.0)), MAX);
        assert_eq!(max_len(&RectF32::of(0.0, 0.0, MAX - 2.0, MAX - 1.0)), MAX);
        assert_eq!(max_len(&RectF32::of(0.0, 0.0, MAX - 1.0, MAX - 2.0)), MAX);
    }
}
