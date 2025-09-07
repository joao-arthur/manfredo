use super::try_saturating_inflate_assign;
use crate::matrix::rect::rect_i32::RectI32;

const MIN: i32 = i32::MIN;
const MAX: i32 = i32::MAX;

#[test]
fn min_bounds() {
    let mut r = RectI32::of(MIN + 7, MIN + 2, MIN + 17, MIN + 13);
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::of(MIN + 6, MIN + 1, MIN + 18, MIN + 14));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::of(MIN + 5, MIN, MIN + 19, MIN + 15));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::of(MIN + 4, MIN, MIN + 20, MIN + 17));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::of(MIN + 3, MIN, MIN + 21, MIN + 19));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::of(MIN + 2, MIN, MIN + 22, MIN + 21));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::of(MIN + 1, MIN, MIN + 23, MIN + 23));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::of(MIN, MIN, MIN + 24, MIN + 25));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::of(MIN, MIN, MIN + 26, MIN + 27));
}

#[test]
fn max_bounds() {
    let mut r = RectI32::of(MAX - 33, MAX - 17, MAX - 5, MAX - 3);
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::of(MAX - 36, MAX - 20, MAX - 2, MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::of(MAX - 37, MAX - 22, MAX - 1, MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::of(MAX - 38, MAX - 24, MAX, MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::of(MAX - 40, MAX - 26, MAX, MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::of(MAX - 42, MAX - 28, MAX, MAX));
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::of(MAX - 44, MAX - 30, MAX, MAX));
}

#[test]
fn to_bounds() {
    let mut r = RectI32::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1);
    assert_eq!(try_saturating_inflate_assign(&mut r), Some(()));
    assert_eq!(r, RectI32::largest());

    let mut r_min = RectI32::of(MIN, MIN, MAX - 1, MAX - 1);
    assert_eq!(try_saturating_inflate_assign(&mut r_min), Some(()));
    assert_eq!(r_min, RectI32::largest());

    let mut r_max = RectI32::of(MIN + 1, MIN + 1, MAX, MAX);
    assert_eq!(try_saturating_inflate_assign(&mut r_max), Some(()));
    assert_eq!(r_max, RectI32::largest());

    let mut r_min_row = RectI32::of(MIN + 1, MIN + 10, MAX - 10, MAX - 10);
    assert_eq!(try_saturating_inflate_assign(&mut r_min_row), Some(()));
    assert_eq!(r_min_row, RectI32::of(MIN, MIN + 9, MAX - 9, MAX - 9));

    let mut r_min_y = RectI32::of(MIN + 10, MIN + 1, MAX - 10, MAX - 10);
    assert_eq!(try_saturating_inflate_assign(&mut r_min_y), Some(()));
    assert_eq!(r_min_y, RectI32::of(MIN + 9, MIN, MAX - 9, MAX - 9));

    let mut r_max_x = RectI32::of(MIN + 10, MIN + 10, MAX - 1, MAX - 10);
    assert_eq!(try_saturating_inflate_assign(&mut r_max_x), Some(()));
    assert_eq!(r_max_x, RectI32::of(MIN + 9, MIN + 9, MAX, MAX - 9));

    let mut r_max_y = RectI32::of(MIN + 10, MIN + 10, MAX - 10, MAX - 1);
    assert_eq!(try_saturating_inflate_assign(&mut r_max_y), Some(()));
    assert_eq!(r_max_y, RectI32::of(MIN + 9, MIN + 9, MAX - 9, MAX));
}

#[test]
fn out_of_bounds() {
    let mut r = RectI32::largest();
    assert_eq!(try_saturating_inflate_assign(&mut r), None);
    assert_eq!(r, RectI32::largest());

    let mut r_x = RectI32::of(MIN, MIN + 10, MAX, MAX - 10);
    assert_eq!(try_saturating_inflate_assign(&mut r_x), None);
    assert_eq!(r_x, RectI32::of(MIN, MIN + 10, MAX, MAX - 10));

    let mut r_y = RectI32::of(MIN + 10, MIN, MAX - 10, MAX);
    assert_eq!(try_saturating_inflate_assign(&mut r_y), None);
    assert_eq!(r_y, RectI32::of(MIN + 10, MIN, MAX - 10, MAX));
}
