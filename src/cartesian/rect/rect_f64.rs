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
        write!(f, "({}, {})", self.min.to_string(), self.max.to_string())
    }
}

const ART_MIN: f64 = -9_007_199_254_740_992.0;
const ART_MAX: f64 = 9_007_199_254_740_991.0;

pub fn delta_x(r: &RectF64) -> f64 {
    point_f64::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectF64) -> f64 {
    point_f64::delta_y(&r.min, &r.max)
}

pub fn inflate(r: &mut RectF64) {
    let is_min_x = r.min.x == ART_MIN;
    let is_max_x = r.max.x == ART_MAX;
    let is_min_y = r.min.y == ART_MIN;
    let is_max_y = r.max.y == ART_MAX;
    if (is_min_x && is_max_x) || (is_min_y && is_max_y) {
        return;
    }
    let min_x_modifier = 1.0 - f64::from(is_min_x) + f64::from(is_max_x);
    let max_x_modifier = 1.0 + f64::from(is_min_x) - f64::from(is_max_x);
    let min_y_modifier = 1.0 - f64::from(is_min_y) + f64::from(is_max_y);
    let max_y_modifier = 1.0 + f64::from(is_min_y) - f64::from(is_max_y);
    r.min.x = (r.min.x - min_x_modifier).max(ART_MIN);
    r.max.x = (r.max.x + max_x_modifier).min(ART_MAX);
    r.min.y = (r.min.y - min_y_modifier).max(ART_MIN);
    r.max.y = (r.max.y + max_y_modifier).min(ART_MAX);
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_f64::PointF64;

    use super::{ART_MAX, ART_MIN, RectF64, delta_x, delta_y, inflate};

    #[test]
    fn rect_f64() {
        assert_eq!(RectF64::of(ART_MIN, -0.0, 0.0, ART_MAX), RectF64 { min: PointF64 { x: ART_MIN, y: -0.0 }, max: PointF64 { x: 0.0, y: ART_MAX } });
        assert_eq!(RectF64::of(ART_MIN, -0.0, 0.0, ART_MAX).to_string(), "((-9007199254740992, -0), (0, 9007199254740991))");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectF64::of(0.0, 0.0, 0.0, ART_MAX)), 0.0);
        assert_eq!(delta_x(&RectF64::of(0.0, -4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0)), 0.0);
        assert_eq!(delta_x(&RectF64::of(0.0, 0.0, ART_MAX, 0.0)), ART_MAX);
        assert_eq!(delta_x(&RectF64::of(-4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0, 0.0)), ART_MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectF64::of(0.0, 0.0, ART_MAX, 0.0)), 0.0);
        assert_eq!(delta_y(&RectF64::of(-4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0, 0.0)), 0.0);
        assert_eq!(delta_y(&RectF64::of(0.0, 0.0, 0.0, ART_MAX)), ART_MAX);
        assert_eq!(delta_y(&RectF64::of(0.0, -4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0)), ART_MAX);
    }

    #[test]
    fn inflate_min_bounds() {
        let mut r = RectF64::of(-9_007_199_254_740_985.0, -9_007_199_254_740_990.0, 4.0, 13.0);
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-9_007_199_254_740_986.0, -9_007_199_254_740_991.0, 5.0, 14.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-9_007_199_254_740_987.0, ART_MIN, 6.0, 15.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-9_007_199_254_740_988.0, ART_MIN, 7.0, 17.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-9_007_199_254_740_989.0, ART_MIN, 8.0, 19.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-9_007_199_254_740_990.0, ART_MIN, 9.0, 21.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-9_007_199_254_740_991.0, ART_MIN, 10.0, 23.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(ART_MIN, ART_MIN, 11.0, 25.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(ART_MIN, ART_MIN, 13.0, 27.0));
    }

    #[test]
    fn inflate_max_bounds() {
        let mut r = RectF64::of(-100.0, 30.0, 9_007_199_254_740_986.0, 9_007_199_254_740_988.0);
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-101.0, 29.0, 9_007_199_254_740_987.0, 9_007_199_254_740_989.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-102.0, 28.0, 9_007_199_254_740_988.0, 9_007_199_254_740_990.0));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-103.0, 27.0, 9_007_199_254_740_989.0, ART_MAX));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-104.0, 25.0, 9_007_199_254_740_990.0, ART_MAX));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-105.0, 23.0, ART_MAX, ART_MAX));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-107.0, 21.0, ART_MAX, ART_MAX));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-109.0, 19.0, ART_MAX, ART_MAX));
        inflate(&mut r);
        assert_eq!(r, RectF64::of(-111.0, 17.0, ART_MAX, ART_MAX));
    }

    #[test]
    fn inflate_almost_min_bounds() {
        let mut r = RectF64::of(-9_007_199_254_740_991.0, -9_007_199_254_740_991.0, ART_MAX, ART_MAX);
        inflate(&mut r);
        assert_eq!(r, RectF64::of(ART_MIN, ART_MIN, ART_MAX, ART_MAX));
    }

    #[test]
    fn inflate_almost_max_bounds() {
        let mut r = RectF64::of(ART_MIN, ART_MIN, 9_007_199_254_740_990.0, 9_007_199_254_740_990.0);
        inflate(&mut r);
        assert_eq!(r, RectF64::of(ART_MIN, ART_MIN, ART_MAX, ART_MAX));
    }

    #[test]
    fn inflate_max_width() {
        let mut r = RectF64::of(ART_MIN, 10.0, ART_MAX, 50.0);
        inflate(&mut r);
        assert_eq!(r, RectF64::of(ART_MIN, 10.0, ART_MAX, 50.0));
    }

    #[test]
    fn inflate_max_height() {
        let mut r = RectF64::of(10.0, ART_MIN, 50.0, ART_MAX);
        inflate(&mut r);
        assert_eq!(r, RectF64::of(10.0, ART_MIN, 50.0, ART_MAX));
    }
}
