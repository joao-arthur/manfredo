use super::wrapping_inflate;
use crate::cartesian::rect::rect_i64::RectI64;

#[test]
fn wrapping_inflate_min_bounds() {
    assert_eq!(wrapping_inflate(&RectI64::of(7, 3, 9, 13)), RectI64::of(6, 2, 10, 14));
    assert_eq!(wrapping_inflate(&RectI64::of(6, 2, 10, 14)), RectI64::of(5, 1, 11, 15));
    assert_eq!(wrapping_inflate(&RectI64::of(5, 1, 11, 15)), RectI64::of(4, 0, 12, 16));
}

#[test]
fn wrapping_inflate_max_bounds() {
    assert_eq!(
        wrapping_inflate(&RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3)),
        RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2)
    );
    assert_eq!(
        wrapping_inflate(&RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2)),
        RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1)
    );
    assert_eq!(
        wrapping_inflate(&RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1)),
        RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX)
    );
}

#[test]
fn wrapping_inflate_to_bounds() {
    assert_eq!(wrapping_inflate(&RectI64::of(1, 1, i64::MAX - 1, i64::MAX - 1)), RectI64::largest());
    assert_eq!(wrapping_inflate(&RectI64::of(1, 10, i64::MAX - 10, i64::MAX - 10)), RectI64::of(0, 9, i64::MAX - 9, i64::MAX - 9));
    assert_eq!(wrapping_inflate(&RectI64::of(10, 1, i64::MAX - 10, i64::MAX - 10)), RectI64::of(9, 0, i64::MAX - 9, i64::MAX - 9));
    assert_eq!(wrapping_inflate(&RectI64::of(10, 10, i64::MAX - 1, i64::MAX - 10)), RectI64::of(9, 9, i64::MAX, i64::MAX - 9));
    assert_eq!(wrapping_inflate(&RectI64::of(10, 10, i64::MAX - 10, i64::MAX - 1)), RectI64::of(9, 9, i64::MAX - 9, i64::MAX));
}

#[test]
fn wrapping_inflate_out_of_bounds() {
    assert_eq!(wrapping_inflate(&RectI64::largest()), RectI64::of(i64::MAX, i64::MAX, 0, 0));
    assert_eq!(wrapping_inflate(&RectI64::of(0, 10, i64::MAX - 10, i64::MAX - 10)), RectI64::of(i64::MAX, 9, i64::MAX - 9, i64::MAX - 9));
    assert_eq!(wrapping_inflate(&RectI64::of(10, 0, i64::MAX - 10, i64::MAX - 10)), RectI64::of(9, i64::MAX, i64::MAX - 9, i64::MAX - 9));
    assert_eq!(wrapping_inflate(&RectI64::of(10, 10, i64::MAX, i64::MAX - 10)), RectI64::of(9, 9, 0, i64::MAX - 9));
    assert_eq!(wrapping_inflate(&RectI64::of(10, 10, i64::MAX - 10, i64::MAX)), RectI64::of(9, 9, i64::MAX - 9, 0));
}
