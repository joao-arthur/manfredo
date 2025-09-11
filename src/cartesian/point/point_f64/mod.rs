use super::point_f32;

mod add;

pub use self::add::{checked_add, checked_add_assign, saturating_add, saturating_add_assign, try_checked_add, try_checked_add_assign, wrapping_add, wrapping_add_assign};

pub const MIN: f64 = -9_007_199_254_740_992.0;
pub const MAX: f64 = 9_007_199_254_740_991.0;

#[derive(PartialEq, Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn of(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn min() -> Self {
        Point { x: MIN, y: MIN }
    }

    pub fn max() -> Self {
        Point { x: MAX, y: MAX }
    }
}

impl From<point_f32::Point> for Point {
    fn from(p: point_f32::Point) -> Self {
        Point { x: p.x.into(), y: p.y.into() }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn delta_x(p1: &Point, p2: &Point) -> f64 {
    p2.x - p1.x
}

pub fn delta_y(p1: &Point, p2: &Point) -> f64 {
    p2.y - p1.y
}

pub fn delta(p1: &Point, p2: &Point) -> Point {
    Point { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{MAX, MIN, Point, delta, delta_x, delta_y};
    use crate::cartesian::point::point_f32;

    #[test]
    fn point_f64() {
        assert_eq!(Point::of(MIN, MAX), Point { x: MIN, y: MAX });
        assert_eq!(Point::min(), Point { x: MIN, y: MIN });
        assert_eq!(Point::max(), Point { x: MAX, y: MAX });
    }

    #[test]
    fn from() {
        assert_eq!(Point::from(point_f32::Point::min()), Point { x: -16777216.0, y: -16777216.0 });
        assert_eq!(Point::from(point_f32::Point::max()), Point { x: 16777215.0, y: 16777215.0 });
    }

    #[test]
    fn to_string() {
        assert_eq!(Point::of(MIN, MAX).to_string(), "(-9007199254740992, 9007199254740991)");
        assert_eq!(Point::min().to_string(), "(-9007199254740992, -9007199254740992)");
        assert_eq!(Point::max().to_string(), "(9007199254740991, 9007199254740991)");
    }

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Point::of(0.0, 0.0), &Point::of(0.0, MAX)), 0.0);
        assert_eq!(delta_x(&Point::of(0.0, -4_503_599_627_370_496.0), &Point::of(0.0, 4_503_599_627_370_495.0)), 0.0);
        assert_eq!(delta_x(&Point::of(0.0, 0.0), &Point::of(MAX, 0.0)), MAX);
        assert_eq!(delta_x(&Point::of(-4_503_599_627_370_496.0, 0.0), &Point::of(4_503_599_627_370_495.0, 0.0)), MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&Point::of(0.0, 0.0), &Point::of(MAX, 0.0)), 0.0);
        assert_eq!(delta_y(&Point::of(-4_503_599_627_370_496.0, 0.0), &Point::of(4_503_599_627_370_495.0, 0.0)), 0.0);
        assert_eq!(delta_y(&Point::of(0.0, 0.0), &Point::of(0.0, MAX)), MAX);
        assert_eq!(delta_y(&Point::of(0.0, -4_503_599_627_370_496.0), &Point::of(0.0, 4_503_599_627_370_495.0)), MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::of(0.0, 0.0), &Point::of(0.0, 0.0)), Point::of(0.0, 0.0));
        assert_eq!(delta(&Point::of(-4_503_599_627_370_496.0, -4_503_599_627_370_496.0), &Point::of(4_503_599_627_370_495.0, 4_503_599_627_370_495.0)), Point::of(MAX, MAX));
    }

    #[test]
    fn delta_min() {
        let p = Point::of(-4_503_599_627_370_496.0, -4_503_599_627_370_496.0);
        assert_eq!(delta(&p, &Point::of(-4_503_599_627_370_496.0, -4_503_599_627_370_496.0)), Point::of(0.0, 0.0));
        assert_eq!(delta(&p, &Point::of(-4_503_599_627_370_496.0, -4_503_599_627_370_495.0)), Point::of(0.0, 1.0));
        assert_eq!(delta(&p, &Point::of(-4_503_599_627_370_496.0, -4_503_599_627_370_494.0)), Point::of(0.0, 2.0));

        assert_eq!(delta(&p, &Point::of(-4_503_599_627_370_495.0, -4_503_599_627_370_496.0)), Point::of(1.0, 0.0));
        assert_eq!(delta(&p, &Point::of(-4_503_599_627_370_495.0, -4_503_599_627_370_495.0)), Point::of(1.0, 1.0));
        assert_eq!(delta(&p, &Point::of(-4_503_599_627_370_495.0, -4_503_599_627_370_494.0)), Point::of(1.0, 2.0));

        assert_eq!(delta(&p, &Point::of(-4_503_599_627_370_494.0, -4_503_599_627_370_496.0)), Point::of(2.0, 0.0));
        assert_eq!(delta(&p, &Point::of(-4_503_599_627_370_494.0, -4_503_599_627_370_495.0)), Point::of(2.0, 1.0));
        assert_eq!(delta(&p, &Point::of(-4_503_599_627_370_494.0, -4_503_599_627_370_494.0)), Point::of(2.0, 2.0));
    }

    #[test]
    fn delta_max() {
        let p = Point::of(4_503_599_627_370_493.0, 4_503_599_627_370_493.0);
        assert_eq!(delta(&p, &Point::of(4_503_599_627_370_493.0, 4_503_599_627_370_493.0)), Point::of(0.0, 0.0));
        assert_eq!(delta(&p, &Point::of(4_503_599_627_370_493.0, 4_503_599_627_370_494.0)), Point::of(0.0, 1.0));
        assert_eq!(delta(&p, &Point::of(4_503_599_627_370_493.0, 4_503_599_627_370_495.0)), Point::of(0.0, 2.0));

        assert_eq!(delta(&p, &Point::of(4_503_599_627_370_494.0, 4_503_599_627_370_493.0)), Point::of(1.0, 0.0));
        assert_eq!(delta(&p, &Point::of(4_503_599_627_370_494.0, 4_503_599_627_370_494.0)), Point::of(1.0, 1.0));
        assert_eq!(delta(&p, &Point::of(4_503_599_627_370_494.0, 4_503_599_627_370_495.0)), Point::of(1.0, 2.0));

        assert_eq!(delta(&p, &Point::of(4_503_599_627_370_495.0, 4_503_599_627_370_493.0)), Point::of(2.0, 0.0));
        assert_eq!(delta(&p, &Point::of(4_503_599_627_370_495.0, 4_503_599_627_370_494.0)), Point::of(2.0, 1.0));
        assert_eq!(delta(&p, &Point::of(4_503_599_627_370_495.0, 4_503_599_627_370_495.0)), Point::of(2.0, 2.0));
    }
}
