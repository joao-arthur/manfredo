use super::wrapping_inflate;
use crate::matrix::d2::rect::rect_i8::Rect;

const MIN: i8 = i8::MIN;
const MAX: i8 = i8::MAX;

#[test]
fn min_bounds() {
    assert_eq!(wrapping_inflate(&Rect::of(MIN + 7, MIN + 3, MIN + 9, MIN + 13)), Rect::of(MIN + 6, MIN + 2, MIN + 10, MIN + 14));
    assert_eq!(wrapping_inflate(&Rect::of(MIN + 6, MIN + 2, MIN + 10, MIN + 14)), Rect::of(MIN + 5, MIN + 1, MIN + 11, MIN + 15));
    assert_eq!(wrapping_inflate(&Rect::of(MIN + 5, MIN + 1, MIN + 11, MIN + 15)), Rect::of(MIN + 4, MIN, MIN + 12, MIN + 16));
}

#[test]
fn max_bounds() {
    assert_eq!(wrapping_inflate(&Rect::of(MAX - 33, MAX - 17, MAX - 5, MAX - 3)), Rect::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2));
    assert_eq!(wrapping_inflate(&Rect::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2)), Rect::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1));
    assert_eq!(wrapping_inflate(&Rect::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1)), Rect::of(MAX - 36, MAX - 20, MAX - 2, MAX));
}

#[test]
fn to_bounds() {
    assert_eq!(wrapping_inflate(&Rect::of(MIN + 1, MIN + 1, MAX - 1, MAX - 1)), Rect::largest());
    assert_eq!(wrapping_inflate(&Rect::of(MIN + 1, MIN + 10, MAX - 10, MAX - 10)), Rect::of(MIN, MIN + 9, MAX - 9, MAX - 9));
    assert_eq!(wrapping_inflate(&Rect::of(MIN + 10, MIN + 1, MAX - 10, MAX - 10)), Rect::of(MIN + 9, MIN, MAX - 9, MAX - 9));
    assert_eq!(wrapping_inflate(&Rect::of(MIN + 10, MIN + 10, MAX - 1, MAX - 10)), Rect::of(MIN + 9, MIN + 9, MAX, MAX - 9));
    assert_eq!(wrapping_inflate(&Rect::of(MIN + 10, MIN + 10, MAX - 10, MAX - 1)), Rect::of(MIN + 9, MIN + 9, MAX - 9, MAX));
}

#[test]
fn out_of_bounds() {
    assert_eq!(wrapping_inflate(&Rect::largest()), Rect::of(MAX, MAX, MIN, MIN));
    assert_eq!(wrapping_inflate(&Rect::of(MIN, MIN + 10, MAX - 10, MAX - 10)), Rect::of(MAX, MIN + 9, MAX - 9, MAX - 9));
    assert_eq!(wrapping_inflate(&Rect::of(MIN + 10, MIN, MAX - 10, MAX - 10)), Rect::of(MIN + 9, MAX, MAX - 9, MAX - 9));
    assert_eq!(wrapping_inflate(&Rect::of(MIN + 10, MIN + 10, MAX, MAX - 10)), Rect::of(MIN + 9, MIN + 9, MIN, MAX - 9));
    assert_eq!(wrapping_inflate(&Rect::of(MIN + 10, MIN + 10, MAX - 10, MAX)), Rect::of(MIN + 9, MIN + 9, MAX - 9, MIN));
}
