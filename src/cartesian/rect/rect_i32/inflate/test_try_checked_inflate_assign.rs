use super::try_checked_inflate_assign;
use crate::cartesian::rect::rect_i32::RectI32;

#[test]
fn min_bounds() {
    let mut r = RectI32::of(i32::MIN + 7, i32::MIN + 3, i32::MIN + 9, i32::MIN + 13);
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::of(i32::MIN + 6, i32::MIN + 2, i32::MIN + 10, i32::MIN + 14));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::of(i32::MIN + 5, i32::MIN + 1, i32::MIN + 11, i32::MIN + 15));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::of(i32::MIN + 4, i32::MIN, i32::MIN + 12, i32::MIN + 16));
}

#[test]
fn max_bounds() {
    let mut r = RectI32::of(i32::MAX - 33, i32::MAX - 17, i32::MAX - 5, i32::MAX - 3);
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::of(i32::MAX - 34, i32::MAX - 18, i32::MAX - 4, i32::MAX - 2));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::of(i32::MAX - 35, i32::MAX - 19, i32::MAX - 3, i32::MAX - 1));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::of(i32::MAX - 36, i32::MAX - 20, i32::MAX - 2, i32::MAX));
}

#[test]
fn to_bounds() {
    let mut r = RectI32::of(i32::MIN + 1, i32::MIN + 1, i32::MAX - 1, i32::MAX - 1);
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::largest());

    let mut r_min_x = RectI32::of(i32::MIN + 1, i32::MIN + 10, i32::MAX - 10, i32::MAX - 10);
    assert_eq!(try_checked_inflate_assign(&mut r_min_x), Some(()));
    assert_eq!(r_min_x, RectI32::of(i32::MIN, i32::MIN + 9, i32::MAX - 9, i32::MAX - 9));

    let mut r_min_y = RectI32::of(i32::MIN + 10, i32::MIN + 1, i32::MAX - 10, i32::MAX - 10);
    assert_eq!(try_checked_inflate_assign(&mut r_min_y), Some(()));
    assert_eq!(r_min_y, RectI32::of(i32::MIN + 9, i32::MIN, i32::MAX - 9, i32::MAX - 9));

    let mut r_max_x = RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 1, i32::MAX - 10);
    assert_eq!(try_checked_inflate_assign(&mut r_max_x), Some(()));
    assert_eq!(r_max_x, RectI32::of(i32::MIN + 9, i32::MIN + 9, i32::MAX, i32::MAX - 9));

    let mut r_max_y = RectI32::of(i32::MIN + 10, i32::MIN + 10, i32::MAX - 10, i32::MAX - 1);
    assert_eq!(try_checked_inflate_assign(&mut r_max_y), Some(()));
    assert_eq!(r_max_y, RectI32::of(i32::MIN + 9, i32::MIN + 9, i32::MAX - 9, i32::MAX));
}

#[test]
fn out_of_bounds() {
    let mut r = RectI32::largest();
    assert_eq!(try_checked_inflate_assign(&mut r), None);
    assert_eq!(r, RectI32::largest());

    let mut r_min_x = RectI32::of(i32::MIN, i32::MIN + 9, i32::MAX - 9, i32::MAX - 9);
    assert_eq!(try_checked_inflate_assign(&mut r_min_x), None);
    assert_eq!(r_min_x, RectI32::of(i32::MIN, i32::MIN + 9, i32::MAX - 9, i32::MAX - 9));

    let mut r_min_y = RectI32::of(i32::MIN + 9, i32::MIN, i32::MAX - 9, i32::MAX - 9);
    assert_eq!(try_checked_inflate_assign(&mut r_min_y), None);
    assert_eq!(r_min_y, RectI32::of(i32::MIN + 9, i32::MIN, i32::MAX - 9, i32::MAX - 9));

    let mut r_max_x = RectI32::of(i32::MIN + 9, i32::MIN + 9, i32::MAX, i32::MAX - 9);
    assert_eq!(try_checked_inflate_assign(&mut r_max_x), None);
    assert_eq!(r_max_x, RectI32::of(i32::MIN + 9, i32::MIN + 9, i32::MAX, i32::MAX - 9));

    let mut r_max_y = RectI32::of(i32::MIN + 9, i32::MIN + 9, i32::MAX - 9, i32::MAX);
    assert_eq!(try_checked_inflate_assign(&mut r_max_y), None);
    assert_eq!(r_max_y, RectI32::of(i32::MIN + 9, i32::MIN + 9, i32::MAX - 9, i32::MAX));
}
