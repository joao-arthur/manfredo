use super::wrapping_inflate;
use crate::matrix::rect::rect_u8::RectU8;

#[test]
fn wrapping_inflate_min_bounds() {
    assert_eq!(wrapping_inflate(&RectU8::of(7, 3, 9, 13)), RectU8::of(6, 2, 10, 14));
    assert_eq!(wrapping_inflate(&RectU8::of(6, 2, 10, 14)), RectU8::of(5, 1, 11, 15));
    assert_eq!(wrapping_inflate(&RectU8::of(5, 1, 11, 15)), RectU8::of(4, 0, 12, 16));
}

#[test]
fn wrapping_inflate_max_bounds() {
    assert_eq!(
        wrapping_inflate(&RectU8::of(u8::MAX - 33, u8::MAX - 17, u8::MAX - 5, u8::MAX - 3)),
        RectU8::of(u8::MAX - 34, u8::MAX - 18, u8::MAX - 4, u8::MAX - 2)
    );
    assert_eq!(
        wrapping_inflate(&RectU8::of(u8::MAX - 34, u8::MAX - 18, u8::MAX - 4, u8::MAX - 2)),
        RectU8::of(u8::MAX - 35, u8::MAX - 19, u8::MAX - 3, u8::MAX - 1)
    );
    assert_eq!(
        wrapping_inflate(&RectU8::of(u8::MAX - 35, u8::MAX - 19, u8::MAX - 3, u8::MAX - 1)),
        RectU8::of(u8::MAX - 36, u8::MAX - 20, u8::MAX - 2, u8::MAX)
    );
}

#[test]
fn wrapping_inflate_to_bounds() {
    assert_eq!(wrapping_inflate(&RectU8::of(1, 1, u8::MAX - 1, u8::MAX - 1)), RectU8::largest());
    assert_eq!(wrapping_inflate(&RectU8::of(1, 10, u8::MAX - 10, u8::MAX - 10)), RectU8::of(0, 9, u8::MAX - 9, u8::MAX - 9));
    assert_eq!(wrapping_inflate(&RectU8::of(10, 1, u8::MAX - 10, u8::MAX - 10)), RectU8::of(9, 0, u8::MAX - 9, u8::MAX - 9));
    assert_eq!(wrapping_inflate(&RectU8::of(10, 10, u8::MAX - 1, u8::MAX - 10)), RectU8::of(9, 9, u8::MAX, u8::MAX - 9));
    assert_eq!(wrapping_inflate(&RectU8::of(10, 10, u8::MAX - 10, u8::MAX - 1)), RectU8::of(9, 9, u8::MAX - 9, u8::MAX));
}

#[test]
fn wrapping_inflate_out_of_bounds() {
    assert_eq!(wrapping_inflate(&RectU8::largest()), RectU8::of(u8::MAX, u8::MAX, 0, 0));
    assert_eq!(wrapping_inflate(&RectU8::of(0, 10, u8::MAX - 10, u8::MAX - 10)), RectU8::of(u8::MAX, 9, u8::MAX - 9, u8::MAX - 9));
    assert_eq!(wrapping_inflate(&RectU8::of(10, 0, u8::MAX - 10, u8::MAX - 10)), RectU8::of(9, u8::MAX, u8::MAX - 9, u8::MAX - 9));
    assert_eq!(wrapping_inflate(&RectU8::of(10, 10, u8::MAX, u8::MAX - 10)), RectU8::of(9, 9, 0, u8::MAX - 9));
    assert_eq!(wrapping_inflate(&RectU8::of(10, 10, u8::MAX - 10, u8::MAX)), RectU8::of(9, 9, u8::MAX - 9, 0));
}
