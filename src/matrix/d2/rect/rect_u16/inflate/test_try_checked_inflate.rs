use super::try_checked_inflate;
use crate::matrix::d2::rect::rect_u16::Rect;

const MAX: u16 = u16::MAX;

#[test]
fn min_bounds() {
    assert_eq!(try_checked_inflate(&Rect::of(7, 3, 9, 13)), Some(Rect::of(6, 2, 10, 14)));
    assert_eq!(try_checked_inflate(&Rect::of(6, 2, 10, 14)), Some(Rect::of(5, 1, 11, 15)));
    assert_eq!(try_checked_inflate(&Rect::of(5, 1, 11, 15)), Some(Rect::of(4, 0, 12, 16)));
}

#[test]
fn max_bounds() {
    assert_eq!(try_checked_inflate(&Rect::of(MAX - 33, MAX - 17, MAX - 5, MAX - 3)), Some(Rect::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2)));
    assert_eq!(try_checked_inflate(&Rect::of(MAX - 34, MAX - 18, MAX - 4, MAX - 2)), Some(Rect::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1)));
    assert_eq!(try_checked_inflate(&Rect::of(MAX - 35, MAX - 19, MAX - 3, MAX - 1)), Some(Rect::of(MAX - 36, MAX - 20, MAX - 2, MAX)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_inflate(&Rect::of(1, 1, MAX - 1, MAX - 1)), Some(Rect::largest()));
    assert_eq!(try_checked_inflate(&Rect::of(1, 10, MAX - 10, MAX - 10)), Some(Rect::of(0, 9, MAX - 9, MAX - 9)));
    assert_eq!(try_checked_inflate(&Rect::of(10, 1, MAX - 10, MAX - 10)), Some(Rect::of(9, 0, MAX - 9, MAX - 9)));
    assert_eq!(try_checked_inflate(&Rect::of(10, 10, MAX - 1, MAX - 10)), Some(Rect::of(9, 9, MAX, MAX - 9)));
    assert_eq!(try_checked_inflate(&Rect::of(10, 10, MAX - 10, MAX - 1)), Some(Rect::of(9, 9, MAX - 9, MAX)));
}

#[test]
fn out_of_bounds() {
    assert_eq!(try_checked_inflate(&Rect::largest()), None);
    assert_eq!(try_checked_inflate(&Rect::of(0, 10, MAX - 10, MAX - 10)), None);
    assert_eq!(try_checked_inflate(&Rect::of(10, 0, MAX - 10, MAX - 10)), None);
    assert_eq!(try_checked_inflate(&Rect::of(10, 10, MAX, MAX - 10)), None);
    assert_eq!(try_checked_inflate(&Rect::of(10, 10, MAX - 10, MAX)), None);
}
