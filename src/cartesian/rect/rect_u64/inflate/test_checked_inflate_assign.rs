use super::checked_inflate_assign;
use crate::cartesian::rect::rect_u64::RectU64;

#[test]
fn min_bounds() {
    let mut r = RectU64::of(7, 3, 9, 13);
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(6, 2, 10, 14));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(5, 1, 11, 15));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(4, 0, 12, 16));
}

#[test]
fn max_bounds() {
    let mut r = RectU64::of(u64::MAX - 33, u64::MAX - 17, u64::MAX - 5, u64::MAX - 3);
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(u64::MAX - 34, u64::MAX - 18, u64::MAX - 4, u64::MAX - 2));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(u64::MAX - 35, u64::MAX - 19, u64::MAX - 3, u64::MAX - 1));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(u64::MAX - 36, u64::MAX - 20, u64::MAX - 2, u64::MAX));
}
