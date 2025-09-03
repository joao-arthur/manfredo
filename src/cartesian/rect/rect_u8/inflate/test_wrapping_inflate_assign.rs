use super::wrapping_inflate_assign;
use crate::cartesian::rect::rect_u8::RectU8;

#[test]
fn wrapping_inflate_assign_min_bounds() {
    let mut r = RectU8::of(7, 3, 9, 13);
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU8::of(6, 2, 10, 14));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU8::of(5, 1, 11, 15));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU8::of(4, 0, 12, 16));
}

#[test]
fn wrapping_inflate_assign_max_bounds() {
    let mut r = RectU8::of(u8::MAX - 33, u8::MAX - 17, u8::MAX - 5, u8::MAX - 3);
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU8::of(u8::MAX - 34, u8::MAX - 18, u8::MAX - 4, u8::MAX - 2));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU8::of(u8::MAX - 35, u8::MAX - 19, u8::MAX - 3, u8::MAX - 1));
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU8::of(u8::MAX - 36, u8::MAX - 20, u8::MAX - 2, u8::MAX));
}

#[test]
fn wrapping_inflate_assign_to_bounds() {
    let mut r = RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1);
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU8::largest());

    let mut r_min_x = RectU8::of(1, 10, u8::MAX - 10, u8::MAX - 10);
    wrapping_inflate_assign(&mut r_min_x);
    assert_eq!(r_min_x, RectU8::of(0, 9, u8::MAX - 9, u8::MAX - 9));

    let mut r_min_y = RectU8::of(10, 1, u8::MAX - 10, u8::MAX - 10);
    wrapping_inflate_assign(&mut r_min_y);
    assert_eq!(r_min_y, RectU8::of(9, 0, u8::MAX - 9, u8::MAX - 9));

    let mut r_max_x = RectU8::of(10, 10, u8::MAX - 1, u8::MAX - 10);
    wrapping_inflate_assign(&mut r_max_x);
    assert_eq!(r_max_x, RectU8::of(9, 9, u8::MAX, u8::MAX - 9));

    let mut r_max_y = RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 1);
    wrapping_inflate_assign(&mut r_max_y);
    assert_eq!(r_max_y, RectU8::of(9, 9, u8::MAX - 9, u8::MAX));
}

#[test]
fn wrapping_inflate_assign_out_of_bounds() {
    let mut r = RectU8::largest();
    wrapping_inflate_assign(&mut r);
    assert_eq!(r, RectU8::of(u8::MAX, u8::MAX, 0, 0));

    let mut r_min_x = RectU8::of(0, 10, u8::MAX - 10, u8::MAX - 10);
    wrapping_inflate_assign(&mut r_min_x);
    assert_eq!(r_min_x, RectU8::of(u8::MAX, 9, u8::MAX - 9, u8::MAX - 9));

    let mut r_min_y = RectU8::of(10, 0, u8::MAX - 10, u8::MAX - 10);
    wrapping_inflate_assign(&mut r_min_y);
    assert_eq!(r_min_y, RectU8::of(9, u8::MAX, u8::MAX - 9, u8::MAX - 9));

    let mut r_max_x = RectU8::of(10, 10, u8::MAX, u8::MAX - 10);
    wrapping_inflate_assign(&mut r_max_x);
    assert_eq!(r_max_x, RectU8::of(9, 9, 0, u8::MAX - 9));

    let mut r_max_y = RectU8::of(10, 10, u8::MAX - 10, u8::MAX);
    wrapping_inflate_assign(&mut r_max_y);
    assert_eq!(r_max_y, RectU8::of(9, 9, u8::MAX - 9, 0));
}
