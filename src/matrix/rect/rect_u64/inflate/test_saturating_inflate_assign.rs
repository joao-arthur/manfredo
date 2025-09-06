use super::saturating_inflate_assign;
use crate::matrix::rect::rect_u64::RectU64;

#[test]
fn min_bounds() {
    let mut r = RectU64::of(7, 2, 17, 13);
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(6, 1, 18, 14));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(5, 0, 19, 15));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(4, 0, 20, 17));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(3, 0, 21, 19));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(2, 0, 22, 21));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(1, 0, 23, 23));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(0, 0, 24, 25));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(0, 0, 26, 27));
}

#[test]
fn max_bounds() {
    let mut r = RectU64::of(u64::MAX - 33, u64::MAX - 17, u64::MAX - 5, u64::MAX - 3);
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(u64::MAX - 34, u64::MAX - 18, u64::MAX - 4, u64::MAX - 2));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(u64::MAX - 35, u64::MAX - 19, u64::MAX - 3, u64::MAX - 1));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(u64::MAX - 36, u64::MAX - 20, u64::MAX - 2, u64::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(u64::MAX - 37, u64::MAX - 22, u64::MAX - 1, u64::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(u64::MAX - 38, u64::MAX - 24, u64::MAX, u64::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(u64::MAX - 40, u64::MAX - 26, u64::MAX, u64::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(u64::MAX - 42, u64::MAX - 28, u64::MAX, u64::MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(u64::MAX - 44, u64::MAX - 30, u64::MAX, u64::MAX));
}
