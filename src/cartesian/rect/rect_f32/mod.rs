use crate::cartesian::point::point_f32;

//pub mod add;
pub mod contains_point;
pub mod contains_rect;
pub mod deflate;
pub mod inflate;
//pub mod resize;
//pub mod translate;

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
    use crate::cartesian::point::point_f32::{MAX, MIN, PointF32};

    use super::{RectF32, delta_x, delta_y, len_x, len_y, max_delta, max_len};

    #[test]
    fn rect_f32() {
        assert_eq!(RectF32::largest(), RectF32 { min: PointF32 { x: MIN, y: MIN }, max: PointF32 { x: MAX, y: MAX } });
        assert_eq!(RectF32::min(), RectF32 { min: PointF32 { x: MIN, y: MIN }, max: PointF32 { x: MIN, y: MIN } });
        assert_eq!(RectF32::max(), RectF32 { min: PointF32 { x: MAX, y: MAX }, max: PointF32 { x: MAX, y: MAX } });
        assert_eq!(RectF32::of(MIN, -0.0, 0.0, MAX), RectF32 { min: PointF32 { x: MIN, y: -0.0 }, max: PointF32 { x: 0.0, y: MAX } });
    }

    #[test]
    fn to_string() {
        assert_eq!(RectF32::largest().to_string(), "((-16777216, -16777216), (16777215, 16777215))");
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

    //    #[test]
    //    fn inflate_min_bounds() {
    //        let mut r = RectF32::of(MIN + 7.0, MIN + 2.0, 4.0, 13.0);
    //        inflate(&mut r);
    //        assert_eq!(r, RectF32::of(MIN + 6.0, MIN + 1.0, 5.0, 14.0));
    //        inflate(&mut r);
    //        assert_eq!(r, RectF32::of(MIN + 5.0, MIN, 6.0, 15.0));
    //        inflate(&mut r);
    //        assert_eq!(r, RectF32::of(MIN + 4.0, MIN, 7.0, 17.0));
    //        inflate(&mut r);
    //        assert_eq!(r, RectF32::of(MIN + 3.0, MIN, 8.0, 19.0));
    //        inflate(&mut r);
    //        assert_eq!(r, RectF32::of(MIN + 2.0, MIN, 9.0, 21.0));
    //        inflate(&mut r);
    //        assert_eq!(r, RectF32::of(MIN + 1.0, MIN, 10.0, 23.0));
    //        inflate(&mut r);
    //        assert_eq!(r, RectF32::of(MIN, MIN, 11.0, 25.0));
    //        inflate(&mut r);
    //        assert_eq!(r, RectF32::of(MIN, MIN, 13.0, 27.0));
    //    }
    //
    //    #[test]
    //    fn inflate_max_bounds() {
    //        let mut r = RectF32::of(-100.0, 30.0, MAX - 5.0, MAX - 3.0);
    //        inflate(&mut r);
    //        assert_eq!(r, RectF32::of(-101.0, 29.0, MAX - 4.0, MAX - 2.0));
    //        inflate(&mut r);
    //        assert_eq!(r, RectF32::of(-102.0, 28.0, MAX - 3.0, MAX - 1.0));
    //        inflate(&mut r);
    //        assert_eq!(r, RectF32::of(-103.0, 27.0, MAX - 2.0, MAX));
    //        inflate(&mut r);
    //        assert_eq!(r, RectF32::of(-104.0, 25.0, MAX - 1.0, MAX));
    //        inflate(&mut r);
    //        assert_eq!(r, RectF32::of(-105.0, 23.0, MAX, MAX));
    //        inflate(&mut r);
    //        assert_eq!(r, RectF32::of(-107.0, 21.0, MAX, MAX));
    //        inflate(&mut r);
    //        assert_eq!(r, RectF32::of(-109.0, 19.0, MAX, MAX));
    //        inflate(&mut r);
    //        assert_eq!(r, RectF32::of(-111.0, 17.0, MAX, MAX));
    //    }
    //
    //    #[test]
    //    fn inflate_almost_min_bounds() {
    //        let mut r = RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX);
    //        inflate(&mut r);
    //        assert_eq!(r, RectF32::largest());
    //    }
    //
    //    #[test]
    //    fn inflate_almost_max_bounds() {
    //        let mut r = RectF32::of(MIN, MIN, MAX - 1.0, MAX - 1.0);
    //        inflate(&mut r);
    //        assert_eq!(r, RectF32::largest());
    //    }
    //
    //    #[test]
    //    fn inflate_max_width() {
    //        let mut r = RectF32::of(MIN, 10.0, MAX, 50.0);
    //        inflate(&mut r);
    //        assert_eq!(r, RectF32::of(MIN, 10.0, MAX, 50.0));
    //    }
    //
    //    #[test]
    //    fn inflate_max_height() {
    //        let mut r = RectF32::of(10.0, MIN, 50.0, MAX);
    //        inflate(&mut r);
    //        assert_eq!(r, RectF32::of(10.0, MIN, 50.0, MAX));
    //    }
    //
    //    #[test]
    //    fn resize_odd() {
    //        let mut r = RectF32::of(-5.0, -5.0, 5.0, 5.0);
    //        resize(&mut r, 11.0);
    //        assert_eq!(r, RectF32::of(-5.0, -5.0, 5.0, 5.0));
    //        resize(&mut r, 9.0);
    //        assert_eq!(r, RectF32::of(-4.0, -4.0, 4.0, 4.0));
    //        resize(&mut r, 7.0);
    //        assert_eq!(r, RectF32::of(-3.0, -3.0, 3.0, 3.0));
    //        resize(&mut r, 5.0);
    //        assert_eq!(r, RectF32::of(-2.0, -2.0, 2.0, 2.0));
    //        resize(&mut r, 3.0);
    //        assert_eq!(r, RectF32::of(-1.0, -1.0, 1.0, 1.0));
    //        resize(&mut r, 1.0);
    //        assert_eq!(r, RectF32::of(-1.0, -1.0, 1.0, 1.0));
    //        resize(&mut r, 1.0);
    //        assert_eq!(r, RectF32::of(-1.0, -1.0, 1.0, 1.0));
    //        resize(&mut r, 3.0);
    //        assert_eq!(r, RectF32::of(-1.0, -1.0, 1.0, 1.0));
    //        resize(&mut r, 5.0);
    //        assert_eq!(r, RectF32::of(-2.0, -2.0, 2.0, 2.0));
    //        resize(&mut r, 7.0);
    //        assert_eq!(r, RectF32::of(-3.0, -3.0, 3.0, 3.0));
    //    }
    //
    //    #[test]
    //    fn resize_even() {
    //        let mut r = RectF32::of(-5.0, -5.0, 4.0, 4.0);
    //        resize(&mut r, 10.0);
    //        assert_eq!(r, RectF32::of(-5.0, -5.0, 4.0, 4.0));
    //        resize(&mut r, 8.0);
    //        assert_eq!(r, RectF32::of(-4.0, -4.0, 3.0, 3.0));
    //        resize(&mut r, 6.0);
    //        assert_eq!(r, RectF32::of(-3.0, -3.0, 2.0, 2.0));
    //        resize(&mut r, 4.0);
    //        assert_eq!(r, RectF32::of(-2.0, -2.0, 1.0, 1.0));
    //        resize(&mut r, 2.0);
    //        assert_eq!(r, RectF32::of(-2.0, -2.0, 1.0, 1.0));
    //        resize(&mut r, 2.0);
    //        assert_eq!(r, RectF32::of(-2.0, -2.0, 1.0, 1.0));
    //        resize(&mut r, 4.0);
    //        assert_eq!(r, RectF32::of(-2.0, -2.0, 1.0, 1.0));
    //        resize(&mut r, 6.0);
    //        assert_eq!(r, RectF32::of(-3.0, -3.0, 2.0, 2.0));
    //    }
    //
    //    #[test]
    //    fn resize_even_2nd_scenario() {
    //        let mut r = RectF32::of(-4.0, -4.0, 5.0, 5.0);
    //        resize(&mut r, 10.0);
    //        assert_eq!(r, RectF32::of(-4.0, -4.0, 5.0, 5.0));
    //        resize(&mut r, 8.0);
    //        assert_eq!(r, RectF32::of(-3.0, -3.0, 4.0, 4.0));
    //        resize(&mut r, 6.0);
    //        assert_eq!(r, RectF32::of(-2.0, -2.0, 3.0, 3.0));
    //        resize(&mut r, 4.0);
    //        assert_eq!(r, RectF32::of(-1.0, -1.0, 2.0, 2.0));
    //        resize(&mut r, 2.0);
    //        assert_eq!(r, RectF32::of(-1.0, -1.0, 2.0, 2.0));
    //        resize(&mut r, 2.0);
    //        assert_eq!(r, RectF32::of(-1.0, -1.0, 2.0, 2.0));
    //        resize(&mut r, 4.0);
    //        assert_eq!(r, RectF32::of(-1.0, -1.0, 2.0, 2.0));
    //        resize(&mut r, 6.0);
    //        assert_eq!(r, RectF32::of(-2.0, -2.0, 3.0, 3.0));
    //    }
    //
    //    #[test]
    //    fn resize_odd_min_bounds_big_delta() {
    //        let mut r = RectF32::of(MIN, MIN, MIN + 2.0, MIN + 2.0);
    //        resize(&mut r, MAX);
    //        assert_eq!(r, RectF32::of(MIN, MIN, -2.0, -2.0));
    //    }
    //
    //    #[test]
    //    fn resize_even_min_bounds_big_delta() {
    //        let mut r = RectF32::of(MIN, MIN, MIN + 3.0, MIN + 3.0);
    //        resize(&mut r, MAX - 1.0);
    //        assert_eq!(r, RectF32::of(MIN, MIN, -3.0, -3.0));
    //    }
    //
    //    #[test]
    //    fn resize_odd_max_bounds_big_delta() {
    //        let mut r = RectF32::of(MAX - 2.0, MAX - 2.0, MAX, MAX);
    //        resize(&mut r, MAX);
    //        assert_eq!(r, RectF32::of(1.0, 1.0, MAX, MAX));
    //    }
    //
    //    #[test]
    //    fn resize_even_max_bounds_big_delta() {
    //        let mut r = RectF32::of(MAX - 3.0, MAX - 3.0, MAX, MAX);
    //        resize(&mut r, MAX - 1.0);
    //        assert_eq!(r, RectF32::of(2.0, 2.0, MAX, MAX));
    //    }
    //
    //    #[test]
    //    fn test_assign_saturating_add() {
    //        let mut r = RectF32::of(0.0, 0.0, 10.0, 10.0);
    //        assign_saturating_add(&mut r, &PointF32::of(10.0, 10.0));
    //        assert_eq!(r, RectF32::of(10.0, 10.0, 20.0, 20.0));
    //        assign_saturating_add(&mut r, &PointF32::of(-20.0, -20.0));
    //        assert_eq!(r, RectF32::of(-10.0, -10.0, 0.0, 0.0));
    //        assign_saturating_add(&mut r, &PointF32::of(2.0, 2.0));
    //        assert_eq!(r, RectF32::of(-8.0, -8.0, 2.0, 2.0));
    //    }
    //
    //    #[test]
    //    fn assign_saturating_add_min_bounds() {
    //        let mut r = RectF32::of(MIN + 5.0, MIN + 10.0, -100.0, -100.0);
    //        assign_saturating_add(&mut r, &PointF32::of(-10.0, -10.0));
    //        assert_eq!(r, RectF32::of(MIN, MIN, -105.0, -110.0));
    //    }
    //
    //    #[test]
    //    fn assign_saturating_add_max_bounds() {
    //        let mut r = RectF32::of(100.0, 100.0, MAX - 5.0, MAX - 10.0);
    //        assign_saturating_add(&mut r, &PointF32::of(20.0, 20.0));
    //        assert_eq!(r, RectF32::of(105.0, 110.0, MAX, MAX));
    //    }
    //
    //    #[test]
    //    fn assign_saturating_add_min_bounds_big_delta() {
    //        let mut r = RectF32::of(MIN, MIN, MIN + 10.0, MIN + 10.0);
    //        assign_saturating_add(&mut r, &PointF32::min());
    //        assert_eq!(r, RectF32::of(MIN, MIN, MIN + 10.0, MIN + 10.0));
    //    }
    //
    //    #[test]
    //    fn assign_saturating_add_max_bounds_big_delta() {
    //        let mut r = RectF32::of(MAX - 10.0, MAX - 10.0, MAX, MAX);
    //        assign_saturating_add(&mut r, &PointF32::max());
    //        assert_eq!(r, RectF32::of(MAX - 10.0, MAX - 10.0, MAX, MAX));
    //    }
    //
    //    #[test]
    //    fn assign_saturating_add_min_bounds_big_rect_big_delta() {
    //        let mut r = RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX);
    //        assign_saturating_add(&mut r, &PointF32::min());
    //        assert_eq!(r, RectF32::of(MIN, MIN, MAX - 1.0, MAX - 1.0));
    //    }
    //
    //    #[test]
    //    fn assign_saturating_add_max_bounds_big_rect_big_delta() {
    //        let mut r = RectF32::of(MIN, MIN, MAX - 1.0, MAX - 1.0);
    //        assign_saturating_add(&mut r, &PointF32::max());
    //        assert_eq!(r, RectF32::of(MIN + 1.0, MIN + 1.0, MAX, MAX));
    //    }
}
