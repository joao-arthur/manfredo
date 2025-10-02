use crate::cartesian::d2::point::point_f64::{MAX, MIN, Point};

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
    use super::{delta, delta_x, delta_y};
    use crate::cartesian::d2::point::point_f64::{MAX, MIN, Point};

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
