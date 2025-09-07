use super::try_saturating_inflate;
use crate::matrix::rect::rect_u64::RectU64;

const MAX: u64 = u64::MAX;

#[test]
fn min_bounds() {
    assert_eq!(try_saturating_inflate(&RectU64::of(7, 2, 17, 13)), Some(RectU64::of(6, 1, 18, 14)));
    assert_eq!(try_saturating_inflate(&RectU64::of(6, 1, 18, 14)), Some(RectU64::of(5, 0, 19, 15)));
    assert_eq!(try_saturating_inflate(&RectU64::of(5, 0, 19, 15)), Some(RectU64::of(4, 0, 20, 17)));
    assert_eq!(try_saturating_inflate(&RectU64::of(4, 0, 20, 17)), Some(RectU64::of(3, 0, 21, 19)));
    assert_eq!(try_saturating_inflate(&RectU64::of(3, 0, 21, 19)), Some(RectU64::of(2, 0, 22, 21)));
    assert_eq!(try_saturating_inflate(&RectU64::of(2, 0, 22, 21)), Some(RectU64::of(1, 0, 23, 23)));
    assert_eq!(try_saturating_inflate(&RectU64::of(1, 0, 23, 23)), Some(RectU64::of(0, 0, 24, 25)));
    assert_eq!(try_saturating_inflate(&RectU64::of(0, 0, 24, 25)), Some(RectU64::of(0, 0, 26, 27)));
}

#[test]
fn max_bounds() {
    assert_eq!(try_saturating_inflate(&RectU64::of(MAX - 33, MAX - 17, MAX - 5, MAX - 3)), Some(RectU64::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2)));
    assert_eq!(try_saturating_inflate(&RectU64::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2)), Some(RectU64::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1)));
    assert_eq!(try_saturating_inflate(&RectU64::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1)), Some(RectU64::of(MAX - 36, MAX - 20, MAX - 2, MAX)));
    assert_eq!(try_saturating_inflate(&RectU64::of(MAX - 36, MAX - 20, MAX - 2, MAX)), Some(RectU64::of(MAX - 37, MAX - 22, MAX - 1, MAX)));
    assert_eq!(try_saturating_inflate(&RectU64::of(MAX - 37, MAX - 22, MAX - 1, MAX)), Some(RectU64::of(MAX - 38, MAX - 24, MAX, MAX)));
    assert_eq!(try_saturating_inflate(&RectU64::of(MAX - 38, MAX - 24, MAX, MAX)), Some(RectU64::of(MAX - 40, MAX - 26, MAX, MAX)));
    assert_eq!(try_saturating_inflate(&RectU64::of(MAX - 40, MAX - 26, MAX, MAX)), Some(RectU64::of(MAX - 42, MAX - 28, MAX, MAX)));
    assert_eq!(try_saturating_inflate(&RectU64::of(MAX - 42, MAX - 28, MAX, MAX)), Some(RectU64::of(MAX - 44, MAX - 30, MAX, MAX)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_saturating_inflate(&RectU64::of(1, 1, MAX - 1, MAX - 1)), Some(RectU64::largest()));
    assert_eq!(try_saturating_inflate(&RectU64::of(0, 0, MAX - 1, MAX - 1)), Some(RectU64::largest()));
    assert_eq!(try_saturating_inflate(&RectU64::of(1, 1, MAX, MAX)), Some(RectU64::largest()));
    assert_eq!(try_saturating_inflate(&RectU64::of(1, 10, MAX - 10, MAX - 10)), Some(RectU64::of(0, 9, MAX - 9, MAX - 9)));
    assert_eq!(try_saturating_inflate(&RectU64::of(10, 1, MAX - 10, MAX - 10)), Some(RectU64::of(9, 0, MAX - 9, MAX - 9)));
    assert_eq!(try_saturating_inflate(&RectU64::of(10, 10, MAX - 1, MAX - 10)), Some(RectU64::of(9, 9, MAX, MAX - 9)));
    assert_eq!(try_saturating_inflate(&RectU64::of(10, 10, MAX - 10, MAX - 1)), Some(RectU64::of(9, 9, MAX - 9, MAX)));
}

#[test]
fn out_of_bounds() {
    assert_eq!(try_saturating_inflate(&RectU64::largest()), None);
    assert_eq!(try_saturating_inflate(&RectU64::of(0, 10, MAX, MAX - 10)), None);
    assert_eq!(try_saturating_inflate(&RectU64::of(10, 0, MAX - 10, MAX)), None);
}
