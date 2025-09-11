use super::saturating_inflate;
use crate::cartesian::{
    point::point_f64::{MAX, MIN},
    rect::rect_f64::Rect,
};

#[test]
fn min_bounds() {
    assert_eq!(saturating_inflate(&Rect::of(MIN + 7.0, MIN + 2.0, MIN + 17.0, MIN + 13.0)), Rect::of(MIN + 6.0, MIN + 1.0, MIN + 18.0, MIN + 14.0));
    assert_eq!(saturating_inflate(&Rect::of(MIN + 6.0, MIN + 1.0, MIN + 18.0, MIN + 14.0)), Rect::of(MIN + 5.0, MIN, MIN + 19.0, MIN + 15.0));
    assert_eq!(saturating_inflate(&Rect::of(MIN + 5.0, MIN, MIN + 19.0, MIN + 15.0)), Rect::of(MIN + 4.0, MIN, MIN + 20.0, MIN + 17.0));
    assert_eq!(saturating_inflate(&Rect::of(MIN + 4.0, MIN, MIN + 20.0, MIN + 17.0)), Rect::of(MIN + 3.0, MIN, MIN + 21.0, MIN + 19.0));
    assert_eq!(saturating_inflate(&Rect::of(MIN + 3.0, MIN, MIN + 21.0, MIN + 19.0)), Rect::of(MIN + 2.0, MIN, MIN + 22.0, MIN + 21.0));
    assert_eq!(saturating_inflate(&Rect::of(MIN + 2.0, MIN, MIN + 22.0, MIN + 21.0)), Rect::of(MIN + 1.0, MIN, MIN + 23.0, MIN + 23.0));
    assert_eq!(saturating_inflate(&Rect::of(MIN + 1.0, MIN, MIN + 23.0, MIN + 23.0)), Rect::of(MIN, MIN, MIN + 24.0, MIN + 25.0));
    assert_eq!(saturating_inflate(&Rect::of(MIN, MIN, MIN + 24.0, MIN + 25.0)), Rect::of(MIN, MIN, MIN + 26.0, MIN + 27.0));
}

#[test]
fn max_bounds() {
    assert_eq!(saturating_inflate(&Rect::of(MAX - 33.0, MAX - 17.0, MAX - 5.0, MAX - 3.0)), Rect::of(MAX - 34.0, MAX - 18.0, MAX - 4.0, MAX - 2.0));
    assert_eq!(saturating_inflate(&Rect::of(MAX - 34.0, MAX - 18.0, MAX - 4.0, MAX - 2.0)), Rect::of(MAX - 35.0, MAX - 19.0, MAX - 3.0, MAX - 1.0));
    assert_eq!(saturating_inflate(&Rect::of(MAX - 35.0, MAX - 19.0, MAX - 3.0, MAX - 1.0)), Rect::of(MAX - 36.0, MAX - 20.0, MAX - 2.0, MAX));
    assert_eq!(saturating_inflate(&Rect::of(MAX - 36.0, MAX - 20.0, MAX - 2.0, MAX)), Rect::of(MAX - 37.0, MAX - 22.0, MAX - 1.0, MAX));
    assert_eq!(saturating_inflate(&Rect::of(MAX - 37.0, MAX - 22.0, MAX - 1.0, MAX)), Rect::of(MAX - 38.0, MAX - 24.0, MAX, MAX));
    assert_eq!(saturating_inflate(&Rect::of(MAX - 38.0, MAX - 24.0, MAX, MAX)), Rect::of(MAX - 40.0, MAX - 26.0, MAX, MAX));
    assert_eq!(saturating_inflate(&Rect::of(MAX - 40.0, MAX - 26.0, MAX, MAX)), Rect::of(MAX - 42.0, MAX - 28.0, MAX, MAX));
    assert_eq!(saturating_inflate(&Rect::of(MAX - 42.0, MAX - 28.0, MAX, MAX)), Rect::of(MAX - 44.0, MAX - 30.0, MAX, MAX));
}
