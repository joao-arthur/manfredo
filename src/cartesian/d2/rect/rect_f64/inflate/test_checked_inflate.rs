use super::checked_inflate;
use crate::cartesian::{
    d1::point::point_f64::{MAX, MIN},
    d2::rect::rect_f64::Rect,
};

#[test]
fn min_bounds() {
    assert_eq!(checked_inflate(&Rect::of(MIN + 7.0, MIN + 3.0, MIN + 9.0, MIN + 13.0)), Rect::of(MIN + 6.0, MIN + 2.0, MIN + 10.0, MIN + 14.0));
    assert_eq!(checked_inflate(&Rect::of(MIN + 6.0, MIN + 2.0, MIN + 10.0, MIN + 14.0)), Rect::of(MIN + 5.0, MIN + 1.0, MIN + 11.0, MIN + 15.0));
    assert_eq!(checked_inflate(&Rect::of(MIN + 5.0, MIN + 1.0, MIN + 11.0, MIN + 15.0)), Rect::of(MIN + 4.0, MIN, MIN + 12.0, MIN + 16.0));
}

#[test]
fn max_bounds() {
    assert_eq!(checked_inflate(&Rect::of(MAX - 33.0, MAX - 17.0, MAX - 5.0, MAX - 3.0)), Rect::of(MAX - 34.0, MAX - 18.0, MAX - 4.0, MAX - 2.0));
    assert_eq!(checked_inflate(&Rect::of(MAX - 34.0, MAX - 18.0, MAX - 4.0, MAX - 2.0)), Rect::of(MAX - 35.0, MAX - 19.0, MAX - 3.0, MAX - 1.0));
    assert_eq!(checked_inflate(&Rect::of(MAX - 35.0, MAX - 19.0, MAX - 3.0, MAX - 1.0)), Rect::of(MAX - 36.0, MAX - 20.0, MAX - 2.0, MAX));
}
