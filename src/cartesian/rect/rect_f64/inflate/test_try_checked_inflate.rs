use super::try_checked_inflate;
use crate::cartesian::{
    point::point_f64::{MAX, MIN},
    rect::rect_f64::RectF64,
};

#[test]
fn min_bounds() {
    assert_eq!(try_checked_inflate(&RectF64::of(MIN + 7.0, MIN + 3.0, MIN + 9.0, MIN + 13.0)), Some(RectF64::of(MIN + 6.0, MIN + 2.0, MIN + 10.0, MIN + 14.0)));
    assert_eq!(try_checked_inflate(&RectF64::of(MIN + 6.0, MIN + 2.0, MIN + 10.0, MIN + 14.0)), Some(RectF64::of(MIN + 5.0, MIN + 1.0, MIN + 11.0, MIN + 15.0)));
    assert_eq!(try_checked_inflate(&RectF64::of(MIN + 5.0, MIN + 1.0, MIN + 11.0, MIN + 15.0)), Some(RectF64::of(MIN + 4.0, MIN, MIN + 12.0, MIN + 16.0)));
}

#[test]
fn max_bounds() {
    assert_eq!(try_checked_inflate(&RectF64::of(MAX - 33.0, MAX - 17.0, MAX - 5.0, MAX - 3.0)), Some(RectF64::of(MAX - 34.0, MAX - 18.0, MAX - 4.0, MAX - 2.0)));
    assert_eq!(try_checked_inflate(&RectF64::of(MAX - 34.0, MAX - 18.0, MAX - 4.0, MAX - 2.0)), Some(RectF64::of(MAX - 35.0, MAX - 19.0, MAX - 3.0, MAX - 1.0)));
    assert_eq!(try_checked_inflate(&RectF64::of(MAX - 35.0, MAX - 19.0, MAX - 3.0, MAX - 1.0)), Some(RectF64::of(MAX - 36.0, MAX - 20.0, MAX - 2.0, MAX)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_inflate(&RectF64::of(MIN + 1.0, MIN + 1.0, MAX - 1.0, MAX - 1.0)), Some(RectF64::largest()));
    assert_eq!(try_checked_inflate(&RectF64::of(MIN + 1.0, MIN + 10.0, MAX - 10.0, MAX - 10.0)), Some(RectF64::of(MIN, MIN + 9.0, MAX - 9.0, MAX - 9.0)));
    assert_eq!(try_checked_inflate(&RectF64::of(MIN + 10.0, MIN + 1.0, MAX - 10.0, MAX - 10.0)), Some(RectF64::of(MIN + 9.0, MIN, MAX - 9.0, MAX - 9.0)));
    assert_eq!(try_checked_inflate(&RectF64::of(MIN + 10.0, MIN + 10.0, MAX - 1.0, MAX - 10.0)), Some(RectF64::of(MIN + 9.0, MIN + 9.0, MAX, MAX - 9.0)));
    assert_eq!(try_checked_inflate(&RectF64::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX - 1.0)), Some(RectF64::of(MIN + 9.0, MIN + 9.0, MAX - 9.0, MAX)));
}

#[test]
fn out_of_bounds() {
    assert_eq!(try_checked_inflate(&RectF64::largest()), None);
    assert_eq!(try_checked_inflate(&RectF64::of(MIN, MIN + 10.0, MAX - 10.0, MAX - 10.0)), None);
    assert_eq!(try_checked_inflate(&RectF64::of(MIN + 10.0, MIN, MAX - 10.0, MAX - 10.0)), None);
    assert_eq!(try_checked_inflate(&RectF64::of(MIN + 10.0, MIN + 10.0, MAX, MAX - 10.0)), None);
    assert_eq!(try_checked_inflate(&RectF64::of(MIN + 10.0, MIN + 10.0, MAX - 10.0, MAX)), None);
}
