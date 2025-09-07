use super::wrapping_inflate_assign;
use crate::matrix::rect::rect_u64::RectU64;

const MAX: u64 = u64::MAX;

#[test]
fn min_bounds() {
    let mut r = RectU64::of(7, 3, 9, 13);
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(6, 2, 10, 14));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(5, 1, 11, 15));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(4, 0, 12, 16));
}

#[test]
fn max_bounds() {
    let mut r = RectU64::of(MAX - 33, MAX - 17, MAX - 5, MAX - 3);
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(MAX - 36, MAX - 20, MAX - 2, MAX));
}

#[test]
fn to_bounds() {
    let mut r = RectU64::of(1, 1, MAX - 1, MAX - 1);
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU64::largest());

    let mut r_min_row = RectU64::of(1, 10, MAX - 10, MAX - 10);
    wrapping_inflate_assign(&mut r_min_row);
    assert_eq!(r_min_row, RectU64::of(0, 9, MAX - 9, MAX - 9));

    let mut r_min_y = RectU64::of(10, 1, MAX - 10, MAX - 10);
    wrapping_inflate_assign(&mut r_min_y);
    assert_eq!(r_min_y, RectU64::of(9, 0, MAX - 9, MAX - 9));

    let mut r_max_x = RectU64::of(10, 10, MAX - 1, MAX - 10);
    wrapping_inflate_assign(&mut r_max_x);
    assert_eq!(r_max_x, RectU64::of(9, 9, MAX, MAX - 9));

    let mut r_max_y = RectU64::of(10, 10, MAX - 10, MAX - 1);
    wrapping_inflate_assign(&mut r_max_y);
    assert_eq!(r_max_y, RectU64::of(9, 9, MAX - 9, MAX));
}

#[test]
fn out_of_bounds() {
    let mut r = RectU64::largest();
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU64::of(MAX, MAX, 0, 0));

    let mut r_min_row = RectU64::of(0, 10, MAX - 10, MAX - 10);
    wrapping_inflate_assign(&mut r_min_row);
    assert_eq!(r_min_row, RectU64::of(MAX, 9, MAX - 9, MAX - 9));

    let mut r_min_y = RectU64::of(10, 0, MAX - 10, MAX - 10);
    wrapping_inflate_assign(&mut r_min_y);
    assert_eq!(r_min_y, RectU64::of(9, MAX, MAX - 9, MAX - 9));

    let mut r_max_x = RectU64::of(10, 10, MAX, MAX - 10);
    wrapping_inflate_assign(&mut r_max_x);
    assert_eq!(r_max_x, RectU64::of(9, 9, 0, MAX - 9));

    let mut r_max_y = RectU64::of(10, 10, MAX - 10, MAX);
    wrapping_inflate_assign(&mut r_max_y);
    assert_eq!(r_max_y, RectU64::of(9, 9, MAX - 9, 0));
}
