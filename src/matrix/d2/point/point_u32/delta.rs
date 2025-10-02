use crate::matrix::d2::point::point_u32::Point;

pub fn delta_row(p1: &Point, p2: &Point) -> u32 {
    p2.row - p1.row
}

pub fn delta_col(p1: &Point, p2: &Point) -> u32 {
    p2.col - p1.col
}

pub fn delta(p1: &Point, p2: &Point) -> Point {
    Point { row: delta_row(p1, p2), col: delta_col(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{delta, delta_col, delta_row};
    use crate::matrix::d2::point::point_u32::Point;

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&Point::min(), &Point::of(0, u32::MAX)), 0);
        assert_eq!(delta_row(&Point::min(), &Point::of(u32::MAX, 0)), u32::MAX);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&Point::min(), &Point::of(u32::MAX, 0)), 0);
        assert_eq!(delta_col(&Point::min(), &Point::of(0, u32::MAX)), u32::MAX);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::min(), &Point::min()), Point::min());
        assert_eq!(delta(&Point::min(), &Point::max()), Point::max());
    }

    #[test]
    fn delta_min() {
        let p = Point::min();
        assert_eq!(delta(&p, &Point::min()), Point::min());
        assert_eq!(delta(&p, &Point::of(0, 1)), Point::of(0, 1));
        assert_eq!(delta(&p, &Point::of(0, 2)), Point::of(0, 2));

        assert_eq!(delta(&p, &Point::of(1, 0)), Point::of(1, 0));
        assert_eq!(delta(&p, &Point::of(1, 1)), Point::of(1, 1));
        assert_eq!(delta(&p, &Point::of(1, 2)), Point::of(1, 2));

        assert_eq!(delta(&p, &Point::of(2, 0)), Point::of(2, 0));
        assert_eq!(delta(&p, &Point::of(2, 1)), Point::of(2, 1));
        assert_eq!(delta(&p, &Point::of(2, 2)), Point::of(2, 2));
    }

    #[test]
    fn delta_max() {
        let p = Point::of(u32::MAX - 2, u32::MAX - 2);
        assert_eq!(delta(&p, &Point::of(u32::MAX - 2, u32::MAX - 2)), Point::min());
        assert_eq!(delta(&p, &Point::of(u32::MAX - 2, u32::MAX - 1)), Point::of(0, 1));
        assert_eq!(delta(&p, &Point::of(u32::MAX - 2, u32::MAX)), Point::of(0, 2));

        assert_eq!(delta(&p, &Point::of(u32::MAX - 1, u32::MAX - 2)), Point::of(1, 0));
        assert_eq!(delta(&p, &Point::of(u32::MAX - 1, u32::MAX - 1)), Point::of(1, 1));
        assert_eq!(delta(&p, &Point::of(u32::MAX - 1, u32::MAX)), Point::of(1, 2));

        assert_eq!(delta(&p, &Point::of(u32::MAX, u32::MAX - 2)), Point::of(2, 0));
        assert_eq!(delta(&p, &Point::of(u32::MAX, u32::MAX - 1)), Point::of(2, 1));
        assert_eq!(delta(&p, &Point::max()), Point::of(2, 2));
    }
}
