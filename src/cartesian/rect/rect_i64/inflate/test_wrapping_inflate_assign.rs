use super::wrapping_inflate_assign;
use crate::cartesian::rect::rect_i64::RectI64;

#[test]
fn min_bounds() {
    let mut r = RectI64::of(i64::MIN + 7, i64::MIN + 3, i64::MIN + 9, i64::MIN + 13);
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MIN + 6, i64::MIN + 2, i64::MIN + 10, i64::MIN + 14));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MIN + 5, i64::MIN + 1, i64::MIN + 11, i64::MIN + 15));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 12, i64::MIN + 16));
}

#[test]
fn max_bounds() {
    let mut r = RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3);
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX));
}

#[test]
fn to_bounds() {
    let mut r = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1);
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI64::largest());

    let mut r_min_x = RectI64::of(i64::MIN + 1, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10);
    wrapping_inflate_assign(&mut r_min_x);
    assert_eq!(r_min_x, RectI64::of(i64::MIN, i64::MIN + 9, i64::MAX - 9, i64::MAX - 9));

    let mut r_min_y = RectI64::of(i64::MIN + 10, i64::MIN + 1, i64::MAX - 10, i64::MAX - 10);
    wrapping_inflate_assign(&mut r_min_y);
    assert_eq!(r_min_y, RectI64::of(i64::MIN + 9, i64::MIN, i64::MAX - 9, i64::MAX - 9));

    let mut r_max_x = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 1, i64::MAX - 10);
    wrapping_inflate_assign(&mut r_max_x);
    assert_eq!(r_max_x, RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MAX, i64::MAX - 9));

    let mut r_max_y = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX - 1);
    wrapping_inflate_assign(&mut r_max_y);
    assert_eq!(r_max_y, RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MAX - 9, i64::MAX));
}

#[test]
fn out_of_bounds() {
    let mut r = RectI64::largest();
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MAX, i64::MAX, i64::MIN, i64::MIN));

    let mut r_min_x = RectI64::of(i64::MIN, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10);
    wrapping_inflate_assign(&mut r_min_x);
    assert_eq!(r_min_x, RectI64::of(i64::MAX, i64::MIN + 9, i64::MAX - 9, i64::MAX - 9));

    let mut r_min_y = RectI64::of(i64::MIN + 10, i64::MIN, i64::MAX - 10, i64::MAX - 10);
    wrapping_inflate_assign(&mut r_min_y);
    assert_eq!(r_min_y, RectI64::of(i64::MIN + 9, i64::MAX, i64::MAX - 9, i64::MAX - 9));

    let mut r_max_x = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX, i64::MAX - 10);
    wrapping_inflate_assign(&mut r_max_x);
    assert_eq!(r_max_x, RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MIN, i64::MAX - 9));

    let mut r_max_y = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX);
    wrapping_inflate_assign(&mut r_max_y);
    assert_eq!(r_max_y, RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MAX - 9, i64::MIN));
}
