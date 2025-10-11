use super::Rect;
use crate::matrix::d2::point::point_u64::Point;

pub fn contains_point(r: &Rect, p: &Point) -> bool {
    p.row >= r.min.row && p.row <= r.max.row && p.col >= r.min.col && p.col <= r.max.col
}

#[cfg(test)]
mod tests {
    use super::contains_point;
    use crate::matrix::d2::{point::point_u64::Point, rect::rect_u64::Rect};

    const MAX: u64 = u64::MAX;

    #[test]
    fn inside_borders() {
        let r = Rect::of(1, 1, MAX - 1, MAX - 1);
        assert!(contains_point(&r, &Point::of(1, 1)));
        assert!(contains_point(&r, &Point::of(1, MAX - 1)));
        assert!(contains_point(&r, &Point::of(MAX - 1, 1)));
        assert!(contains_point(&r, &Point::of(MAX - 1, MAX - 1)));
    }

    #[test]
    fn outside_borders() {
        let r = Rect::of(1, 1, MAX - 1, MAX - 1);
        assert!(!contains_point(&r, &Point::min()));
        assert!(!contains_point(&r, &Point::of(1, 0)));
        assert!(!contains_point(&r, &Point::of(0, 1)));

        assert!(!contains_point(&r, &Point::of(0, MAX)));
        assert!(!contains_point(&r, &Point::of(0, MAX - 1)));
        assert!(!contains_point(&r, &Point::of(1, MAX)));

        assert!(!contains_point(&r, &Point::of(MAX, 0)));
        assert!(!contains_point(&r, &Point::of(MAX, 1)));
        assert!(!contains_point(&r, &Point::of(MAX - 1, 0)));

        assert!(!contains_point(&r, &Point::max()));
        assert!(!contains_point(&r, &Point::of(MAX - 1, MAX)));
        assert!(!contains_point(&r, &Point::of(MAX, MAX - 1)));
    }

    #[test]
    fn inside() {
        let r = Rect::of(1, 1, MAX - 1, MAX - 1);
        assert!(contains_point(&r, &Point::of(10, 10)));
        assert!(contains_point(&r, &Point::of(MAX - 10, 10)));
        assert!(contains_point(&r, &Point::of(10, MAX - 10)));
        assert!(contains_point(&r, &Point::of(MAX - 10, MAX - 10)));
    }
}
