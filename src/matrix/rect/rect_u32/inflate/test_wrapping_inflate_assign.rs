use super::wrapping_inflate_assign;
use crate::matrix::rect::rect_u32::RectU32;

#[test]
fn min_bounds() {
    let mut r = RectU32::of(7, 3, 9, 13);
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU32::of(6, 2, 10, 14));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU32::of(5, 1, 11, 15));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU32::of(4, 0, 12, 16));
}

#[test]
fn max_bounds() {
    let mut r = RectU32::of(u32::MAX - 33, u32::MAX - 17, u32::MAX - 5, u32::MAX - 3);
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU32::of(u32::MAX - 36, u32::MAX - 20, u32::MAX - 2, u32::MAX));
}

#[test]
fn to_bounds() {
    let mut r = RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1);
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU32::largest());

    let mut r_min_row = RectU32::of(1, 10, u32::MAX - 10, u32::MAX - 10);
    wrapping_inflate_assign(&mut r_min_row);
    assert_eq!(r_min_row, RectU32::of(0, 9, u32::MAX - 9, u32::MAX - 9));

    let mut r_min_y = RectU32::of(10, 1, u32::MAX - 10, u32::MAX - 10);
    wrapping_inflate_assign(&mut r_min_y);
    assert_eq!(r_min_y, RectU32::of(9, 0, u32::MAX - 9, u32::MAX - 9));

    let mut r_max_x = RectU32::of(10, 10, u32::MAX - 1, u32::MAX - 10);
    wrapping_inflate_assign(&mut r_max_x);
    assert_eq!(r_max_x, RectU32::of(9, 9, u32::MAX, u32::MAX - 9));

    let mut r_max_y = RectU32::of(10, 10, u32::MAX - 10, u32::MAX - 1);
    wrapping_inflate_assign(&mut r_max_y);
    assert_eq!(r_max_y, RectU32::of(9, 9, u32::MAX - 9, u32::MAX));
}

#[test]
fn out_of_bounds() {
    let mut r = RectU32::largest();
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU32::of(u32::MAX, u32::MAX, 0, 0));

    let mut r_min_row = RectU32::of(0, 10, u32::MAX - 10, u32::MAX - 10);
    wrapping_inflate_assign(&mut r_min_row);
    assert_eq!(r_min_row, RectU32::of(u32::MAX, 9, u32::MAX - 9, u32::MAX - 9));

    let mut r_min_y = RectU32::of(10, 0, u32::MAX - 10, u32::MAX - 10);
    wrapping_inflate_assign(&mut r_min_y);
    assert_eq!(r_min_y, RectU32::of(9, u32::MAX, u32::MAX - 9, u32::MAX - 9));

    let mut r_max_x = RectU32::of(10, 10, u32::MAX, u32::MAX - 10);
    wrapping_inflate_assign(&mut r_max_x);
    assert_eq!(r_max_x, RectU32::of(9, 9, 0, u32::MAX - 9));

    let mut r_max_y = RectU32::of(10, 10, u32::MAX - 10, u32::MAX);
    wrapping_inflate_assign(&mut r_max_y);
    assert_eq!(r_max_y, RectU32::of(9, 9, u32::MAX - 9, 0));
}
