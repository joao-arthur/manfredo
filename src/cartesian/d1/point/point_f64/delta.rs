use super::Point;

pub fn delta(p1: &Point, p2: &Point) -> f64 {
    p2.x - p1.x
}

#[cfg(test)]
mod tests {
    use super::delta;
    use crate::cartesian::d1::point::point_f64::{MAX, Point};

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::of(0.0), &Point::of(0.0)), 0.0);
        assert_eq!(delta(&Point::of(0.0), &Point::max()), MAX);
        assert_eq!(delta(&Point::min(), &Point::of(0.0)), MAX + 1.0);
        assert_eq!(delta(&Point::of(-4_503_599_627_370_496.0), &Point::of(4_503_599_627_370_495.0)), MAX);
    }
}
