use crate::cartesian::point::point_f64;

#[derive(PartialEq, Debug, Clone)]
pub struct RectF64 {
    pub min: point_f64::PointF64,
    pub max: point_f64::PointF64,
}

impl RectF64 {
    pub fn of(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        RectF64 {
            min: point_f64::PointF64 { x: x1, y: y1 },
            max: point_f64::PointF64 { x: x2, y: y2 },
        }
    }
}

impl std::fmt::Display for RectF64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.min.to_string(), self.max.to_string())
    }
}

pub fn delta_x(r: &RectF64) -> f64 {
    point_f64::delta_x(&r.min, &r.max)
}

pub fn delta_y(r: &RectF64) -> f64 {
    point_f64::delta_y(&r.min, &r.max)
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_f64::PointF64;

    use super::{RectF64, delta_x, delta_y};

    #[test]
    fn rect_f64() {
        assert_eq!(
            RectF64::of(-9_007_199_254_740_992.0, -0.0, 0.0, 9_007_199_254_740_991.0),
            RectF64 {
                min: PointF64 { x: -9_007_199_254_740_992.0, y: -0.0 },
                max: PointF64 { x: 0.0, y: 9_007_199_254_740_991.0 }
            }
        );
        assert_eq!(
            RectF64::of(-9_007_199_254_740_992.0, -0.0, 0.0, 9_007_199_254_740_991.0).to_string(),
            "((-9007199254740992, -0), (0, 9007199254740991))"
        );
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&RectF64::of(0.0, 0.0, 0.0, 9_007_199_254_740_991.0)), 0.0);
        assert_eq!(
            delta_x(&RectF64::of(0.0, -4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0)),
            0.0
        );
        assert_eq!(
            delta_x(&RectF64::of(0.0, 0.0, 9_007_199_254_740_991.0, 0.0)),
            9_007_199_254_740_991.0
        );
        assert_eq!(
            delta_x(&RectF64::of(-4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0, 0.0)),
            9_007_199_254_740_991.0
        );
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&RectF64::of(0.0, 0.0, 9_007_199_254_740_991.0, 0.0)), 0.0);
        assert_eq!(
            delta_y(&RectF64::of(-4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0, 0.0)),
            0.0
        );
        assert_eq!(
            delta_y(&RectF64::of(0.0, 0.0, 0.0, 9_007_199_254_740_991.0)),
            9_007_199_254_740_991.0
        );
        assert_eq!(
            delta_y(&RectF64::of(0.0, -4_503_599_627_370_496.0, 0.0, 4_503_599_627_370_495.0)),
            9_007_199_254_740_991.0
        );
    }
}
