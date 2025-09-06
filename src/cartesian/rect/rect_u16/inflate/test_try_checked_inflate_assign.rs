use super::try_checked_inflate_assign;
use crate::cartesian::rect::rect_u16::RectU16;

#[test]
fn min_bounds() {
    let mut r = RectU16::of(7, 3, 9, 13);
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::of(6, 2, 10, 14));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::of(5, 1, 11, 15));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::of(4, 0, 12, 16));
}

#[test]
fn max_bounds() {
    let mut r = RectU16::of(u16::MAX - 33, u16::MAX - 17, u16::MAX - 5, u16::MAX - 3);
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX));
}

#[test]
fn to_bounds() {
    let mut r = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::largest());

    let mut r_min_x = RectU16::of(1, 10, u16::MAX - 10, u16::MAX - 10);
    assert_eq!(try_checked_inflate_assign(&mut r_min_x), Some(()));
    assert_eq!(r_min_x, RectU16::of(0, 9, u16::MAX - 9, u16::MAX - 9));

    let mut r_min_y = RectU16::of(10, 1, u16::MAX - 10, u16::MAX - 10);
    assert_eq!(try_checked_inflate_assign(&mut r_min_y), Some(()));
    assert_eq!(r_min_y, RectU16::of(9, 0, u16::MAX - 9, u16::MAX - 9));

    let mut r_max_x = RectU16::of(10, 10, u16::MAX - 1, u16::MAX - 10);
    assert_eq!(try_checked_inflate_assign(&mut r_max_x), Some(()));
    assert_eq!(r_max_x, RectU16::of(9, 9, u16::MAX, u16::MAX - 9));

    let mut r_max_y = RectU16::of(10, 10, u16::MAX - 10, u16::MAX - 1);
    assert_eq!(try_checked_inflate_assign(&mut r_max_y), Some(()));
    assert_eq!(r_max_y, RectU16::of(9, 9, u16::MAX - 9, u16::MAX));
}

#[test]
fn out_of_bounds() {
    let mut r = RectU16::largest();
    assert_eq!(try_checked_inflate_assign(&mut r), None);
    assert_eq!(r, RectU16::largest());

    let mut r_min_x = RectU16::of(0, 9, u16::MAX - 9, u16::MAX - 9);
    assert_eq!(try_checked_inflate_assign(&mut r_min_x), None);
    assert_eq!(r_min_x, RectU16::of(0, 9, u16::MAX - 9, u16::MAX - 9));

    let mut r_min_y = RectU16::of(9, 0, u16::MAX - 9, u16::MAX - 9);
    assert_eq!(try_checked_inflate_assign(&mut r_min_y), None);
    assert_eq!(r_min_y, RectU16::of(9, 0, u16::MAX - 9, u16::MAX - 9));

    let mut r_max_x = RectU16::of(9, 9, u16::MAX, u16::MAX - 9);
    assert_eq!(try_checked_inflate_assign(&mut r_max_x), None);
    assert_eq!(r_max_x, RectU16::of(9, 9, u16::MAX, u16::MAX - 9));

    let mut r_max_y = RectU16::of(9, 9, u16::MAX - 9, u16::MAX);
    assert_eq!(try_checked_inflate_assign(&mut r_max_y), None);
    assert_eq!(r_max_y, RectU16::of(9, 9, u16::MAX - 9, u16::MAX));
}
