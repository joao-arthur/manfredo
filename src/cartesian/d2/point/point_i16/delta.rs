use crate::cartesian::d2::point::{point_i16::Point, point_u16};

pub fn delta_x(p1: &Point, p2: &Point) -> u16 {
    (i32::from(p2.x) - i32::from(p1.x)).unsigned_abs() as u16
}

pub fn delta_y(p1: &Point, p2: &Point) -> u16 {
    (i32::from(p2.y) - i32::from(p1.y)).unsigned_abs() as u16
}

pub fn delta(p1: &Point, p2: &Point) -> point_u16::Point {
    point_u16::Point { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{delta, delta_x, delta_y};
    use crate::cartesian::d2::point::{point_i16::Point, point_u16};

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Point::of(0, i16::MIN), &Point::of(0, i16::MAX)), 0);
        assert_eq!(delta_x(&Point::of(i16::MIN, 0), &Point::of(i16::MAX, 0)), u16::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&Point::of(i16::MIN, 0), &Point::of(i16::MAX, 0)), 0);
        assert_eq!(delta_y(&Point::of(0, i16::MIN), &Point::of(0, i16::MAX)), u16::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::of(0, 0), &Point::of(0, 0)), point_u16::Point::min());
        assert_eq!(delta(&Point::min(), &Point::max()), point_u16::Point::max());
    }

    #[test]
    fn delta_min() {
        let p = Point::min();
        assert_eq!(delta(&p, &Point::min()), point_u16::Point::min());
        assert_eq!(delta(&p, &Point::of(i16::MIN, i16::MIN + 1)), point_u16::Point::of(0, 1));
        assert_eq!(delta(&p, &Point::of(i16::MIN, i16::MIN + 2)), point_u16::Point::of(0, 2));

        assert_eq!(delta(&p, &Point::of(i16::MIN + 1, i16::MIN)), point_u16::Point::of(1, 0));
        assert_eq!(delta(&p, &Point::of(i16::MIN + 1, i16::MIN + 1)), point_u16::Point::of(1, 1));
        assert_eq!(delta(&p, &Point::of(i16::MIN + 1, i16::MIN + 2)), point_u16::Point::of(1, 2));

        assert_eq!(delta(&p, &Point::of(i16::MIN + 2, i16::MIN)), point_u16::Point::of(2, 0));
        assert_eq!(delta(&p, &Point::of(i16::MIN + 2, i16::MIN + 1)), point_u16::Point::of(2, 1));
        assert_eq!(delta(&p, &Point::of(i16::MIN + 2, i16::MIN + 2)), point_u16::Point::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p = Point::of(i16::MAX - 2, i16::MAX - 2);
        assert_eq!(delta(&p, &Point::of(i16::MAX - 2, i16::MAX - 2)), point_u16::Point::min());
        assert_eq!(delta(&p, &Point::of(i16::MAX - 2, i16::MAX - 1)), point_u16::Point::of(0, 1));
        assert_eq!(delta(&p, &Point::of(i16::MAX - 2, i16::MAX)), point_u16::Point::of(0, 2));

        assert_eq!(delta(&p, &Point::of(i16::MAX - 1, i16::MAX - 2)), point_u16::Point::of(1, 0));
        assert_eq!(delta(&p, &Point::of(i16::MAX - 1, i16::MAX - 1)), point_u16::Point::of(1, 1));
        assert_eq!(delta(&p, &Point::of(i16::MAX - 1, i16::MAX)), point_u16::Point::of(1, 2));

        assert_eq!(delta(&p, &Point::of(i16::MAX, i16::MAX - 2)), point_u16::Point::of(2, 0));
        assert_eq!(delta(&p, &Point::of(i16::MAX, i16::MAX - 1)), point_u16::Point::of(2, 1));
        assert_eq!(delta(&p, &Point::max()), point_u16::Point::of(2, 2));
    }
}
