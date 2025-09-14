use super::saturating_inflate_assign;
use crate::cartesian::rect::rect_i8::Rect;

const MIN: i8 = i8::MIN;
const MAX: i8 = i8::MAX;

#[test]
fn min_bounds() {
    let mut r = Rect::of(MIN + 7, MIN + 2, MIN + 17, MIN + 13);
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MIN + 6, MIN + 1, MIN + 18, MIN + 14));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MIN + 5, MIN, MIN + 19, MIN + 15));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MIN + 4, MIN, MIN + 20, MIN + 17));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MIN + 3, MIN, MIN + 21, MIN + 19));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MIN + 2, MIN, MIN + 22, MIN + 21));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MIN + 1, MIN, MIN + 23, MIN + 23));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MIN, MIN, MIN + 24, MIN + 25));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MIN, MIN, MIN + 26, MIN + 27));
}

#[test]
fn max_bounds() {
    let mut r = Rect::of(MAX - 33, MAX - 17, MAX - 5, MAX - 3);
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MAX - 36, MAX - 20, MAX - 2, MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MAX - 37, MAX - 22, MAX - 1, MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MAX - 38, MAX - 24, MAX, MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MAX - 40, MAX - 26, MAX, MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MAX - 42, MAX - 28, MAX, MAX));
    saturating_inflate_assign(&mut r);
    assert_eq!(r, Rect::of(MAX - 44, MAX - 30, MAX, MAX));
}
