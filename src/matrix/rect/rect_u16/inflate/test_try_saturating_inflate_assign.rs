use super::try_saturating_inflate_assign;
use crate::matrix::rect::rect_u16::RectU16;

#[test]
fn min_bounds() {
    let mut r = RectU16::of(7, 2, 17, 13);
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::of(6, 1, 18, 14));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::of(5, 0, 19, 15));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::of(4, 0, 20, 17));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::of(3, 0, 21, 19));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::of(2, 0, 22, 21));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::of(1, 0, 23, 23));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::of(0, 0, 24, 25));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::of(0, 0, 26, 27));
}

#[test]
fn max_bounds() {
    let mut r = RectU16::of(u16::MAX - 33, u16::MAX - 17, u16::MAX - 5, u16::MAX - 3);
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::of(u16::MAX - 37, u16::MAX - 22, u16::MAX - 1, u16::MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::of(u16::MAX - 38, u16::MAX - 24, u16::MAX, u16::MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::of(u16::MAX - 40, u16::MAX - 26, u16::MAX, u16::MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::of(u16::MAX - 42, u16::MAX - 28, u16::MAX, u16::MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::of(u16::MAX - 44, u16::MAX - 30, u16::MAX, u16::MAX));
}

#[test]
fn to_bounds() {
    let mut r = RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1);
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU16::largest());

    let mut r_min = RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1);
    assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
    assert_eq!(r_min, RectU16::largest());

    let mut r_max = RectU16::of(1, 1, u16::MAX, u16::MAX);
    assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
    assert_eq!(r_max, RectU16::largest());

    let mut r_min_row = RectU16::of(1, 10, 20, 20);
    assert_eq!(try_saturating_inflate_assign(&mut r_min_row), Some(()));
    assert_eq!(r_min_row, RectU16::of(0, 9, 21, 21));

    let mut r_min_y = RectU16::of(10, 1, 20, 20);
    assert_eq!(try_saturating_inflate_assign(&mut r_min_y), Some(()));
    assert_eq!(r_min_y, RectU16::of(9, 0, 21, 21));

    let mut r_max_x = RectU16::of(10, 10, u16::MAX - 1, 20);
    assert_eq!(try_saturating_inflate_assign(&mut r_max_x), Some(()));
    assert_eq!(r_max_x, RectU16::of(9, 9, u16::MAX, 21));

    let mut r_max_y = RectU16::of(10, 10, 20, u16::MAX - 1);
    assert_eq!(try_saturating_inflate_assign(&mut r_max_y), Some(()));
    assert_eq!(r_max_y, RectU16::of(9, 9, 21, u16::MAX));
}

#[test]
fn out_of_bounds() {
    let mut r = RectU16::largest();
    assert_eq!(try_saturating_inflate_assign(&mut r), None);
    assert_eq!(r, RectU16::largest());

    let mut r_x = RectU16::of(0, 10, u16::MAX, 20);
    assert_eq!(try_saturating_inflate_assign(&mut r_x), None);
    assert_eq!(r_x, RectU16::of(0, 10, u16::MAX, 20));

    let mut r_y = RectU16::of(10, 0, 20, u16::MAX);
    assert_eq!(try_saturating_inflate_assign(&mut r_y), None);
    assert_eq!(r_y, RectU16::of(10, 0, 20, u16::MAX));
}
