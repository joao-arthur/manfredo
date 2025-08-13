use crate::cartesian::point::point_u8;

use super::{RectU8, delta_x, delta_y};

pub mod saturated;

pub fn deflate(r: &mut RectU8) {
    if delta_x(r) < 3 || delta_y(r) < 3 {
        return;
    }
    r.min.x += 1;
    r.min.y += 1;
    r.max.x -= 1;
    r.max.y -= 1;
}

pub fn contains(r: &RectU8, p: &point_u8::PointU8) -> bool {
    p.x >= r.min.x && p.x <= r.max.x && p.y >= r.min.y && p.y <= r.max.y
}

#[cfg(test)]
mod tests {
    use crate::cartesian::point::point_u8::PointU8;

    use super::{RectU8, contains, deflate};

    #[test]
    fn rect_u8() {
        assert_eq!(RectU8::of(0, 2, 4, 8), RectU8 { min: PointU8 { x: 0, y: 2 }, max: PointU8 { x: 4, y: 8 } });
        assert_eq!(RectU8::of(u8::MAX, 0, 0, u8::MAX).to_string(), "((255, 0), (0, 255))");
    }

    #[test]
    fn iter_x() {
        assert_eq!(RectU8::of(3, 6, 2, 8).iter_x().collect::<Vec<u8>>(), []);
        assert_eq!(RectU8::of(3, 6, 3, 8).iter_x().collect::<Vec<u8>>(), [3]);
        assert_eq!(RectU8::of(3, 6, 4, 8).iter_x().collect::<Vec<u8>>(), [3, 4]);
        assert_eq!(RectU8::of(3, 6, 5, 8).iter_x().collect::<Vec<u8>>(), [3, 4, 5]);
        assert_eq!(RectU8::of(3, 6, 6, 8).iter_x().collect::<Vec<u8>>(), [3, 4, 5, 6]);
        assert_eq!(RectU8::of(3, 6, 6, 8).iter_x().rev().collect::<Vec<u8>>(), [6, 5, 4, 3]);
        assert_eq!(RectU8::of(3, 6, 5, 8).iter_x().rev().collect::<Vec<u8>>(), [5, 4, 3]);
        assert_eq!(RectU8::of(3, 6, 4, 8).iter_x().rev().collect::<Vec<u8>>(), [4, 3]);
        assert_eq!(RectU8::of(3, 6, 3, 8).iter_x().rev().collect::<Vec<u8>>(), [3]);
        assert_eq!(RectU8::of(3, 6, 2, 8).iter_x().rev().collect::<Vec<u8>>(), []);
    }

    #[test]
    fn iter_y() {
        assert_eq!(RectU8::of(3, 6, 4, 5).iter_y().collect::<Vec<u8>>(), []);
        assert_eq!(RectU8::of(3, 6, 4, 6).iter_y().collect::<Vec<u8>>(), [6]);
        assert_eq!(RectU8::of(3, 6, 4, 7).iter_y().collect::<Vec<u8>>(), [6, 7]);
        assert_eq!(RectU8::of(3, 6, 4, 8).iter_y().collect::<Vec<u8>>(), [6, 7, 8]);
        assert_eq!(RectU8::of(3, 6, 4, 9).iter_y().collect::<Vec<u8>>(), [6, 7, 8, 9]);
        assert_eq!(RectU8::of(3, 6, 4, 9).iter_y().rev().collect::<Vec<u8>>(), [9, 8, 7, 6]);
        assert_eq!(RectU8::of(3, 6, 4, 8).iter_y().rev().collect::<Vec<u8>>(), [8, 7, 6]);
        assert_eq!(RectU8::of(3, 6, 4, 7).iter_y().rev().collect::<Vec<u8>>(), [7, 6]);
        assert_eq!(RectU8::of(3, 6, 4, 6).iter_y().rev().collect::<Vec<u8>>(), [6]);
        assert_eq!(RectU8::of(3, 6, 4, 5).iter_y().rev().collect::<Vec<u8>>(), []);
    }

    #[test]
    fn deflate_odd() {
        let mut r = RectU8::of(0, 0, 9, 9);
        deflate(&mut r);
        assert_eq!(r, RectU8::of(1, 1, 8, 8));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(2, 2, 7, 7));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(3, 3, 6, 6));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(4, 4, 5, 5));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(4, 4, 5, 5));
    }

    #[test]
    fn deflate_even() {
        let mut r = RectU8::of(0, 0, 10, 10);
        deflate(&mut r);
        assert_eq!(r, RectU8::of(1, 1, 9, 9));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(2, 2, 8, 8));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(3, 3, 7, 7));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(4, 4, 6, 6));
        deflate(&mut r);
        assert_eq!(r, RectU8::of(4, 4, 6, 6));
    }

    #[test]
    fn contains_inside_borders() {
        let r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert!(contains(&r, &PointU8::of(1, 1)));
        assert!(contains(&r, &PointU8::of(1, u8::MAX - 1)));
        assert!(contains(&r, &PointU8::of(u8::MAX - 1, 1)));
        assert!(contains(&r, &PointU8::of(u8::MAX - 1, u8::MAX - 1)));
    }

    #[test]
    fn contains_outside_borders() {
        let r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert!(!contains(&r, &PointU8::of(0, 0)));
        assert!(!contains(&r, &PointU8::of(0, u8::MAX)));
        assert!(!contains(&r, &PointU8::of(u8::MAX, 0)));
        assert!(!contains(&r, &PointU8::max()));
        assert!(!contains(&r, &PointU8::of(1, 0)));
        assert!(!contains(&r, &PointU8::of(1, u8::MAX)));
        assert!(!contains(&r, &PointU8::of(u8::MAX - 1, 0)));
        assert!(!contains(&r, &PointU8::of(u8::MAX - 1, u8::MAX)));
        assert!(!contains(&r, &PointU8::of(0, 1)));
        assert!(!contains(&r, &PointU8::of(0, u8::MAX - 1)));
        assert!(!contains(&r, &PointU8::of(u8::MAX, 1)));
        assert!(!contains(&r, &PointU8::of(u8::MAX, u8::MAX - 1)));
    }

    #[test]
    fn contains_inside() {
        let r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
        assert!(contains(&r, &PointU8::of(10, 10)));
        assert!(contains(&r, &PointU8::of(u8::MAX - 10, u8::MAX - 10)));
    }
}
