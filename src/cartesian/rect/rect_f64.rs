use crate::cartesian::point::point_f64;

#[derive(PartialEq, Debug, Clone)]
pub struct RectF64 {
    pub min: point_f64::PointF64,
    pub max: point_f64::PointF64,
}

impl RectF64 {
    pub fn of(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        RectF64 { min: point_f64::PointF64 { x: x1, y: y1 }, max: point_f64::PointF64 { x: x2, y: y2 } }
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

pub fn max_dimension(r: &RectF64) -> f64 {
    let dx = delta_x(r);
    let dy = delta_y(r);
    dx.max(dy)
}

pub fn inflate(r: &mut RectF64) {
    let is_min_x = r.min.x == point_f64::MIN;
    let is_min_y = r.min.y == point_f64::MIN;
    let is_max_x = r.max.x == point_f64::MAX;
    let is_max_y = r.max.y == point_f64::MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return;
    }
    let min_x_modifier = 1.0 - f64::from(is_min_x) + f64::from(is_max_x);
    let min_y_modifier = 1.0 - f64::from(is_min_y) + f64::from(is_max_y);
    let max_x_modifier = 1.0 + f64::from(is_min_x) - f64::from(is_max_x);
    let max_y_modifier = 1.0 + f64::from(is_min_y) - f64::from(is_max_y);
    r.min.x = (r.min.x - min_x_modifier).max(point_f64::MIN);
    r.min.y = (r.min.y - min_y_modifier).max(point_f64::MIN);
    r.max.x = (r.max.x + max_x_modifier).min(point_f64::MAX);
    r.max.y = (r.max.y + max_y_modifier).min(point_f64::MAX);
}

pub fn deflate(r: &mut RectF64) {
    if delta_x(r) < 3.0 || delta_y(r) < 3.0 {
        return;
    }
    r.min.x += 1.0;
    r.min.y += 1.0;
    r.max.x -= 1.0;
    r.max.y -= 1.0;
}

pub fn translate(r: &mut RectF64, delta: &point_f64::PointF64) {
    let dx = delta_x(r);
    let dy = delta_y(r);
    let temp_min_x = r.min.x + delta.x;
    let temp_min_y = r.min.y + delta.y;
    let min_x = temp_min_x.clamp(point_f64::MIN, point_f64::MAX - dx);
    let min_y = temp_min_y.clamp(point_f64::MIN, point_f64::MAX - dy);
    r.min.x = min_x;
    r.min.y = min_y;
    r.max.x = min_x + dx;
    r.max.y = min_y + dy;
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_f64::{MAX, MIN, PointF64};

    use super::{RectF64, deflate, delta_x, delta_y, inflate, max_dimension, translate};

    #[test]
    fn rect_f64() {
        assert_eq!(RectF64::of(MIN, -0.0, 0.0, MAX), RectF64 { min: PointF64 { x: MIN, y: -0.0 }, max: PointF64 { x: 0.0, y: MAX } });
        assert_eq!(RectF64::of(MIN, -0.0, 0.0, MAX).to_string(), "((-9007199254740992, -0), (0, 9007199254740991))");
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
    fn test_max_dimension() {
        assert_eq!(max_dimension(&RectF64::of(0.0, 5.0, 10.0, 10.0)), 10.0);
        assert_eq!(max_dimension(&RectF64::of(-10.0, -10.0, -5.0, 0.0)), 10.0);
        assert_eq!(max_dimension(&RectF64::of(-5.0, 0.0, 5.0, 5.0)), 10.0);
    }

    #[test]
    fn max_dimension_0() {
        assert_eq!(max_dimension(&RectF64::of(0.0, 0.0, 0.0, 0.0)), 0.0);
        assert_eq!(max_dimension(&RectF64::of(1.0, 1.0, 1.0, 1.0)), 0.0);
        assert_eq!(max_dimension(&RectF64::of(-1.0, -1.0, -1.0, -1.0)), 0.0);
        assert_eq!(max_dimension(&RectF64::of(5.0, 10.0, 5.0, 10.0)), 0.0);
    }

    #[test]
    fn max_dimension_1() {
        assert_eq!(max_dimension(&RectF64::of(0.0, 0.0, 1.0, 1.0)), 1.0);
        assert_eq!(max_dimension(&RectF64::of(5.0, 5.0, 6.0, 6.0)), 1.0);
        assert_eq!(max_dimension(&RectF64::of(-6.0, -6.0, -5.0, -5.0)), 1.0);
        assert_eq!(max_dimension(&RectF64::of(0.0, 0.0, 0.0, 1.0)), 1.0);
        assert_eq!(max_dimension(&RectF64::of(5.0, 9.0, 5.0, 10.0)), 1.0);
    }

    #[test]
    fn max_dimension_bounds() {
        assert_eq!(max_dimension(&RectF64::of(MIN + 1.0, MIN + 2.0, 0.0, 0.0)), MAX);
        assert_eq!(max_dimension(&RectF64::of(MIN + 2.0, MIN + 1.0, 0.0, 0.0)), MAX);
        assert_eq!(max_dimension(&RectF64::of(0.0, 0.0, MAX - 1.0, MAX)), MAX);
        assert_eq!(max_dimension(&RectF64::of(0.0, 0.0, MAX, MAX - 1.0)), MAX);
    }

    #[test]
    fn inflate_min_bounds() {
        let mut r = RectF64::of(MIN + 7.0, MIN + 2.0, 4.0, 13.0);
        inflate(&mut r);
        assert_eq!(r, RectF64::of(MIN + 6.0, MIN + 1.0, 5.0, 14.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(MIN + 5.0, MIN, 6.0, 15.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(MIN + 4.0, MIN, 7.0, 17.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(MIN + 3.0, MIN, 8.0, 19.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(MIN + 2.0, MIN, 9.0, 21.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(MIN + 1.0, MIN, 10.0, 23.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(MIN, MIN, 11.0, 25.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(MIN, MIN, 13.0, 27.0));
    }

    #[test]
    fn inflate_max_bounds() {
        let mut r = RectF64::of(-100.0, 30.0, MAX - 5.0, MAX - 3.0);
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-101.0, 29.0, MAX - 4.0, MAX - 2.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-102.0, 28.0, MAX - 3.0, MAX - 1.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-103.0, 27.0, MAX - 2.0, MAX));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-104.0, 25.0, MAX - 1.0, MAX));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-105.0, 23.0, MAX, MAX));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-107.0, 21.0, MAX, MAX));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-109.0, 19.0, MAX, MAX));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-111.0, 17.0, MAX, MAX));
    }

    #[test]
    fn inflate_almost_min_bounds() {
        let mut r = RectF64::of(MIN + 1.0, MIN + 1.0, MAX, MAX);
        inflate(&mut r);
        assert_eq!(r, RectF64::of(MIN, MIN, MAX, MAX));
    }

    #[test]
    fn inflate_almost_max_bounds() {
        let mut r = RectF64::of(MIN, MIN, MAX - 1.0, MAX - 1.0);
        inflate(&mut r);
        assert_eq!(r, RectF64::of(MIN, MIN, MAX, MAX));
    }

    #[test]
    fn inflate_max_width() {
        let mut r = RectF64::of(MIN, 10.0, MAX, 50.0);
        inflate(&mut r);
        assert_eq!(r, RectF64::of(MIN, 10.0, MAX, 50.0));
    }

    #[test]
    fn inflate_max_height() {
        let mut r = RectF64::of(10.0, MIN, 50.0, MAX);
        inflate(&mut r);
        assert_eq!(r, RectF64::of(10.0, MIN, 50.0, MAX));
    }

    #[test]
    fn deflate_odd_size() {
        let mut r = RectF64::of(-5.0, -5.0, 5.0, 5.0);
        deflate(&mut r);
        assert_eq!(r, RectF64::of(-4.0, -4.0, 4.0, 4.0));
        deflate(&mut r);
        assert_eq!(r, RectF64::of(-3.0, -3.0, 3.0, 3.0));
        deflate(&mut r);
        assert_eq!(r, RectF64::of(-2.0, -2.0, 2.0, 2.0));
        deflate(&mut r);
        assert_eq!(r, RectF64::of(-1.0, -1.0, 1.0, 1.0));
        deflate(&mut r);
        assert_eq!(r, RectF64::of(-1.0, -1.0, 1.0, 1.0));
    }

    #[test]
    fn deflate_even_size() {
        let mut r = RectF64::of(-5.0, -5.0, 4.0, 4.0);
        deflate(&mut r);
        assert_eq!(r, RectF64::of(-4.0, -4.0, 3.0, 3.0));
        deflate(&mut r);
        assert_eq!(r, RectF64::of(-3.0, -3.0, 2.0, 2.0));
        deflate(&mut r);
        assert_eq!(r, RectF64::of(-2.0, -2.0, 1.0, 1.0));
        deflate(&mut r);
        assert_eq!(r, RectF64::of(-1.0, -1.0, 0.0, 0.0));
        deflate(&mut r);
        assert_eq!(r, RectF64::of(-1.0, -1.0, 0.0, 0.0));
    }

    #[test]
    fn test_translate() {
        let mut r = RectF64::of(0.0, 0.0, 10.0, 10.0);
        translate(&mut r, &PointF64::of(10.0, 10.0));
        assert_eq!(r, RectF64::of(10.0, 10.0, 20.0, 20.0));
        translate(&mut r, &PointF64::of(-20.0, -20.0));
        assert_eq!(r, RectF64::of(-10.0, -10.0, 0.0, 0.0));
        translate(&mut r, &PointF64::of(2.0, 2.0));
        assert_eq!(r, RectF64::of(-8.0, -8.0, 2.0, 2.0));
    }

    #[test]
    fn translate_min_bounds() {
        let mut r = RectF64::of(MIN + 5.0, MIN + 10.0, -100.0, -100.0);
        translate(&mut r, &PointF64::of(-10.0, -10.0));
        assert_eq!(r, RectF64::of(MIN, MIN, -105.0, -110.0));
    }

    #[test]
    fn translate_max_bounds() {
        let mut r = RectF64::of(100.0, 100.0, MAX - 5.0, MAX - 10.0);
        translate(&mut r, &PointF64::of(20.0, 20.0));
        assert_eq!(r, RectF64::of(105.0, 110.0, MAX, MAX));
    }

    #[test]
    fn translate_min_bounds_big_delta() {
        let mut r = RectF64::of(MIN, MIN, MIN + 10.0, MIN + 10.0);
        translate(&mut r, &PointF64::of(MIN, MIN));
        assert_eq!(r, RectF64::of(MIN, MIN, MIN + 10.0, MIN + 10.0));
    }

    #[test]
    fn translate_max_bounds_big_delta() {
        let mut r = RectF64::of(MAX - 10.0, MAX - 10.0, MAX, MAX);
        translate(&mut r, &PointF64::of(MAX, MAX));
        assert_eq!(r, RectF64::of(MAX - 10.0, MAX - 10.0, MAX, MAX));
    }

    #[test]
    fn translate_min_bounds_big_rect_big_delta() {
        let mut r = RectF64::of(MIN + 1.0, MIN + 1.0, MAX, MAX);
        translate(&mut r, &PointF64::of(MIN, MIN));
        assert_eq!(r, RectF64::of(MIN, MIN, MAX - 1.0, MAX - 1.0));
    }

    #[test]
    fn translate_max_bounds_big_rect_big_delta() {
        let mut r = RectF64::of(MIN, MIN, MAX - 1.0, MAX - 1.0);
        translate(&mut r, &PointF64::of(MAX, MAX));
        assert_eq!(r, RectF64::of(MIN + 1.0, MIN + 1.0, MAX, MAX));
    }
}
