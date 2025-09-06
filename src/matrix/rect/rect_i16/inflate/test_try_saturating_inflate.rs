use super::try_saturating_inflate;
use crate::matrix::rect::rect_i16::RectI16;

#[test]
fn min_bounds() {
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN + 7, i16::MIN + 2, i16::MIN + 17, i16::MIN + 13)), Some(RectI16::of(i16::MIN + 6, i16::MIN + 1, i16::MIN + 18, i16::MIN + 14)));
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN + 6, i16::MIN + 1, i16::MIN + 18, i16::MIN + 14)), Some(RectI16::of(i16::MIN + 5, i16::MIN, i16::MIN + 19, i16::MIN + 15)));
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN + 5, i16::MIN, i16::MIN + 19, i16::MIN + 15)), Some(RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 20, i16::MIN + 17)));
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN + 4, i16::MIN, i16::MIN + 20, i16::MIN + 17)), Some(RectI16::of(i16::MIN + 3, i16::MIN, i16::MIN + 21, i16::MIN + 19)));
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN + 3, i16::MIN, i16::MIN + 21, i16::MIN + 19)), Some(RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 22, i16::MIN + 21)));
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN + 2, i16::MIN, i16::MIN + 22, i16::MIN + 21)), Some(RectI16::of(i16::MIN + 1, i16::MIN, i16::MIN + 23, i16::MIN + 23)));
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN + 1, i16::MIN, i16::MIN + 23, i16::MIN + 23)), Some(RectI16::of(i16::MIN, i16::MIN, i16::MIN + 24, i16::MIN + 25)));
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN, i16::MIN, i16::MIN + 24, i16::MIN + 25)), Some(RectI16::of(i16::MIN, i16::MIN, i16::MIN + 26, i16::MIN + 27)));
}

#[test]
fn max_bounds() {
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MAX - 33, i16::MAX - 17, i16::MAX - 5, i16::MAX - 3)), Some(RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2)));
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MAX - 34, i16::MAX - 18, i16::MAX - 4, i16::MAX - 2)), Some(RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1)));
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MAX - 35, i16::MAX - 19, i16::MAX - 3, i16::MAX - 1)), Some(RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX)));
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MAX - 36, i16::MAX - 20, i16::MAX - 2, i16::MAX)), Some(RectI16::of(i16::MAX - 37, i16::MAX - 22, i16::MAX - 1, i16::MAX)));
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MAX - 37, i16::MAX - 22, i16::MAX - 1, i16::MAX)), Some(RectI16::of(i16::MAX - 38, i16::MAX - 24, i16::MAX, i16::MAX)));
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MAX - 38, i16::MAX - 24, i16::MAX, i16::MAX)), Some(RectI16::of(i16::MAX - 40, i16::MAX - 26, i16::MAX, i16::MAX)));
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MAX - 40, i16::MAX - 26, i16::MAX, i16::MAX)), Some(RectI16::of(i16::MAX - 42, i16::MAX - 28, i16::MAX, i16::MAX)));
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MAX - 42, i16::MAX - 28, i16::MAX, i16::MAX)), Some(RectI16::of(i16::MAX - 44, i16::MAX - 30, i16::MAX, i16::MAX)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX - 1, i16::MAX - 1)), Some(RectI16::largest()));
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN, i16::MIN, i16::MAX - 1, i16::MAX - 1)), Some(RectI16::largest()));
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN + 1, i16::MIN + 1, i16::MAX, i16::MAX)), Some(RectI16::largest()));
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN + 1, i16::MIN + 10, i16::MAX - 10, i16::MAX - 10)), Some(RectI16::of(i16::MIN, i16::MIN + 9, i16::MAX - 9, i16::MAX - 9)));
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN + 10, i16::MIN + 1, i16::MAX - 10, i16::MAX - 10)), Some(RectI16::of(i16::MIN + 9, i16::MIN, i16::MAX - 9, i16::MAX - 9)));
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 1, i16::MAX - 10)), Some(RectI16::of(i16::MIN + 9, i16::MIN + 9, i16::MAX, i16::MAX - 9)));
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN + 10, i16::MIN + 10, i16::MAX - 10, i16::MAX - 1)), Some(RectI16::of(i16::MIN + 9, i16::MIN + 9, i16::MAX - 9, i16::MAX)));
}

#[test]
fn out_of_bounds() {
    assert_eq!(try_saturating_inflate(&RectI16::largest()), None);
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN, i16::MIN + 10, i16::MAX, i16::MAX - 10)), None);
    assert_eq!(try_saturating_inflate(&RectI16::of(i16::MIN + 10, i16::MIN, i16::MAX - 10, i16::MAX)), None);
}
