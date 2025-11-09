use super::Rect;
use crate::cartesian::d2::point::point_u16::Point;

pub fn contains_point(r: &Rect, p: &Point) -> bool {
    p.x >= r.min.x && p.x <= r.max.x && p.y >= r.min.y && p.y <= r.max.y
}

#[cfg(test)]
mod tests {
    use super::contains_point;
    use crate::cartesian::{
        d1::point::point_u16::MAX,
        d2::{point::point_u16::Point, rect::rect_u16::Rect},
    };

    #[test]
    fn inside_borders() {
        let r = Rect::new((1, 1), (MAX - 1, MAX - 1));
        assert!(contains_point(&r, &Point::new(1, 1)));
        assert!(contains_point(&r, &Point::new(1, MAX - 1)));
        assert!(contains_point(&r, &Point::new(MAX - 1, 1)));
        assert!(contains_point(&r, &Point::new(MAX - 1, MAX - 1)));
    }

    #[test]
    fn outside_borders() {
        let r = Rect::new((1, 1), (MAX - 1, MAX - 1));
        assert!(!contains_point(&r, &Point::min()));
        assert!(!contains_point(&r, &Point::new(1, 0)));
        assert!(!contains_point(&r, &Point::new(0, 1)));

        assert!(!contains_point(&r, &Point::new(0, MAX)));
        assert!(!contains_point(&r, &Point::new(0, MAX - 1)));
        assert!(!contains_point(&r, &Point::new(1, MAX)));

        assert!(!contains_point(&r, &Point::new(MAX, 0)));
        assert!(!contains_point(&r, &Point::new(MAX, 1)));
        assert!(!contains_point(&r, &Point::new(MAX - 1, 0)));

        assert!(!contains_point(&r, &Point::max()));
        assert!(!contains_point(&r, &Point::new(MAX - 1, MAX)));
        assert!(!contains_point(&r, &Point::new(MAX, MAX - 1)));
    }

    #[test]
    fn inside() {
        let r = Rect::new((1, 1), (MAX - 1, MAX - 1));
        assert!(contains_point(&r, &Point::new(10, 10)));
        assert!(contains_point(&r, &Point::new(MAX - 10, 10)));
        assert!(contains_point(&r, &Point::new(10, MAX - 10)));
        assert!(contains_point(&r, &Point::new(MAX - 10, MAX - 10)));
    }
}
