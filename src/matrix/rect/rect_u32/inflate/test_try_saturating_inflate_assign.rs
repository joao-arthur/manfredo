use super::try_saturating_inflate_assign;
use crate::matrix::rect::rect_u32::RectU32;

#[test]
fn min_bounds() {
    let mut r = RectU32::of(7, 2, 17, 13);
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU32::of(6, 1, 18, 14));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU32::of(5, 0, 19, 15));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU32::of(4, 0, 20, 17));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU32::of(3, 0, 21, 19));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU32::of(2, 0, 22, 21));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU32::of(1, 0, 23, 23));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU32::of(0, 0, 24, 25));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU32::of(0, 0, 26, 27));
}

#[test]
fn max_bounds() {
    let mut r = RectU32::of(u32::MAX - 33, u32::MAX - 17, u32::MAX - 5, u32::MAX - 3);
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU32::of(u32::MAX - 34, u32::MAX - 18, u32::MAX - 4, u32::MAX - 2));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU32::of(u32::MAX - 35, u32::MAX - 19, u32::MAX - 3, u32::MAX - 1));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU32::of(u32::MAX - 36, u32::MAX - 20, u32::MAX - 2, u32::MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU32::of(u32::MAX - 37, u32::MAX - 22, u32::MAX - 1, u32::MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU32::of(u32::MAX - 38, u32::MAX - 24, u32::MAX, u32::MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU32::of(u32::MAX - 40, u32::MAX - 26, u32::MAX, u32::MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU32::of(u32::MAX - 42, u32::MAX - 28, u32::MAX, u32::MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU32::of(u32::MAX - 44, u32::MAX - 30, u32::MAX, u32::MAX));
}

#[test]
fn to_bounds() {
    let mut r = RectU32::of(1, 1, u32::MAX - 1, u32::MAX - 1);
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectU32::largest());

    let mut r_min = RectU32::of(0, 0, u32::MAX - 1, u32::MAX - 1);
    assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
    assert_eq!(r_min, RectU32::largest());

    let mut r_max = RectU32::of(1, 1, u32::MAX, u32::MAX);
    assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
    assert_eq!(r_max, RectU32::largest());

    let mut r_min_x = RectU32::of(1, 10, 20, 20);
    assert_eq!(try_saturating_inflate_assign(&mut r_min_x), Some(()));
    assert_eq!(r_min_x, RectU32::of(0, 9, 21, 21));

    let mut r_min_y = RectU32::of(10, 1, 20, 20);
    assert_eq!(try_saturating_inflate_assign(&mut r_min_y), Some(()));
    assert_eq!(r_min_y, RectU32::of(9, 0, 21, 21));

    let mut r_max_x = RectU32::of(10, 10, u32::MAX - 1, 20);
    assert_eq!(try_saturating_inflate_assign(&mut r_max_x), Some(()));
    assert_eq!(r_max_x, RectU32::of(9, 9, u32::MAX, 21));

    let mut r_max_y = RectU32::of(10, 10, 20, u32::MAX - 1);
    assert_eq!(try_saturating_inflate_assign(&mut r_max_y), Some(()));
    assert_eq!(r_max_y, RectU32::of(9, 9, 21, u32::MAX));
}

#[test]
fn out_of_bounds() {
    let mut r = RectU32::largest();
    assert_eq!(try_saturating_inflate_assign(&mut r), None);
    assert_eq!(r, RectU32::largest());

    let mut r_x = RectU32::of(0, 10, u32::MAX, 20);
    assert_eq!(try_saturating_inflate_assign(&mut r_x), None);
    assert_eq!(r_x, RectU32::of(0, 10, u32::MAX, 20));

    let mut r_y = RectU32::of(10, 0, 20, u32::MAX);
    assert_eq!(try_saturating_inflate_assign(&mut r_y), None);
    assert_eq!(r_y, RectU32::of(10, 0, 20, u32::MAX));
}
