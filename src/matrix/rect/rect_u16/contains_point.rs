use crate::matrix::{point::point_u16::PointU16, rect::rect_u16::RectU16};

pub fn contains_point(r: &RectU16, p: &PointU16) -> bool {
    p.row >= r.min.row && p.row <= r.max.row && p.col >= r.min.col && p.col <= r.max.col
}

#[cfg(test)]
mod tests {
    use super::contains_point;
    use crate::matrix::{point::point_u16::PointU16, rect::rect_u16::RectU16};

    #[test]
    fn contains_point_inside_borders() {
        let r = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assert!(contains_point(&r, &PointU16::of(1, 1)));
        assert!(contains_point(&r, &PointU16::of(1, u16::MAX - 1)));
        assert!(contains_point(&r, &PointU16::of(u16::MAX - 1, 1)));
        assert!(contains_point(&r, &PointU16::of(u16::MAX - 1, u16::MAX - 1)));
    }

    #[test]
    fn contains_point_outside_borders() {
        let r = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assert!(!contains_point(&r, &PointU16::min()));
        assert!(!contains_point(&r, &PointU16::of(1, 0)));
        assert!(!contains_point(&r, &PointU16::of(0, 1)));

        assert!(!contains_point(&r, &PointU16::of(0, u16::MAX)));
        assert!(!contains_point(&r, &PointU16::of(0, u16::MAX - 1)));
        assert!(!contains_point(&r, &PointU16::of(1, u16::MAX)));

        assert!(!contains_point(&r, &PointU16::of(u16::MAX, 0)));
        assert!(!contains_point(&r, &PointU16::of(u16::MAX, 1)));
        assert!(!contains_point(&r, &PointU16::of(u16::MAX - 1, 0)));

        assert!(!contains_point(&r, &PointU16::max()));
        assert!(!contains_point(&r, &PointU16::of(u16::MAX - 1, u16::MAX)));
        assert!(!contains_point(&r, &PointU16::of(u16::MAX, u16::MAX - 1)));
    }

    #[test]
    fn contains_point_inside() {
        let r = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
        assert!(contains_point(&r, &PointU16::of(10, 10)));
        assert!(contains_point(&r, &PointU16::of(u16::MAX - 10, 10)));
        assert!(contains_point(&r, &PointU16::of(10, u16::MAX - 10)));
        assert!(contains_point(&r, &PointU16::of(u16::MAX - 10, u16::MAX - 10)));
    }
}
