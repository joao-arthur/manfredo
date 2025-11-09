use super::Rect;
use crate::cartesian::d2::point::point_f32::Point;

pub fn contains_point(r: &Rect, p: &Point) -> bool {
    p.x >= r.min.x && p.x <= r.max.x && p.y >= r.min.y && p.y <= r.max.y
}

#[cfg(test)]
mod tests {
    use super::contains_point;
    use crate::cartesian::{
        d1::point::point_f32::{MAX, MIN},
        d2::{point::point_f32::Point, rect::rect_f32::Rect},
    };

    #[test]
    fn inside_borders() {
        let r_negative = Rect::new((MIN + 1.0, MIN + 1.0), (-1.0, -1.0));
        assert!(contains_point(&r_negative, &Point::new(MIN + 1.0, MIN + 1.0)));
        assert!(contains_point(&r_negative, &Point::new(MIN + 1.0, -1.0)));
        assert!(contains_point(&r_negative, &Point::new(-1.0, MIN + 1.0)));
        assert!(contains_point(&r_negative, &Point::new(-1.0, -1.0)));

        let r_positive = Rect::new((1.0, 1.0), (MAX - 1.0, MAX - 1.0));
        assert!(contains_point(&r_positive, &Point::new(1.0, 1.0)));
        assert!(contains_point(&r_positive, &Point::new(1.0, MAX - 1.0)));
        assert!(contains_point(&r_positive, &Point::new(MAX - 1.0, 1.0)));
        assert!(contains_point(&r_positive, &Point::new(MAX - 1.0, MAX - 1.0)));
    }

    #[test]
    fn outside_borders() {
        let r_negative = Rect::new((MIN + 1.0, MIN + 1.0), (-1.0, -1.0));
        assert!(!contains_point(&r_negative, &Point::min()));
        assert!(!contains_point(&r_negative, &Point::new(MIN + 1.0, MIN)));
        assert!(!contains_point(&r_negative, &Point::new(MIN, MIN + 1.0)));

        assert!(!contains_point(&r_negative, &Point::new(MIN, 0.0)));
        assert!(!contains_point(&r_negative, &Point::new(MIN, -1.0)));
        assert!(!contains_point(&r_negative, &Point::new(MIN + 1.0, 0.0)));

        assert!(!contains_point(&r_negative, &Point::new(MAX, MIN)));
        assert!(!contains_point(&r_negative, &Point::new(MAX, MIN + 1.0)));
        assert!(!contains_point(&r_negative, &Point::new(-1.0, MIN)));

        assert!(!contains_point(&r_negative, &Point::max()));
        assert!(!contains_point(&r_negative, &Point::new(-1.0, 0.0)));
        assert!(!contains_point(&r_negative, &Point::new(0.0, -1.0)));

        let r_positive = Rect::new((1.0, 1.0), (MAX - 1.0, MAX - 1.0));
        assert!(!contains_point(&r_positive, &Point::min()));
        assert!(!contains_point(&r_positive, &Point::new(1.0, 0.0)));
        assert!(!contains_point(&r_positive, &Point::new(0.0, 1.0)));

        assert!(!contains_point(&r_positive, &Point::new(0.0, MAX)));
        assert!(!contains_point(&r_positive, &Point::new(0.0, MAX - 1.0)));
        assert!(!contains_point(&r_positive, &Point::new(1.0, MAX)));

        assert!(!contains_point(&r_positive, &Point::new(MAX, 0.0)));
        assert!(!contains_point(&r_positive, &Point::new(MAX, 1.0)));
        assert!(!contains_point(&r_positive, &Point::new(MAX - 1.0, 0.0)));

        assert!(!contains_point(&r_positive, &Point::max()));
        assert!(!contains_point(&r_positive, &Point::new(MAX - 1.0, MAX)));
        assert!(!contains_point(&r_positive, &Point::new(MAX, MAX - 1.0)));
    }

    #[test]
    fn inside() {
        let r_negative = Rect::new((MIN + 1.0, MIN + 1.0), (-1.0, -1.0));
        assert!(contains_point(&r_negative, &Point::new(MIN + 10.0, MIN + 10.0)));
        assert!(contains_point(&r_negative, &Point::new(-10.0, MIN + 10.0)));
        assert!(contains_point(&r_negative, &Point::new(MIN + 10.0, -10.0)));
        assert!(contains_point(&r_negative, &Point::new(-10.0, -10.0)));

        let r_positive = Rect::new((1.0, 1.0), (MAX - 1.0, MAX - 1.0));
        assert!(contains_point(&r_positive, &Point::new(10.0, 10.0)));
        assert!(contains_point(&r_positive, &Point::new(MAX - 10.0, 10.0)));
        assert!(contains_point(&r_positive, &Point::new(10.0, MAX - 10.0)));
        assert!(contains_point(&r_positive, &Point::new(MAX - 10.0, MAX - 10.0)));
    }
}
