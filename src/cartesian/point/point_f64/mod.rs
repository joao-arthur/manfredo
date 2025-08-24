use super::point_f32::PointF32;

pub mod add;

pub const MIN: f64 = -9_007_199_254_740_992.0;
pub const MAX: f64 = 9_007_199_254_740_991.0;

#[derive(PartialEq, Debug, Clone)]
pub struct PointF64 {
    pub x: f64,
    pub y: f64,
}

impl PointF64 {
    pub fn of(x: f64, y: f64) -> Self {
        PointF64 { x, y }
    }

    pub fn min() -> Self {
        PointF64 { x: MIN, y: MIN }
    }

    pub fn max() -> Self {
        PointF64 { x: MAX, y: MAX }
    }
}

impl From<PointF32> for PointF64 {
    fn from(p: PointF32) -> Self {
        PointF64 { x: p.x.into(), y: p.y.into() }
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

pub fn delta(p1: &PointF64, p2: &PointF64) -> PointF64 {
    PointF64 { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{MAX, MIN, PointF64, delta, delta_x, delta_y};
    use crate::cartesian::point::point_f32::PointF32;

    #[test]
    fn point_f64() {
        assert_eq!(PointF64::of(MIN, MAX), PointF64 { x: MIN, y: MAX });
        assert_eq!(PointF64::min(), PointF64 { x: MIN, y: MIN });
        assert_eq!(PointF64::max(), PointF64 { x: MAX, y: MAX });
    }

    #[test]
    fn from() {
        assert_eq!(PointF64::from(PointF32::min()), PointF64 { x: -16777216.0, y: -16777216.0 });
        assert_eq!(PointF64::from(PointF32::max()), PointF64 { x: 16777215.0, y: 16777215.0 });
    }

    #[test]
    fn to_string() {
        assert_eq!(PointF64::of(MIN, MAX).to_string(), "(-9007199254740992, 9007199254740991)");
        assert_eq!(PointF64::min().to_string(), "(-9007199254740992, -9007199254740992)");
        assert_eq!(PointF64::max().to_string(), "(9007199254740991, 9007199254740991)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&PointF64::of(0.0, 0.0), &PointF64::of(0.0, MAX)), 0.0);
        assert_eq!(delta_x(&PointF64::of(0.0, -4_503_599_627_370_496.0), &PointF64::of(0.0, 4_503_599_627_370_495.0)), 0.0);
        assert_eq!(delta_x(&PointF64::of(0.0, 0.0), &PointF64::of(MAX, 0.0)), MAX);
        assert_eq!(delta_x(&PointF64::of(-4_503_599_627_370_496.0, 0.0), &PointF64::of(4_503_599_627_370_495.0, 0.0)), MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&PointF64::of(0.0, 0.0), &PointF64::of(MAX, 0.0)), 0.0);
        assert_eq!(delta_y(&PointF64::of(-4_503_599_627_370_496.0, 0.0), &PointF64::of(4_503_599_627_370_495.0, 0.0)), 0.0);
        assert_eq!(delta_y(&PointF64::of(0.0, 0.0), &PointF64::of(0.0, MAX)), MAX);
        assert_eq!(delta_y(&PointF64::of(0.0, -4_503_599_627_370_496.0), &PointF64::of(0.0, 4_503_599_627_370_495.0)), MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&PointF64::of(0.0, 0.0), &PointF64::of(0.0, 0.0)), PointF64::of(0.0, 0.0));
        assert_eq!(
            delta(&PointF64::of(-4_503_599_627_370_496.0, -4_503_599_627_370_496.0), &PointF64::of(4_503_599_627_370_495.0, 4_503_599_627_370_495.0)),
            PointF64::of(MAX, MAX)
        );
    }

    #[test]
    fn delta_min() {
        let p = PointF64::of(-4_503_599_627_370_496.0, -4_503_599_627_370_496.0);
        assert_eq!(delta(&p, &PointF64::of(-4_503_599_627_370_496.0, -4_503_599_627_370_496.0)), PointF64::of(0.0, 0.0));
        assert_eq!(delta(&p, &PointF64::of(-4_503_599_627_370_496.0, -4_503_599_627_370_495.0)), PointF64::of(0.0, 1.0));
        assert_eq!(delta(&p, &PointF64::of(-4_503_599_627_370_496.0, -4_503_599_627_370_494.0)), PointF64::of(0.0, 2.0));

        assert_eq!(delta(&p, &PointF64::of(-4_503_599_627_370_495.0, -4_503_599_627_370_496.0)), PointF64::of(1.0, 0.0));
        assert_eq!(delta(&p, &PointF64::of(-4_503_599_627_370_495.0, -4_503_599_627_370_495.0)), PointF64::of(1.0, 1.0));
        assert_eq!(delta(&p, &PointF64::of(-4_503_599_627_370_495.0, -4_503_599_627_370_494.0)), PointF64::of(1.0, 2.0));

        assert_eq!(delta(&p, &PointF64::of(-4_503_599_627_370_494.0, -4_503_599_627_370_496.0)), PointF64::of(2.0, 0.0));
        assert_eq!(delta(&p, &PointF64::of(-4_503_599_627_370_494.0, -4_503_599_627_370_495.0)), PointF64::of(2.0, 1.0));
        assert_eq!(delta(&p, &PointF64::of(-4_503_599_627_370_494.0, -4_503_599_627_370_494.0)), PointF64::of(2.0, 2.0));
    }

    #[test]
    fn delta_max() {
        let p = PointF64::of(4_503_599_627_370_493.0, 4_503_599_627_370_493.0);
        assert_eq!(delta(&p, &PointF64::of(4_503_599_627_370_493.0, 4_503_599_627_370_493.0)), PointF64::of(0.0, 0.0));
        assert_eq!(delta(&p, &PointF64::of(4_503_599_627_370_493.0, 4_503_599_627_370_494.0)), PointF64::of(0.0, 1.0));
        assert_eq!(delta(&p, &PointF64::of(4_503_599_627_370_493.0, 4_503_599_627_370_495.0)), PointF64::of(0.0, 2.0));

        assert_eq!(delta(&p, &PointF64::of(4_503_599_627_370_494.0, 4_503_599_627_370_493.0)), PointF64::of(1.0, 0.0));
        assert_eq!(delta(&p, &PointF64::of(4_503_599_627_370_494.0, 4_503_599_627_370_494.0)), PointF64::of(1.0, 1.0));
        assert_eq!(delta(&p, &PointF64::of(4_503_599_627_370_494.0, 4_503_599_627_370_495.0)), PointF64::of(1.0, 2.0));

        assert_eq!(delta(&p, &PointF64::of(4_503_599_627_370_495.0, 4_503_599_627_370_493.0)), PointF64::of(2.0, 0.0));
        assert_eq!(delta(&p, &PointF64::of(4_503_599_627_370_495.0, 4_503_599_627_370_494.0)), PointF64::of(2.0, 1.0));
        assert_eq!(delta(&p, &PointF64::of(4_503_599_627_370_495.0, 4_503_599_627_370_495.0)), PointF64::of(2.0, 2.0));
    }
}
