use super::checked_inflate;
use crate::cartesian::rect::rect_u8::RectU8;

#[test]
fn checked_inflate_min_bounds() {
    assert_eq!(checked_inflate(&RectU8::of(7, 3, 9, 13)), RectU8::of(6, 2, 10, 14));
    assert_eq!(checked_inflate(&RectU8::of(6, 2, 10, 14)), RectU8::of(5, 1, 11, 15));
    assert_eq!(checked_inflate(&RectU8::of(5, 1, 11, 15)), RectU8::of(4, 0, 12, 16));
}

#[test]
fn checked_inflate_max_bounds() {
    assert_eq!(
        checked_inflate(&RectU8::of(u8::MAX - 33, u8::MAX - 17, u8::MAX - 5, u8::MAX - 3)),
        RectU8::of(u8::MAX - 34, u8::MAX - 18, u8::MAX - 4, u8::MAX - 2)
    );
    assert_eq!(
        checked_inflate(&RectU8::of(u8::MAX - 34, u8::MAX - 18, u8::MAX - 4, u8::MAX - 2)),
        RectU8::of(u8::MAX - 35, u8::MAX - 19, u8::MAX - 3, u8::MAX - 1)
    );
    assert_eq!(
        checked_inflate(&RectU8::of(u8::MAX - 35, u8::MAX - 19, u8::MAX - 3, u8::MAX - 1)),
        RectU8::of(u8::MAX - 36, u8::MAX - 20, u8::MAX - 2, u8::MAX)
    );
}
