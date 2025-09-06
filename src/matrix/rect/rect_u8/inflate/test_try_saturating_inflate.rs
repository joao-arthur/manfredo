use super::try_saturating_inflate;
use crate::matrix::rect::rect_u8::RectU8;

#[test]
fn min_bounds() {
    assert_eq!(try_saturating_inflate(&RectU8::of(7, 2, 17, 13)), Some(RectU8::of(6, 1, 18, 14)));
    assert_eq!(try_saturating_inflate(&RectU8::of(6, 1, 18, 14)), Some(RectU8::of(5, 0, 19, 15)));
    assert_eq!(try_saturating_inflate(&RectU8::of(5, 0, 19, 15)), Some(RectU8::of(4, 0, 20, 17)));
    assert_eq!(try_saturating_inflate(&RectU8::of(4, 0, 20, 17)), Some(RectU8::of(3, 0, 21, 19)));
    assert_eq!(try_saturating_inflate(&RectU8::of(3, 0, 21, 19)), Some(RectU8::of(2, 0, 22, 21)));
    assert_eq!(try_saturating_inflate(&RectU8::of(2, 0, 22, 21)), Some(RectU8::of(1, 0, 23, 23)));
    assert_eq!(try_saturating_inflate(&RectU8::of(1, 0, 23, 23)), Some(RectU8::of(0, 0, 24, 25)));
    assert_eq!(try_saturating_inflate(&RectU8::of(0, 0, 24, 25)), Some(RectU8::of(0, 0, 26, 27)));
}

#[test]
fn max_bounds() {
    assert_eq!(try_saturating_inflate(&RectU8::of(u8::MAX - 33, u8::MAX - 17, u8::MAX - 5, u8::MAX - 3)), Some(RectU8::of(u8::MAX - 34, u8::MAX - 18, u8::MAX - 4, u8::MAX - 2)));
    assert_eq!(try_saturating_inflate(&RectU8::of(u8::MAX - 34, u8::MAX - 18, u8::MAX - 4, u8::MAX - 2)), Some(RectU8::of(u8::MAX - 35, u8::MAX - 19, u8::MAX - 3, u8::MAX - 1)));
    assert_eq!(try_saturating_inflate(&RectU8::of(u8::MAX - 35, u8::MAX - 19, u8::MAX - 3, u8::MAX - 1)), Some(RectU8::of(u8::MAX - 36, u8::MAX - 20, u8::MAX - 2, u8::MAX)));
    assert_eq!(try_saturating_inflate(&RectU8::of(u8::MAX - 36, u8::MAX - 20, u8::MAX - 2, u8::MAX)), Some(RectU8::of(u8::MAX - 37, u8::MAX - 22, u8::MAX - 1, u8::MAX)));
    assert_eq!(try_saturating_inflate(&RectU8::of(u8::MAX - 37, u8::MAX - 22, u8::MAX - 1, u8::MAX)), Some(RectU8::of(u8::MAX - 38, u8::MAX - 24, u8::MAX, u8::MAX)));
    assert_eq!(try_saturating_inflate(&RectU8::of(u8::MAX - 38, u8::MAX - 24, u8::MAX, u8::MAX)), Some(RectU8::of(u8::MAX - 40, u8::MAX - 26, u8::MAX, u8::MAX)));
    assert_eq!(try_saturating_inflate(&RectU8::of(u8::MAX - 40, u8::MAX - 26, u8::MAX, u8::MAX)), Some(RectU8::of(u8::MAX - 42, u8::MAX - 28, u8::MAX, u8::MAX)));
    assert_eq!(try_saturating_inflate(&RectU8::of(u8::MAX - 42, u8::MAX - 28, u8::MAX, u8::MAX)), Some(RectU8::of(u8::MAX - 44, u8::MAX - 30, u8::MAX, u8::MAX)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_saturating_inflate(&RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1)), Some(RectU8::largest()));
    assert_eq!(try_saturating_inflate(&RectU8::of(0, 0, u8::MAX - 1, u8::MAX - 1)), Some(RectU8::largest()));
    assert_eq!(try_saturating_inflate(&RectU8::of(1, 1, u8::MAX, u8::MAX)), Some(RectU8::largest()));
    assert_eq!(try_saturating_inflate(&RectU8::of(1, 10, u8::MAX - 10, u8::MAX - 10)), Some(RectU8::of(0, 9, u8::MAX - 9, u8::MAX - 9)));
    assert_eq!(try_saturating_inflate(&RectU8::of(10, 1, u8::MAX - 10, u8::MAX - 10)), Some(RectU8::of(9, 0, u8::MAX - 9, u8::MAX - 9)));
    assert_eq!(try_saturating_inflate(&RectU8::of(10, 10, u8::MAX - 1, u8::MAX - 10)), Some(RectU8::of(9, 9, u8::MAX, u8::MAX - 9)));
    assert_eq!(try_saturating_inflate(&RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 1)), Some(RectU8::of(9, 9, u8::MAX - 9, u8::MAX)));
}

#[test]
fn out_of_bounds() {
    assert_eq!(try_saturating_inflate(&RectU8::largest()), None);
    assert_eq!(try_saturating_inflate(&RectU8::of(0, 10, u8::MAX, u8::MAX - 10)), None);
    assert_eq!(try_saturating_inflate(&RectU8::of(10, 0, u8::MAX - 10, u8::MAX)), None);
}
