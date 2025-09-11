use super::saturating_inflate_assign;
use crate::matrix::rect::rect_u64::Rect;

const MAX: u64 = u64::MAX;

#[test]
fn min_bounds() {
    let mut r = Rect::of(7, 2, 17, 13);
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(6, 1, 18, 14));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(5, 0, 19, 15));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(4, 0, 20, 17));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(3, 0, 21, 19));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(2, 0, 22, 21));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(1, 0, 23, 23));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(0, 0, 24, 25));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(0, 0, 26, 27));
}

#[test]
fn max_bounds() {
    let mut r = Rect::of(MAX - 33, MAX - 17, MAX - 5, MAX - 3);
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MAX - 36, MAX - 20, MAX - 2, MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MAX - 37, MAX - 22, MAX - 1, MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MAX - 38, MAX - 24, MAX, MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MAX - 40, MAX - 26, MAX, MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MAX - 42, MAX - 28, MAX, MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MAX - 44, MAX - 30, MAX, MAX));
}
