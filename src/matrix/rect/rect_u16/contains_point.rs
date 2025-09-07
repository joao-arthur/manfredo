use crate::matrix::{point::point_u16::PointU16, rect::rect_u16::RectU16};

pub fn contains_point(r: &RectU16, p: &PointU16) -> bool {
    p.row >= r.min.row && p.row <= r.max.row && p.col >= r.min.col && p.col <= r.max.col
}

#[cfg(test)]
mod tests {
    use super::contains_point;
    use crate::matrix::{point::point_u16::PointU16, rect::rect_u16::RectU16};

    const MAX: u16 = u16::MAX;

    #[test]
    fn inside_borders() {
        let r = RectU16::of(1, 1, MAX - 1, MAX - 1);
        assert!(contains_point(&r, &PointU16::of(1, 1)));
        assert!(contains_point(&r, &PointU16::of(1, MAX - 1)));
        assert!(contains_point(&r, &PointU16::of(u16::MAX - 1, 1)));
        assert!(contains_point(&r, &PointU16::of(u16::MAX - 1, MAX - 1)));
    }

    #[test]
    fn outside_borders() {
        let r = RectU16::of(1, 1, MAX - 1, MAX - 1);
        assert!(!contains_point(&r, &PointU16::min()));
        assert!(!contains_point(&r, &PointU16::of(1, 0)));
        assert!(!contains_point(&r, &PointU16::of(0, 1)));

        assert!(!contains_point(&r, &PointU16::of(0, MAX)));
        assert!(!contains_point(&r, &PointU16::of(0, MAX - 1)));
        assert!(!contains_point(&r, &PointU16::of(1, MAX)));

        assert!(!contains_point(&r, &PointU16::of(u16::MAX, 0)));
        assert!(!contains_point(&r, &PointU16::of(u16::MAX, 1)));
        assert!(!contains_point(&r, &PointU16::of(u16::MAX - 1, 0)));

        assert!(!contains_point(&r, &PointU16::max()));
        assert!(!contains_point(&r, &PointU16::of(u16::MAX - 1, MAX)));
        assert!(!contains_point(&r, &PointU16::of(u16::MAX, MAX - 1)));
    }

    #[test]
    fn inside() {
        let r = RectU16::of(1, 1, MAX - 1, MAX - 1);
        assert!(contains_point(&r, &PointU16::of(10, 10)));
        assert!(contains_point(&r, &PointU16::of(u16::MAX - 10, 10)));
        assert!(contains_point(&r, &PointU16::of(10, MAX - 10)));
        assert!(contains_point(&r, &PointU16::of(u16::MAX - 10, MAX - 10)));
    }
}
