use super::try_saturating_inflate;
use crate::matrix::rect::rect_i64::RectI64;

#[test]
fn min_bounds() {
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MIN + 7, i64::MIN + 2, i64::MIN + 17, i64::MIN + 13)), Some(RectI64::of(i64::MIN + 6, i64::MIN + 1, i64::MIN + 18, i64::MIN + 14)));
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MIN + 6, i64::MIN + 1, i64::MIN + 18, i64::MIN + 14)), Some(RectI64::of(i64::MIN + 5, i64::MIN, i64::MIN + 19, i64::MIN + 15)));
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MIN + 5, i64::MIN, i64::MIN + 19, i64::MIN + 15)), Some(RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 20, i64::MIN + 17)));
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 20, i64::MIN + 17)), Some(RectI64::of(i64::MIN + 3, i64::MIN, i64::MIN + 21, i64::MIN + 19)));
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MIN + 3, i64::MIN, i64::MIN + 21, i64::MIN + 19)), Some(RectI64::of(i64::MIN + 2, i64::MIN, i64::MIN + 22, i64::MIN + 21)));
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MIN + 2, i64::MIN, i64::MIN + 22, i64::MIN + 21)), Some(RectI64::of(i64::MIN + 1, i64::MIN, i64::MIN + 23, i64::MIN + 23)));
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MIN + 1, i64::MIN, i64::MIN + 23, i64::MIN + 23)), Some(RectI64::of(i64::MIN, i64::MIN, i64::MIN + 24, i64::MIN + 25)));
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MIN, i64::MIN, i64::MIN + 24, i64::MIN + 25)), Some(RectI64::of(i64::MIN, i64::MIN, i64::MIN + 26, i64::MIN + 27)));
}

#[test]
fn max_bounds() {
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3)), Some(RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2)));
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2)), Some(RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1)));
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1)), Some(RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX)));
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX)), Some(RectI64::of(i64::MAX - 37, i64::MAX - 22, i64::MAX - 1, i64::MAX)));
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MAX - 37, i64::MAX - 22, i64::MAX - 1, i64::MAX)), Some(RectI64::of(i64::MAX - 38, i64::MAX - 24, i64::MAX, i64::MAX)));
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MAX - 38, i64::MAX - 24, i64::MAX, i64::MAX)), Some(RectI64::of(i64::MAX - 40, i64::MAX - 26, i64::MAX, i64::MAX)));
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MAX - 40, i64::MAX - 26, i64::MAX, i64::MAX)), Some(RectI64::of(i64::MAX - 42, i64::MAX - 28, i64::MAX, i64::MAX)));
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MAX - 42, i64::MAX - 28, i64::MAX, i64::MAX)), Some(RectI64::of(i64::MAX - 44, i64::MAX - 30, i64::MAX, i64::MAX)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1)), Some(RectI64::largest()));
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MIN, i64::MIN, i64::MAX - 1, i64::MAX - 1)), Some(RectI64::largest()));
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX, i64::MAX)), Some(RectI64::largest()));
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MIN + 1, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10)), Some(RectI64::of(i64::MIN, i64::MIN + 9, i64::MAX - 9, i64::MAX - 9)));
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 1, i64::MAX - 10, i64::MAX - 10)), Some(RectI64::of(i64::MIN + 9, i64::MIN, i64::MAX - 9, i64::MAX - 9)));
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 1, i64::MAX - 10)), Some(RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MAX, i64::MAX - 9)));
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX - 1)), Some(RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MAX - 9, i64::MAX)));
}

#[test]
fn out_of_bounds() {
    assert_eq!(try_saturating_inflate(&RectI64::largest()), None);
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MIN, i64::MIN + 10, i64::MAX, i64::MAX - 10)), None);
    assert_eq!(try_saturating_inflate(&RectI64::of(i64::MIN + 10, i64::MIN, i64::MAX - 10, i64::MAX)), None);
}
