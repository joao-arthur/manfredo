use super::wrapping_inflate_assign;
use crate::cartesian::rect::rect_i64::RectI64;

#[test]
fn wrapping_inflate_assign_min_bounds() {
    let mut r = RectI64::of(7, 3, 9, 13);
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(6, 2, 10, 14));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(5, 1, 11, 15));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(4, 0, 12, 16));
}

#[test]
fn wrapping_inflate_assign_max_bounds() {
    let mut r = RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3);
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX));
}

#[test]
fn wrapping_inflate_assign_to_bounds() {
    let mut r = RectI64::of(1, 1, i64::MAX - 1, i64::MAX - 1);
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI64::largest());

    let mut r_min_x = RectI64::of(1, 10, i64::MAX - 10, i64::MAX - 10);
    wrapping_inflate_assign(&mut r_min_x);
    assert_eq!(r_min_x, RectI64::of(0, 9, i64::MAX - 9, i64::MAX - 9));

    let mut r_min_y = RectI64::of(10, 1, i64::MAX - 10, i64::MAX - 10);
    wrapping_inflate_assign(&mut r_min_y);
    assert_eq!(r_min_y, RectI64::of(9, 0, i64::MAX - 9, i64::MAX - 9));

    let mut r_max_x = RectI64::of(10, 10, i64::MAX - 1, i64::MAX - 10);
    wrapping_inflate_assign(&mut r_max_x);
    assert_eq!(r_max_x, RectI64::of(9, 9, i64::MAX, i64::MAX - 9));

    let mut r_max_y = RectI64::of(10, 10, i64::MAX - 10, i64::MAX - 1);
    wrapping_inflate_assign(&mut r_max_y);
    assert_eq!(r_max_y, RectI64::of(9, 9, i64::MAX - 9, i64::MAX));
}

#[test]
fn wrapping_inflate_assign_out_of_bounds() {
    let mut r = RectI64::largest();
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectI64::of(i64::MAX, i64::MAX, 0, 0));

    let mut r_min_x = RectI64::of(0, 10, i64::MAX - 10, i64::MAX - 10);
    wrapping_inflate_assign(&mut r_min_x);
    assert_eq!(r_min_x, RectI64::of(i64::MAX, 9, i64::MAX - 9, i64::MAX - 9));

    let mut r_min_y = RectI64::of(10, 0, i64::MAX - 10, i64::MAX - 10);
    wrapping_inflate_assign(&mut r_min_y);
    assert_eq!(r_min_y, RectI64::of(9, i64::MAX, i64::MAX - 9, i64::MAX - 9));

    let mut r_max_x = RectI64::of(10, 10, i64::MAX, i64::MAX - 10);
    wrapping_inflate_assign(&mut r_max_x);
    assert_eq!(r_max_x, RectI64::of(9, 9, 0, i64::MAX - 9));

    let mut r_max_y = RectI64::of(10, 10, i64::MAX - 10, i64::MAX);
    wrapping_inflate_assign(&mut r_max_y);
    assert_eq!(r_max_y, RectI64::of(9, 9, i64::MAX - 9, 0));
}
