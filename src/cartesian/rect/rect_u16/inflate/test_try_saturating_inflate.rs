use super::try_saturating_inflate;
use crate::cartesian::rect::rect_u16::RectU16;

#[test]
fn try_saturating_inflate_min_bounds() {
    assert_eq!(try_saturating_inflate(&RectU16::of(7, 2, 17, 13)), Some(RectU16::of(6, 1, 18, 14)));
    assert_eq!(try_saturating_inflate(&RectU16::of(6, 1, 18, 14)), Some(RectU16::of(5, 0, 19, 15)));
    assert_eq!(try_saturating_inflate(&RectU16::of(5, 0, 19, 15)), Some(RectU16::of(4, 0, 20, 17)));
    assert_eq!(try_saturating_inflate(&RectU16::of(4, 0, 20, 17)), Some(RectU16::of(3, 0, 21, 19)));
    assert_eq!(try_saturating_inflate(&RectU16::of(3, 0, 21, 19)), Some(RectU16::of(2, 0, 22, 21)));
    assert_eq!(try_saturating_inflate(&RectU16::of(2, 0, 22, 21)), Some(RectU16::of(1, 0, 23, 23)));
    assert_eq!(try_saturating_inflate(&RectU16::of(1, 0, 23, 23)), Some(RectU16::of(0, 0, 24, 25)));
    assert_eq!(try_saturating_inflate(&RectU16::of(0, 0, 24, 25)), Some(RectU16::of(0, 0, 26, 27)));
}

#[test]
fn try_saturating_inflate_max_bounds() {
    assert_eq!(try_saturating_inflate(&RectU16::of(u16::MAX - 33, u16::MAX - 17, u16::MAX - 5, u16::MAX - 3)), Some(RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2)));
    assert_eq!(try_saturating_inflate(&RectU16::of(u16::MAX - 34, u16::MAX - 18, u16::MAX - 4, u16::MAX - 2)), Some(RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1)));
    assert_eq!(try_saturating_inflate(&RectU16::of(u16::MAX - 35, u16::MAX - 19, u16::MAX - 3, u16::MAX - 1)), Some(RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX)));
    assert_eq!(try_saturating_inflate(&RectU16::of(u16::MAX - 36, u16::MAX - 20, u16::MAX - 2, u16::MAX)), Some(RectU16::of(u16::MAX - 37, u16::MAX - 22, u16::MAX - 1, u16::MAX)));
    assert_eq!(try_saturating_inflate(&RectU16::of(u16::MAX - 37, u16::MAX - 22, u16::MAX - 1, u16::MAX)), Some(RectU16::of(u16::MAX - 38, u16::MAX - 24, u16::MAX, u16::MAX)));
    assert_eq!(try_saturating_inflate(&RectU16::of(u16::MAX - 38, u16::MAX - 24, u16::MAX, u16::MAX)), Some(RectU16::of(u16::MAX - 40, u16::MAX - 26, u16::MAX, u16::MAX)));
    assert_eq!(try_saturating_inflate(&RectU16::of(u16::MAX - 40, u16::MAX - 26, u16::MAX, u16::MAX)), Some(RectU16::of(u16::MAX - 42, u16::MAX - 28, u16::MAX, u16::MAX)));
    assert_eq!(try_saturating_inflate(&RectU16::of(u16::MAX - 42, u16::MAX - 28, u16::MAX, u16::MAX)), Some(RectU16::of(u16::MAX - 44, u16::MAX - 30, u16::MAX, u16::MAX)));
}

#[test]
fn try_saturating_inflate_to_bounds() {
    assert_eq!(try_saturating_inflate(&RectU16::of(1, 1, u16::MAX - 1, u16::MAX - 1)), Some(RectU16::largest()));
    assert_eq!(try_saturating_inflate(&RectU16::of(0, 0, u16::MAX - 1, u16::MAX - 1)), Some(RectU16::largest()));
    assert_eq!(try_saturating_inflate(&RectU16::of(1, 1, u16::MAX, u16::MAX)), Some(RectU16::largest()));
    assert_eq!(try_saturating_inflate(&RectU16::of(1, 10, u16::MAX - 10, u16::MAX - 10)), Some(RectU16::of(0, 9, u16::MAX - 9, u16::MAX - 9)));
    assert_eq!(try_saturating_inflate(&RectU16::of(10, 1, u16::MAX - 10, u16::MAX - 10)), Some(RectU16::of(9, 0, u16::MAX - 9, u16::MAX - 9)));
    assert_eq!(try_saturating_inflate(&RectU16::of(10, 10, u16::MAX - 1, u16::MAX - 10)), Some(RectU16::of(9, 9, u16::MAX, u16::MAX - 9)));
    assert_eq!(try_saturating_inflate(&RectU16::of(10, 10, u16::MAX - 10, u16::MAX - 1)), Some(RectU16::of(9, 9, u16::MAX - 9, u16::MAX)));
}

#[test]
fn try_saturating_inflate_out_of_bounds() {
    assert_eq!(try_saturating_inflate(&RectU16::largest()), None);
    assert_eq!(try_saturating_inflate(&RectU16::of(0, 10, u16::MAX, u16::MAX - 10)), None);
    assert_eq!(try_saturating_inflate(&RectU16::of(10, 0, u16::MAX - 10, u16::MAX)), None);
}
