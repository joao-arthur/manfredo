use crate::cartesian::d2::{point::point_i32::Point, rect::rect_i32::Rect};

pub fn contains_point(r: &Rect, p: &Point) -> bool {
    p.x >= r.min.x && p.x <= r.max.x && p.y >= r.min.y && p.y <= r.max.y
}

#[cfg(test)]
mod tests {
    use super::contains_point;
    use crate::cartesian::d2::{point::point_i32::Point, rect::rect_i32::Rect};

    const MIN: i32 = i32::MIN;
    const MAX: i32 = i32::MAX;

    #[test]
    fn inside_borders() {
        let r = Rect::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(contains_point(&r, &Point::of(MIN + 1, MIN + 1)));
        assert!(contains_point(&r, &Point::of(MIN + 1, MAX - 1)));
        assert!(contains_point(&r, &Point::of(MAX - 1, MIN + 1)));
        assert!(contains_point(&r, &Point::of(MAX - 1, MAX - 1)));
    }

    #[test]
    fn outside_borders() {
        let r = Rect::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(!contains_point(&r, &Point::min()));
        assert!(!contains_point(&r, &Point::of(MIN + 1, MIN)));
        assert!(!contains_point(&r, &Point::of(MIN, MIN + 1)));

        assert!(!contains_point(&r, &Point::of(MIN, MAX)));
        assert!(!contains_point(&r, &Point::of(MIN, MAX - 1)));
        assert!(!contains_point(&r, &Point::of(MIN + 1, MAX)));

        assert!(!contains_point(&r, &Point::of(MAX, MIN)));
        assert!(!contains_point(&r, &Point::of(MAX, MIN + 1)));
        assert!(!contains_point(&r, &Point::of(MAX - 1, MIN)));

        assert!(!contains_point(&r, &Point::max()));
        assert!(!contains_point(&r, &Point::of(MAX - 1, MAX)));
        assert!(!contains_point(&r, &Point::of(MAX, MAX - 1)));
    }

    #[test]
    fn inside() {
        let r = Rect::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
        assert!(contains_point(&r, &Point::of(MIN + 10, MIN + 10)));
        assert!(contains_point(&r, &Point::of(MAX - 10, MIN + 10)));
        assert!(contains_point(&r, &Point::of(MIN + 10, MAX - 10)));
        assert!(contains_point(&r, &Point::of(MAX - 10, MAX - 10)));
    }
}
