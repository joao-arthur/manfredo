#[derive(PartialEq, Debug, Clone)]
pub struct PointF64 {
    pub x: f64,
    pub y: f64,
}

impl PointF64 {
    pub fn of(x: f64, y: f64) -> Self {
        PointF64 { x, y }
    }
}

impl std::fmt::Display for PointF64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &PointF64, p2: &PointF64) -> f64 {
    p2.x - p1.x
}

pub fn delta_y(p1: &PointF64, p2: &PointF64) -> f64 {
    p2.y - p1.y
}

#[cfg(test)]
mod tests {
    use super::{PointF64, delta_x, delta_y};

    #[test]
    fn point_f64() {
        assert_eq!(
            PointF64::of(-9_007_199_254_740_992.0, 9_007_199_254_740_991.0),
            PointF64 { x: -9_007_199_254_740_992.0, y: 9_007_199_254_740_991.0 }
        );
        assert_eq!(PointF64::of(-9_007_199_254_740_992.0, 9_007_199_254_740_991.0).to_string(), "(-9007199254740992, 9007199254740991)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointF64::of(0.0, 0.0), &PointF64::of(0.0, 9_007_199_254_740_991.0)), 0.0);
        assert_eq!(delta_x(&PointF64::of(0.0, 0.0), &PointF64::of(9_007_199_254_740_991.0, 0.0)), 9_007_199_254_740_991.0);
        assert_eq!(delta_x(&PointF64::of(-4_503_599_627_370_496.0, 0.0), &PointF64::of(4_503_599_627_370_495.0, 0.0)), 9_007_199_254_740_991.0);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointF64::of(0.0, 0.0), &PointF64::of(9_007_199_254_740_991.0, 0.0)), 0.0);
        assert_eq!(delta_y(&PointF64::of(0.0, 0.0), &PointF64::of(0.0, 9_007_199_254_740_991.0)), 9_007_199_254_740_991.0);
        assert_eq!(delta_y(&PointF64::of(0.0, -4_503_599_627_370_496.0), &PointF64::of(0.0, 4_503_599_627_370_495.0)), 9_007_199_254_740_991.0);
    }
}
