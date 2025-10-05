use crate::cartesian::d2::point::{point_i32::Point, point_u32};

pub fn delta_x(p1: &Point, p2: &Point) -> u32 {
    (i64::from(p2.x) - i64::from(p1.x)).unsigned_abs() as u32
}

pub fn delta_y(p1: &Point, p2: &Point) -> u32 {
    (i64::from(p2.y) - i64::from(p1.y)).unsigned_abs() as u32
}

pub fn delta(p1: &Point, p2: &Point) -> point_u32::Point {
    point_u32::Point { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{delta, delta_x, delta_y};
    use crate::cartesian::d2::point::{point_i32::Point, point_u32};

    const MIN: i32 = i32::MIN;
    const MAX: i32 = i32::MAX;

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Point::of(0, MIN), &Point::of(0, MAX)), 0);
        assert_eq!(delta_x(&Point::of(MIN, 0), &Point::of(MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&Point::of(MIN, 0), &Point::of(MAX, 0)), 0);
        assert_eq!(delta_y(&Point::of(0, MIN), &Point::of(0, MAX)), u32::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::of(0, 0), &Point::of(0, 0)), point_u32::Point::min());
        assert_eq!(delta(&Point::min(), &Point::max()), point_u32::Point::max());
    }

    #[test]
    fn delta_min() {
        let p = Point::min();
        assert_eq!(delta(&p, &Point::min()), point_u32::Point::min());
        assert_eq!(delta(&p, &Point::of(MIN, MIN + 1)), point_u32::Point::of(0, 1));
        assert_eq!(delta(&p, &Point::of(MIN, MIN + 2)), point_u32::Point::of(0, 2));

        assert_eq!(delta(&p, &Point::of(MIN + 1, MIN)), point_u32::Point::of(1, 0));
        assert_eq!(delta(&p, &Point::of(MIN + 1, MIN + 1)), point_u32::Point::of(1, 1));
        assert_eq!(delta(&p, &Point::of(MIN + 1, MIN + 2)), point_u32::Point::of(1, 2));

        assert_eq!(delta(&p, &Point::of(MIN + 2, MIN)), point_u32::Point::of(2, 0));
        assert_eq!(delta(&p, &Point::of(MIN + 2, MIN + 1)), point_u32::Point::of(2, 1));
        assert_eq!(delta(&p, &Point::of(MIN + 2, MIN + 2)), point_u32::Point::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p = Point::of(MAX - 2, MAX - 2);
        assert_eq!(delta(&p, &Point::of(MAX - 2, MAX - 2)), point_u32::Point::min());
        assert_eq!(delta(&p, &Point::of(MAX - 2, MAX - 1)), point_u32::Point::of(0, 1));
        assert_eq!(delta(&p, &Point::of(MAX - 2, MAX)), point_u32::Point::of(0, 2));

        assert_eq!(delta(&p, &Point::of(MAX - 1, MAX - 2)), point_u32::Point::of(1, 0));
        assert_eq!(delta(&p, &Point::of(MAX - 1, MAX - 1)), point_u32::Point::of(1, 1));
        assert_eq!(delta(&p, &Point::of(MAX - 1, MAX)), point_u32::Point::of(1, 2));

        assert_eq!(delta(&p, &Point::of(MAX, MAX - 2)), point_u32::Point::of(2, 0));
        assert_eq!(delta(&p, &Point::of(MAX, MAX - 1)), point_u32::Point::of(2, 1));
        assert_eq!(delta(&p, &Point::max()), point_u32::Point::of(2, 2));
    }
}
