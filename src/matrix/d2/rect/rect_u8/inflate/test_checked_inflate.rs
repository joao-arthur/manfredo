use super::checked_inflate;
use crate::matrix::{d1::point::point_u8::MAX, d2::rect::rect_u8::Rect};

#[test]
fn min_bounds() {
    assert_eq!(checked_inflate(&Rect::new((7, 3), (9, 13))), Rect::new((6, 2), (10, 14)));
    assert_eq!(checked_inflate(&Rect::new((6, 2), (10, 14))), Rect::new((5, 1), (11, 15)));
    assert_eq!(checked_inflate(&Rect::new((5, 1), (11, 15))), Rect::new((4, 0), (12, 16)));
}

#[test]
fn max_bounds() {
    assert_eq!(checked_inflate(&Rect::new((MAX - 33, MAX - 17), (MAX - 5, MAX - 3))), Rect::new((MAX - 34, MAX - 18), (MAX - 4, MAX - 2)));
    assert_eq!(checked_inflate(&Rect::new((MAX - 34, MAX - 18), (MAX - 4, MAX - 2))), Rect::new((MAX - 35, MAX - 19), (MAX - 3, MAX - 1)));
    assert_eq!(checked_inflate(&Rect::new((MAX - 35, MAX - 19), (MAX - 3, MAX - 1))), Rect::new((MAX - 36, MAX - 20), (MAX - 2, MAX)));
}
