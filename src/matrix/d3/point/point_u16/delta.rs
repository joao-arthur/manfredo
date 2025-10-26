use super::Point;

pub fn delta_row(p1: &Point, p2: &Point) -> u16 {
    p2.row - p1.row
}

pub fn delta_col(p1: &Point, p2: &Point) -> u16 {
    p2.col - p1.col
}

pub fn delta_depth(p1: &Point, p2: &Point) -> u16 {
    p2.depth - p1.depth
}

pub fn delta_min(p1: &Point, p2: &Point) -> u16 {
    delta_row(p1, p2).min(delta_col(p1, p2)).min(delta_depth(p1, p2))
}

pub fn delta_max(p1: &Point, p2: &Point) -> u16 {
    delta_row(p1, p2).max(delta_col(p1, p2)).max(delta_depth(p1, p2))
}

pub fn delta(p1: &Point, p2: &Point) -> Point {
    Point { row: delta_row(p1, p2), col: delta_col(p1, p2), depth: delta_depth(p1, p2) }
}

#[cfg(test)]
mod tests {
    use super::{delta, delta_col, delta_max, delta_min, delta_row, delta_depth};
    use crate::matrix::{d1::point::point_u16::MAX, d3::point::point_u16::Point};

    #[test]
    fn test_delta_row() {
        assert_eq!(delta_row(&Point::min(), &Point::of(MAX, 0, 0)), MAX);
        assert_eq!(delta_row(&Point::min(), &Point::of(0, MAX, 0)), 0);
        assert_eq!(delta_row(&Point::min(), &Point::of(0, 0, MAX)), 0);
    }

    #[test]
    fn test_delta_col() {
        assert_eq!(delta_col(&Point::min(), &Point::of(MAX, 0, 0)), 0);
        assert_eq!(delta_col(&Point::min(), &Point::of(0, MAX, 0)), MAX);
        assert_eq!(delta_col(&Point::min(), &Point::of(0, 0, MAX)), 0);
    }

    #[test]
    fn test_delta_depth() {
        assert_eq!(delta_depth(&Point::min(), &Point::of(MAX, 0, 0)), 0);
        assert_eq!(delta_depth(&Point::min(), &Point::of(0, MAX, 0)), 0);
        assert_eq!(delta_depth(&Point::min(), &Point::of(0, 0, MAX)), MAX);
    }

    #[test]
    fn test_delta_max() {
        assert_eq!(delta_max(&Point::of(0, 1, 2), &Point::of(10, 10, 10)), 10);
        assert_eq!(delta_max(&Point::of(1, 2, 0), &Point::of(9, 9, 9)), 9);
        assert_eq!(delta_max(&Point::of(2, 0, 1), &Point::of(8, 8, 8)), 8);
    }

    #[test]
    fn test_delta_min() {
        assert_eq!(delta_min(&Point::of(0, 1, 2), &Point::of(10, 10, 10)), 8);
        assert_eq!(delta_min(&Point::of(1, 2, 0), &Point::of(9, 9, 9)), 7);
        assert_eq!(delta_min(&Point::of(2, 0, 1), &Point::of(8, 8, 8)), 6);
    }

    #[test]
    fn test_delta() {
        assert_eq!(delta(&Point::min(), &Point::min()), Point::min());
        assert_eq!(delta(&Point::min(), &Point::max()), Point::max());
    }
}
