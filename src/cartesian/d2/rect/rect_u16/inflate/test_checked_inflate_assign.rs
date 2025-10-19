use super::checked_inflate_assign;
use crate::cartesian::{d1::point::point_u16::MAX, d2::rect::rect_u16::Rect};

#[test]
fn min_bounds() {
    let mut r = Rect::of(7, 3, 9, 13);
    checked_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(6, 2, 10, 14));
    checked_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(5, 1, 11, 15));
    checked_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(4, 0, 12, 16));
}

#[test]
fn max_bounds() {
    let mut r = Rect::of(MAX - 33, MAX - 17, MAX - 5, MAX - 3);
    checked_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2));
    checked_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1));
    checked_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MAX - 36, MAX - 20, MAX - 2, MAX));
}
