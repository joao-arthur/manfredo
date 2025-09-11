use super::checked_inflate;
use crate::matrix::rect::rect_u8::Rect;

const MAX: u8 = u8::MAX;

#[test]
fn min_bounds() {
    assert_eq!(checked_inflate(&Rect::of(7, 3, 9, 13)), Rect::of(6, 2, 10, 14));
    assert_eq!(checked_inflate(&Rect::of(6, 2, 10, 14)), Rect::of(5, 1, 11, 15));
    assert_eq!(checked_inflate(&Rect::of(5, 1, 11, 15)), Rect::of(4, 0, 12, 16));
}

#[test]
fn max_bounds() {
    assert_eq!(checked_inflate(&Rect::of(MAX - 33, MAX - 17, MAX - 5, MAX - 3)), Rect::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2));
    assert_eq!(checked_inflate(&Rect::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2)), Rect::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1));
    assert_eq!(checked_inflate(&Rect::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1)), Rect::of(MAX - 36, MAX - 20, MAX - 2, MAX));
}
