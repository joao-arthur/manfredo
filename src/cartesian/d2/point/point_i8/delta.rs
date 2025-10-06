use crate::cartesian::d2::point::{point_i8::Point, point_u8};

pub fn delta_x(p1: &Point, p2: &Point) -> u8 {
    (i16::from(p2.x) - i16::from(p1.x)).unsigned_abs() as u8
}

pub fn delta_y(p1: &Point, p2: &Point) -> u8 {
    (i16::from(p2.y) - i16::from(p1.y)).unsigned_abs() as u8
}

pub fn delta_min(p1: &Point, p2: &Point) -> u8 {
    std::cmp::min(delta_x(p1, p2), delta_y(p1, p2))
}

pub fn delta_max(p1: &Point, p2: &Point) -> u8 {
    std::cmp::max(delta_x(p1, p2), delta_y(p1, p2))
}

pub fn delta(p1: &Point, p2: &Point) -> point_u8::Point {
    point_u8::Point { x: delta_x(p1, p2), y: delta_y(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{delta, delta_x, delta_y};
    use crate::cartesian::d2::point::{point_i8::Point, point_u8};

    const MIN: i8 = i8::MIN;
    const MAX: i8 = i8::MAX;

    #[test]
    fn test_delta_x() {
        assert_eq!(delta_x(&Point::of(0, MIN), &Point::of(0, MAX)), 0);
        assert_eq!(delta_x(&Point::of(MIN, 0), &Point::of(MAX, 0)), u8::MAX);
    }

    #[test]
    fn test_delta_y() {
        assert_eq!(delta_y(&Point::of(MIN, 0), &Point::of(MAX, 0)), 0);
        assert_eq!(delta_y(&Point::of(0, MIN), &Point::of(0, MAX)), u8::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::of(0, 0), &Point::of(0, 0)), point_u8::Point::min());
        assert_eq!(delta(&Point::min(), &Point::max()), point_u8::Point::max());
    }

    #[test]
    fn delta_min() {
        let p = Point::min();
        assert_eq!(delta(&p, &Point::min()), point_u8::Point::min());
        assert_eq!(delta(&p, &Point::of(MIN, MIN + 1)), point_u8::Point::of(0, 1));
        assert_eq!(delta(&p, &Point::of(MIN, MIN + 2)), point_u8::Point::of(0, 2));

        assert_eq!(delta(&p, &Point::of(MIN + 1, MIN)), point_u8::Point::of(1, 0));
        assert_eq!(delta(&p, &Point::of(MIN + 1, MIN + 1)), point_u8::Point::of(1, 1));
        assert_eq!(delta(&p, &Point::of(MIN + 1, MIN + 2)), point_u8::Point::of(1, 2));

        assert_eq!(delta(&p, &Point::of(MIN + 2, MIN)), point_u8::Point::of(2, 0));
        assert_eq!(delta(&p, &Point::of(MIN + 2, MIN + 1)), point_u8::Point::of(2, 1));
        assert_eq!(delta(&p, &Point::of(MIN + 2, MIN + 2)), point_u8::Point::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p = Point::of(MAX - 2, MAX - 2);
        assert_eq!(delta(&p, &Point::of(MAX - 2, MAX - 2)), point_u8::Point::min());
        assert_eq!(delta(&p, &Point::of(MAX - 2, MAX - 1)), point_u8::Point::of(0, 1));
        assert_eq!(delta(&p, &Point::of(MAX - 2, MAX)), point_u8::Point::of(0, 2));

        assert_eq!(delta(&p, &Point::of(MAX - 1, MAX - 2)), point_u8::Point::of(1, 0));
        assert_eq!(delta(&p, &Point::of(MAX - 1, MAX - 1)), point_u8::Point::of(1, 1));
        assert_eq!(delta(&p, &Point::of(MAX - 1, MAX)), point_u8::Point::of(1, 2));

        assert_eq!(delta(&p, &Point::of(MAX, MAX - 2)), point_u8::Point::of(2, 0));
        assert_eq!(delta(&p, &Point::of(MAX, MAX - 1)), point_u8::Point::of(2, 1));
        assert_eq!(delta(&p, &Point::max()), point_u8::Point::of(2, 2));
    }
}
