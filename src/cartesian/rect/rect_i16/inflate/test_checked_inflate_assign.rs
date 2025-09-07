use super::checked_inflate_assign;
use crate::cartesian::rect::rect_i16::RectI16;

const MIN: i16 = i16::MIN;
const MAX: i16 = i16::MAX;

#[test]
fn min_bounds() {
    let mut r = RectI16::of(MIN + 7, MIN + 3, MIN + 9, MIN + 13);
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectI16::of(MIN + 6, MIN + 2, MIN + 10, MIN + 14));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectI16::of(MIN + 5, MIN + 1, MIN + 11, MIN + 15));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectI16::of(MIN + 4, MIN, MIN + 12, MIN + 16));
}

#[test]
fn max_bounds() {
    let mut r = RectI16::of(MAX - 33, MAX - 17, MAX - 5, MAX - 3);
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectI16::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectI16::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1));
    checked_inflate_assign(&mut r);
    assert_eq!(r, RectI16::of(MAX - 36, MAX - 20, MAX - 2, MAX));
}
