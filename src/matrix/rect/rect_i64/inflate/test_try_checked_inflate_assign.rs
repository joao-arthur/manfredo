use super::try_checked_inflate_assign;
use crate::matrix::rect::rect_i64::RectI64;

#[test]
fn min_bounds() {
    let mut r = RectI64::of(i64::MIN + 7, i64::MIN + 3, i64::MIN + 9, i64::MIN + 13);
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::of(i64::MIN + 6, i64::MIN + 2, i64::MIN + 10, i64::MIN + 14));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::of(i64::MIN + 5, i64::MIN + 1, i64::MIN + 11, i64::MIN + 15));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 12, i64::MIN + 16));
}

#[test]
fn max_bounds() {
    let mut r = RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3);
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX));
}

#[test]
fn to_bounds() {
    let mut r = RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1);
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI64::largest());

    let mut r_min_row = RectI64::of(i64::MIN + 1, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10);
    assert_eq!(try_checked_inflate_assign(&mut r_min_row), Some(()));
    assert_eq!(r_min_row, RectI64::of(i64::MIN, i64::MIN + 9, i64::MAX - 9, i64::MAX - 9));

    let mut r_min_y = RectI64::of(i64::MIN + 10, i64::MIN + 1, i64::MAX - 10, i64::MAX - 10);
    assert_eq!(try_checked_inflate_assign(&mut r_min_y), Some(()));
    assert_eq!(r_min_y, RectI64::of(i64::MIN + 9, i64::MIN, i64::MAX - 9, i64::MAX - 9));

    let mut r_max_x = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 1, i64::MAX - 10);
    assert_eq!(try_checked_inflate_assign(&mut r_max_x), Some(()));
    assert_eq!(r_max_x, RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MAX, i64::MAX - 9));

    let mut r_max_y = RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX - 1);
    assert_eq!(try_checked_inflate_assign(&mut r_max_y), Some(()));
    assert_eq!(r_max_y, RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MAX - 9, i64::MAX));
}

#[test]
fn out_of_bounds() {
    let mut r = RectI64::largest();
    assert_eq!(try_checked_inflate_assign(&mut r), None);
    assert_eq!(r, RectI64::largest());

    let mut r_min_row = RectI64::of(i64::MIN, i64::MIN + 9, i64::MAX - 9, i64::MAX - 9);
    assert_eq!(try_checked_inflate_assign(&mut r_min_row), None);
    assert_eq!(r_min_row, RectI64::of(i64::MIN, i64::MIN + 9, i64::MAX - 9, i64::MAX - 9));

    let mut r_min_y = RectI64::of(i64::MIN + 9, i64::MIN, i64::MAX - 9, i64::MAX - 9);
    assert_eq!(try_checked_inflate_assign(&mut r_min_y), None);
    assert_eq!(r_min_y, RectI64::of(i64::MIN + 9, i64::MIN, i64::MAX - 9, i64::MAX - 9));

    let mut r_max_x = RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MAX, i64::MAX - 9);
    assert_eq!(try_checked_inflate_assign(&mut r_max_x), None);
    assert_eq!(r_max_x, RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MAX, i64::MAX - 9));

    let mut r_max_y = RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MAX - 9, i64::MAX);
    assert_eq!(try_checked_inflate_assign(&mut r_max_y), None);
    assert_eq!(r_max_y, RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MAX - 9, i64::MAX));
}
