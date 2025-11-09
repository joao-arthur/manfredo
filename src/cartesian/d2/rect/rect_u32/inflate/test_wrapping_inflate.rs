use super::wrapping_inflate;
use crate::cartesian::{d1::point::point_u32::MAX, d2::rect::rect_u32::Rect};

#[test]
fn min_bounds() {
    assert_eq!(wrapping_inflate(&Rect::new((7, 3), (9, 13))), Rect::new((6, 2), (10, 14)));
    assert_eq!(wrapping_inflate(&Rect::new((6, 2), (10, 14))), Rect::new((5, 1), (11, 15)));
    assert_eq!(wrapping_inflate(&Rect::new((5, 1), (11, 15))), Rect::new((4, 0), (12, 16)));
}

#[test]
fn max_bounds() {
    assert_eq!(wrapping_inflate(&Rect::new((MAX - 33, MAX - 17), (MAX - 5, MAX - 3))), Rect::new((MAX - 34, MAX - 18), (MAX - 4, MAX - 2)));
    assert_eq!(wrapping_inflate(&Rect::new((MAX - 34, MAX - 18), (MAX - 4, MAX - 2))), Rect::new((MAX - 35, MAX - 19), (MAX - 3, MAX - 1)));
    assert_eq!(wrapping_inflate(&Rect::new((MAX - 35, MAX - 19), (MAX - 3, MAX - 1))), Rect::new((MAX - 36, MAX - 20), (MAX - 2, MAX)));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_inflate(&Rect::new((1, 1), (MAX - 1, MAX - 1))), Rect::largest());
    assert_eq!(wrapping_inflate(&Rect::new((1, 10), (MAX - 10, MAX - 10))), Rect::new((0, 9), (MAX - 9, MAX - 9)));
    assert_eq!(wrapping_inflate(&Rect::new((10, 1), (MAX - 10, MAX - 10))), Rect::new((9, 0), (MAX - 9, MAX - 9)));
    assert_eq!(wrapping_inflate(&Rect::new((10, 10), (MAX - 1, MAX - 10))), Rect::new((9, 9), (MAX, MAX - 9)));
    assert_eq!(wrapping_inflate(&Rect::new((10, 10), (MAX - 10, MAX - 1))), Rect::new((9, 9), (MAX - 9, MAX)));
}

#[test]
fn out_of_bounds() {
    assert_eq!(wrapping_inflate(&Rect::largest()), Rect::new((MAX, MAX), (0, 0)));
    assert_eq!(wrapping_inflate(&Rect::new((0, 10), (MAX - 10, MAX - 10))), Rect::new((MAX, 9), (MAX - 9, MAX - 9)));
    assert_eq!(wrapping_inflate(&Rect::new((10, 0), (MAX - 10, MAX - 10))), Rect::new((9, MAX), (MAX - 9, MAX - 9)));
    assert_eq!(wrapping_inflate(&Rect::new((10, 10), (MAX, MAX - 10))), Rect::new((9, 9), (0, MAX - 9)));
    assert_eq!(wrapping_inflate(&Rect::new((10, 10), (MAX - 10, MAX))), Rect::new((9, 9), (MAX - 9, 0)));
}
