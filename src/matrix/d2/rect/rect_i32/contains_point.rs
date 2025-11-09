use super::Rect;
use crate::matrix::d2::point::point_i32::Point;

pub fn contains_point(r: &Rect, p: &Point) -> bool {
    p.row >= r.min.row && p.row <= r.max.row && p.col >= r.min.col && p.col <= r.max.col
}

#[cfg(test)]
mod tests {
    use super::contains_point;
    use crate::matrix::{
        d1::point::point_i32::{MAX, MIN},
        d2::{point::point_i32::Point, rect::rect_i32::Rect},
    };

    #[test]
    fn inside_borders() {
        let r = Rect::new((MIN + 1, MIN + 1), (MAX - 1, MAX - 1));
        assert!(contains_point(&r, &Point::new(MIN + 1, MIN + 1)));
        assert!(contains_point(&r, &Point::new(MIN + 1, MAX - 1)));
        assert!(contains_point(&r, &Point::new(MAX - 1, MIN + 1)));
        assert!(contains_point(&r, &Point::new(MAX - 1, MAX - 1)));
    }

    #[test]
    fn outside_borders() {
        let r = Rect::new((MIN + 1, MIN + 1), (MAX - 1, MAX - 1));
        assert!(!contains_point(&r, &Point::min()));
        assert!(!contains_point(&r, &Point::new(MIN + 1, MIN)));
        assert!(!contains_point(&r, &Point::new(MIN, MIN + 1)));

        assert!(!contains_point(&r, &Point::new(MIN, MAX)));
        assert!(!contains_point(&r, &Point::new(MIN, MAX - 1)));
        assert!(!contains_point(&r, &Point::new(MIN + 1, MAX)));

        assert!(!contains_point(&r, &Point::new(MAX, MIN)));
        assert!(!contains_point(&r, &Point::new(MAX, MIN + 1)));
        assert!(!contains_point(&r, &Point::new(MAX - 1, MIN)));

        assert!(!contains_point(&r, &Point::max()));
        assert!(!contains_point(&r, &Point::new(MAX - 1, MAX)));
        assert!(!contains_point(&r, &Point::new(MAX, MAX - 1)));
    }

    #[test]
    fn inside() {
        let r = Rect::new((MIN + 1, MIN + 1), (MAX - 1, MAX - 1));
        assert!(contains_point(&r, &Point::new(MIN + 10, MIN + 10)));
        assert!(contains_point(&r, &Point::new(MAX - 10, MIN + 10)));
        assert!(contains_point(&r, &Point::new(MIN + 10, MAX - 10)));
        assert!(contains_point(&r, &Point::new(MAX - 10, MAX - 10)));
    }
}
