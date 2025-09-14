use super::try_checked_inflate_assign;
use crate::matrix::rect::rect_i64::Rect;

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

#[test]
fn min_bounds() {
    let mut r = Rect::of(MIN + 7, MIN + 3, MIN + 9, MIN + 13);
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::of(MIN + 6, MIN + 2, MIN + 10, MIN + 14));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::of(MIN + 5, MIN + 1, MIN + 11, MIN + 15));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::of(MIN + 4, MIN, MIN + 12, MIN + 16));
}

#[test]
fn max_bounds() {
    let mut r = Rect::of(MAX - 33, MAX - 17, MAX - 5, MAX - 3);
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1));
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::of(MAX - 36, MAX - 20, MAX - 2, MAX));
}

#[test]
fn to_bounds() {
    let mut r = Rect::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
    assert_eq!(try_checked_inflate_assign(&mut r), Some(()));
    assert_eq!(r, Rect::largest());

    let mut r_min_row = Rect::of(MIN + 1, MIN + 10, MAX - 10, MAX - 10);
    assert_eq!(try_checked_inflate_assign(&mut r_min_row), Some(()));
    assert_eq!(r_min_row, Rect::of(MIN, MIN + 9, MAX - 9, MAX - 9));

    let mut r_min_y = Rect::of(MIN + 10, MIN + 1, MAX - 10, MAX - 10);
    assert_eq!(try_checked_inflate_assign(&mut r_min_y), Some(()));
    assert_eq!(r_min_y, Rect::of(MIN + 9, MIN, MAX - 9, MAX - 9));

    let mut r_max_x = Rect::of(MIN + 10, MIN + 10, MAX - 1, MAX - 10);
    assert_eq!(try_checked_inflate_assign(&mut r_max_x), Some(()));
    assert_eq!(r_max_x, Rect::of(MIN + 9, MIN + 9, MAX, MAX - 9));

    let mut r_max_y = Rect::of(MIN + 10, MIN + 10, MAX - 10, MAX - 1);
    assert_eq!(try_checked_inflate_assign(&mut r_max_y), Some(()));
    assert_eq!(r_max_y, Rect::of(MIN + 9, MIN + 9, MAX - 9, MAX));
}

#[test]
fn out_of_bounds() {
    let mut r = Rect::largest();
    assert_eq!(try_checked_inflate_assign(&mut r), None);
    assert_eq!(r, Rect::largest());

    let mut r_min_row = Rect::of(MIN, MIN + 9, MAX - 9, MAX - 9);
    assert_eq!(try_checked_inflate_assign(&mut r_min_row), None);
    assert_eq!(r_min_row, Rect::of(MIN, MIN + 9, MAX - 9, MAX - 9));

    let mut r_min_y = Rect::of(MIN + 9, MIN, MAX - 9, MAX - 9);
    assert_eq!(try_checked_inflate_assign(&mut r_min_y), None);
    assert_eq!(r_min_y, Rect::of(MIN + 9, MIN, MAX - 9, MAX - 9));

    let mut r_max_x = Rect::of(MIN + 9, MIN + 9, MAX, MAX - 9);
    assert_eq!(try_checked_inflate_assign(&mut r_max_x), None);
    assert_eq!(r_max_x, Rect::of(MIN + 9, MIN + 9, MAX, MAX - 9));

    let mut r_max_y = Rect::of(MIN + 9, MIN + 9, MAX - 9, MAX);
    assert_eq!(try_checked_inflate_assign(&mut r_max_y), None);
    assert_eq!(r_max_y, Rect::of(MIN + 9, MIN + 9, MAX - 9, MAX));
}
