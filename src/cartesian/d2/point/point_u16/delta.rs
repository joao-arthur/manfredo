use super::Point;

pub fn delta_x(p1: &Point, p2: &Point) -> u16 {
    p2.x - p1.x
}

pub fn delta_y(p1: &Point, p2: &Point) -> u16 {
    p2.y - p1.y
}

pub fn delta_min(p1: &Point, p2: &Point) -> u16 {
    delta_x(p1, p2).min(delta_y(p1, p2))
}

pub fn delta_max(p1: &Point, p2: &Point) -> u16 {
    delta_x(p1, p2).max(delta_y(p1, p2))
}

pub fn delta(p1: &Point, p2: &Point) -> Point {
    Point { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{delta, delta_max, delta_min, delta_x, delta_y};
    use crate::cartesian::{d1::point::point_u16::MAX, d2::point::point_u16::Point};

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Point::min(), &Point::of(MAX, 0)), MAX);
        assert_eq!(delta_x(&Point::min(), &Point::of(0, MAX)), 0);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&Point::min(), &Point::of(MAX, 0)), 0);
        assert_eq!(delta_y(&Point::min(), &Point::of(0, MAX)), MAX);
    }

    #[test]
    fn test_delta_max() {
        assert_eq!(delta_max(&Point::of(0, 1), &Point::of(10, 10)), 10);
        assert_eq!(delta_max(&Point::of(1, 0), &Point::of(9, 9)), 9);
    }

    #[test]
    fn test_delta_min() {
        assert_eq!(delta_min(&Point::of(0, 1), &Point::of(10, 10)), 9);
        assert_eq!(delta_min(&Point::of(1, 0), &Point::of(9, 9)), 8);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::min(), &Point::min()), Point::min());
        assert_eq!(delta(&Point::min(), &Point::max()), Point::max());
    }
}
