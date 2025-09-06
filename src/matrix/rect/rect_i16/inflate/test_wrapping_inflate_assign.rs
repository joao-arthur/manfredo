use super::wrapping_inflate_assign;
use crate::matrix::rect::rect_i16::RectI16;

#[test]
fn min_bounds() {
    let mut r = RectI16::of(i16::MIN + 7, i16::MIN + 3, i16::MIN + 9, i16::MIN + 13);
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI16::of(i16::MIN + 6, i16::MIN + 2, i16::MIN + 10, i16::MIN + 14));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI16::of(i16::MIN + 5, i16::MIN + 1, i16::MIN + 11, i16::MIN + 15));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 12, i16::MIN + 16));
}

#[test]
fn max_bounds() {
    let mut r = RectI16::of(i16::MAX - 33, i16::MAX - 17, i16::MAX - 5, i16::MAX - 3);
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX));
}

#[test]
fn to_bounds() {
    let mut r = RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1);
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI16::largest());

    let mut r_min_row = RectI16::of(i16::MIN + 1, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10);
    wrapping_inflate_assign(&mut r_min_row);
    assert_eq!(r_min_row, RectI16::of(i16::MIN, i16::MIN + 9, i16::MAX - 9, i16::MAX - 9));

    let mut r_min_y = RectI16::of(i16::MIN + 10, i16::MIN + 1, i16::MAX - 10, i16::MAX - 10);
    wrapping_inflate_assign(&mut r_min_y);
    assert_eq!(r_min_y, RectI16::of(i16::MIN + 9, i16::MIN, i16::MAX - 9, i16::MAX - 9));

    let mut r_max_x = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 1, i16::MAX - 10);
    wrapping_inflate_assign(&mut r_max_x);
    assert_eq!(r_max_x, RectI16::of(i16::MIN + 9, i16::MIN + 9, i16::MAX, i16::MAX - 9));

    let mut r_max_y = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 1);
    wrapping_inflate_assign(&mut r_max_y);
    assert_eq!(r_max_y, RectI16::of(i16::MIN + 9, i16::MIN + 9, i16::MAX - 9, i16::MAX));
}

#[test]
fn out_of_bounds() {
    let mut r = RectI16::largest();
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI16::of(i16::MAX, i16::MAX, i16::MIN, i16::MIN));

    let mut r_min_row = RectI16::of(i16::MIN, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10);
    wrapping_inflate_assign(&mut r_min_row);
    assert_eq!(r_min_row, RectI16::of(i16::MAX, i16::MIN + 9, i16::MAX - 9, i16::MAX - 9));

    let mut r_min_y = RectI16::of(i16::MIN + 10, i16::MIN, i16::MAX - 10, i16::MAX - 10);
    wrapping_inflate_assign(&mut r_min_y);
    assert_eq!(r_min_y, RectI16::of(i16::MIN + 9, i16::MAX, i16::MAX - 9, i16::MAX - 9));

    let mut r_max_x = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX, i16::MAX - 10);
    wrapping_inflate_assign(&mut r_max_x);
    assert_eq!(r_max_x, RectI16::of(i16::MIN + 9, i16::MIN + 9, i16::MIN, i16::MAX - 9));

    let mut r_max_y = RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX);
    wrapping_inflate_assign(&mut r_max_y);
    assert_eq!(r_max_y, RectI16::of(i16::MIN + 9, i16::MIN + 9, i16::MAX - 9, i16::MIN));
}
