use super::try_saturating_inflate;
use crate::cartesian::rect::rect_i8::RectI8;

#[test]
fn min_bounds() {
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MIN + 7, i8::MIN + 2, i8::MIN + 17, i8::MIN + 13)), Some(RectI8::of(i8::MIN + 6, i8::MIN + 1, i8::MIN + 18, i8::MIN + 14)));
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MIN + 6, i8::MIN + 1, i8::MIN + 18, i8::MIN + 14)), Some(RectI8::of(i8::MIN + 5, i8::MIN, i8::MIN + 19, i8::MIN + 15)));
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MIN + 5, i8::MIN, i8::MIN + 19, i8::MIN + 15)), Some(RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 20, i8::MIN + 17)));
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MIN + 4, i8::MIN, i8::MIN + 20, i8::MIN + 17)), Some(RectI8::of(i8::MIN + 3, i8::MIN, i8::MIN + 21, i8::MIN + 19)));
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MIN + 3, i8::MIN, i8::MIN + 21, i8::MIN + 19)), Some(RectI8::of(i8::MIN + 2, i8::MIN, i8::MIN + 22, i8::MIN + 21)));
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MIN + 2, i8::MIN, i8::MIN + 22, i8::MIN + 21)), Some(RectI8::of(i8::MIN + 1, i8::MIN, i8::MIN + 23, i8::MIN + 23)));
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MIN + 1, i8::MIN, i8::MIN + 23, i8::MIN + 23)), Some(RectI8::of(i8::MIN, i8::MIN, i8::MIN + 24, i8::MIN + 25)));
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MIN, i8::MIN, i8::MIN + 24, i8::MIN + 25)), Some(RectI8::of(i8::MIN, i8::MIN, i8::MIN + 26, i8::MIN + 27)));
}

#[test]
fn max_bounds() {
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MAX - 33, i8::MAX - 17, i8::MAX - 5, i8::MAX - 3)), Some(RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2)));
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MAX - 34, i8::MAX - 18, i8::MAX - 4, i8::MAX - 2)), Some(RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1)));
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MAX - 35, i8::MAX - 19, i8::MAX - 3, i8::MAX - 1)), Some(RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX)));
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MAX - 36, i8::MAX - 20, i8::MAX - 2, i8::MAX)), Some(RectI8::of(i8::MAX - 37, i8::MAX - 22, i8::MAX - 1, i8::MAX)));
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MAX - 37, i8::MAX - 22, i8::MAX - 1, i8::MAX)), Some(RectI8::of(i8::MAX - 38, i8::MAX - 24, i8::MAX, i8::MAX)));
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MAX - 38, i8::MAX - 24, i8::MAX, i8::MAX)), Some(RectI8::of(i8::MAX - 40, i8::MAX - 26, i8::MAX, i8::MAX)));
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MAX - 40, i8::MAX - 26, i8::MAX, i8::MAX)), Some(RectI8::of(i8::MAX - 42, i8::MAX - 28, i8::MAX, i8::MAX)));
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MAX - 42, i8::MAX - 28, i8::MAX, i8::MAX)), Some(RectI8::of(i8::MAX - 44, i8::MAX - 30, i8::MAX, i8::MAX)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX - 1, i8::MAX - 1)), Some(RectI8::largest()));
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MIN, i8::MIN, i8::MAX - 1, i8::MAX - 1)), Some(RectI8::largest()));
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MIN + 1, i8::MIN + 1, i8::MAX, i8::MAX)), Some(RectI8::largest()));
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MIN + 1, i8::MIN + 10, i8::MAX - 10, i8::MAX - 10)), Some(RectI8::of(i8::MIN, i8::MIN + 9, i8::MAX - 9, i8::MAX - 9)));
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MIN + 10, i8::MIN + 1, i8::MAX - 10, i8::MAX - 10)), Some(RectI8::of(i8::MIN + 9, i8::MIN, i8::MAX - 9, i8::MAX - 9)));
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 1, i8::MAX - 10)), Some(RectI8::of(i8::MIN + 9, i8::MIN + 9, i8::MAX, i8::MAX - 9)));
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MIN + 10, i8::MIN + 10, i8::MAX - 10, i8::MAX - 1)), Some(RectI8::of(i8::MIN + 9, i8::MIN + 9, i8::MAX - 9, i8::MAX)));
}

#[test]
fn out_of_bounds() {
    assert_eq!(try_saturating_inflate(&RectI8::largest()), None);
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MIN, i8::MIN + 10, i8::MAX, i8::MAX - 10)), None);
    assert_eq!(try_saturating_inflate(&RectI8::of(i8::MIN + 10, i8::MIN, i8::MAX - 10, i8::MAX)), None);
}
