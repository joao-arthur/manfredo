use super::wrapping_inflate_assign;
use crate::cartesian::rect::rect_i8::RectI8;

#[test]
fn min_bounds() {
    let mut r = RectI8::of(i8::MIN + 7, i8::MIN + 3, i8::MIN + 9, i8::MIN + 13);
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MIN + 6, i8::MIN + 2, i8::MIN + 10, i8::MIN + 14));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MIN + 5, i8::MIN + 1, i8::MIN + 11, i8::MIN + 15));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 12, i8::MIN + 16));
}

#[test]
fn max_bounds() {
    let mut r = RectI8::of(i8::MAX - 33, i8::MAX - 17, i8::MAX - 5, i8::MAX - 3);
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX));
}

#[test]
fn to_bounds() {
    let mut r = RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1);
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI8::largest());

    let mut r_min_x = RectI8::of(i8::MIN + 1, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10);
    wrapping_inflate_assign(&mut r_min_x);
    assert_eq!(r_min_x, RectI8::of(i8::MIN, i8::MIN + 9, i8::MAX - 9, i8::MAX - 9));

    let mut r_min_y = RectI8::of(i8::MIN + 10, i8::MIN + 1, i8::MAX - 10, i8::MAX - 10);
    wrapping_inflate_assign(&mut r_min_y);
    assert_eq!(r_min_y, RectI8::of(i8::MIN + 9, i8::MIN, i8::MAX - 9, i8::MAX - 9));

    let mut r_max_x = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 1, i8::MAX - 10);
    wrapping_inflate_assign(&mut r_max_x);
    assert_eq!(r_max_x, RectI8::of(i8::MIN + 9, i8::MIN + 9, i8::MAX, i8::MAX - 9));

    let mut r_max_y = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX - 1);
    wrapping_inflate_assign(&mut r_max_y);
    assert_eq!(r_max_y, RectI8::of(i8::MIN + 9, i8::MIN + 9, i8::MAX - 9, i8::MAX));
}

#[test]
fn out_of_bounds() {
    let mut r = RectI8::largest();
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI8::of(i8::MAX, i8::MAX, i8::MIN, i8::MIN));

    let mut r_min_x = RectI8::of(i8::MIN, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10);
    wrapping_inflate_assign(&mut r_min_x);
    assert_eq!(r_min_x, RectI8::of(i8::MAX, i8::MIN + 9, i8::MAX - 9, i8::MAX - 9));

    let mut r_min_y = RectI8::of(i8::MIN + 10, i8::MIN, i8::MAX - 10, i8::MAX - 10);
    wrapping_inflate_assign(&mut r_min_y);
    assert_eq!(r_min_y, RectI8::of(i8::MIN + 9, i8::MAX, i8::MAX - 9, i8::MAX - 9));

    let mut r_max_x = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX, i8::MAX - 10);
    wrapping_inflate_assign(&mut r_max_x);
    assert_eq!(r_max_x, RectI8::of(i8::MIN + 9, i8::MIN + 9, i8::MIN, i8::MAX - 9));

    let mut r_max_y = RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX);
    wrapping_inflate_assign(&mut r_max_y);
    assert_eq!(r_max_y, RectI8::of(i8::MIN + 9, i8::MIN + 9, i8::MAX - 9, i8::MIN));
}
