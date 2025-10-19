use super::saturating_inflate;
use crate::matrix::{
    d1::point::point_i64::{MAX, MIN},
    d2::rect::rect_i64::Rect,
};

#[test]
fn min_bounds() {
    assert_eq!(saturating_inflate(&Rect::of(MIN + 7, MIN + 2, MIN + 17, MIN + 13)), Rect::of(MIN + 6, MIN + 1, MIN + 18, MIN + 14));
    assert_eq!(saturating_inflate(&Rect::of(MIN + 6, MIN + 1, MIN + 18, MIN + 14)), Rect::of(MIN + 5, MIN, MIN + 19, MIN + 15));
    assert_eq!(saturating_inflate(&Rect::of(MIN + 5, MIN, MIN + 19, MIN + 15)), Rect::of(MIN + 4, MIN, MIN + 20, MIN + 17));
    assert_eq!(saturating_inflate(&Rect::of(MIN + 4, MIN, MIN + 20, MIN + 17)), Rect::of(MIN + 3, MIN, MIN + 21, MIN + 19));
    assert_eq!(saturating_inflate(&Rect::of(MIN + 3, MIN, MIN + 21, MIN + 19)), Rect::of(MIN + 2, MIN, MIN + 22, MIN + 21));
    assert_eq!(saturating_inflate(&Rect::of(MIN + 2, MIN, MIN + 22, MIN + 21)), Rect::of(MIN + 1, MIN, MIN + 23, MIN + 23));
    assert_eq!(saturating_inflate(&Rect::of(MIN + 1, MIN, MIN + 23, MIN + 23)), Rect::of(MIN, MIN, MIN + 24, MIN + 25));
    assert_eq!(saturating_inflate(&Rect::of(MIN, MIN, MIN + 24, MIN + 25)), Rect::of(MIN, MIN, MIN + 26, MIN + 27));
}

#[test]
fn max_bounds() {
    assert_eq!(saturating_inflate(&Rect::of(MAX - 33, MAX - 17, MAX - 5, MAX - 3)), Rect::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2));
    assert_eq!(saturating_inflate(&Rect::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2)), Rect::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1));
    assert_eq!(saturating_inflate(&Rect::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1)), Rect::of(MAX - 36, MAX - 20, MAX - 2, MAX));
    assert_eq!(saturating_inflate(&Rect::of(MAX - 36, MAX - 20, MAX - 2, MAX)), Rect::of(MAX - 37, MAX - 22, MAX - 1, MAX));
    assert_eq!(saturating_inflate(&Rect::of(MAX - 37, MAX - 22, MAX - 1, MAX)), Rect::of(MAX - 38, MAX - 24, MAX, MAX));
    assert_eq!(saturating_inflate(&Rect::of(MAX - 38, MAX - 24, MAX, MAX)), Rect::of(MAX - 40, MAX - 26, MAX, MAX));
    assert_eq!(saturating_inflate(&Rect::of(MAX - 40, MAX - 26, MAX, MAX)), Rect::of(MAX - 42, MAX - 28, MAX, MAX));
    assert_eq!(saturating_inflate(&Rect::of(MAX - 42, MAX - 28, MAX, MAX)), Rect::of(MAX - 44, MAX - 30, MAX, MAX));
}
