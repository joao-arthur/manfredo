use super::wrapping_inflate;
use crate::cartesian::rect::rect_u64::RectU64;

#[test]
fn wrapping_inflate_min_bounds() {
    assert_eq!(wrapping_inflate(&RectU64::of(7, 3, 9, 13)), RectU64::of(6, 2, 10, 14));
    assert_eq!(wrapping_inflate(&RectU64::of(6, 2, 10, 14)), RectU64::of(5, 1, 11, 15));
    assert_eq!(wrapping_inflate(&RectU64::of(5, 1, 11, 15)), RectU64::of(4, 0, 12, 16));
}

#[test]
fn wrapping_inflate_max_bounds() {
    assert_eq!(
        wrapping_inflate(&RectU64::of(u64::MAX - 33, u64::MAX - 17, u64::MAX - 5, u64::MAX - 3)),
        RectU64::of(u64::MAX - 34, u64::MAX - 18, u64::MAX - 4, u64::MAX - 2)
    );
    assert_eq!(
        wrapping_inflate(&RectU64::of(u64::MAX - 34, u64::MAX - 18, u64::MAX - 4, u64::MAX - 2)),
        RectU64::of(u64::MAX - 35, u64::MAX - 19, u64::MAX - 3, u64::MAX - 1)
    );
    assert_eq!(
        wrapping_inflate(&RectU64::of(u64::MAX - 35, u64::MAX - 19, u64::MAX - 3, u64::MAX - 1)),
        RectU64::of(u64::MAX - 36, u64::MAX - 20, u64::MAX - 2, u64::MAX)
    );
}

#[test]
fn wrapping_inflate_to_bounds() {
    assert_eq!(wrapping_inflate(&RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 1)), RectU64::largest());
    assert_eq!(wrapping_inflate(&RectU64::of(1, 10, u64::MAX - 10, u64::MAX - 10)), RectU64::of(0, 9, u64::MAX - 9, u64::MAX - 9));
    assert_eq!(wrapping_inflate(&RectU64::of(10, 1, u64::MAX - 10, u64::MAX - 10)), RectU64::of(9, 0, u64::MAX - 9, u64::MAX - 9));
    assert_eq!(wrapping_inflate(&RectU64::of(10, 10, u64::MAX - 1, u64::MAX - 10)), RectU64::of(9, 9, u64::MAX, u64::MAX - 9));
    assert_eq!(wrapping_inflate(&RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 1)), RectU64::of(9, 9, u64::MAX - 9, u64::MAX));
}

#[test]
fn wrapping_inflate_out_of_bounds() {
    assert_eq!(wrapping_inflate(&RectU64::largest()), RectU64::of(u64::MAX, u64::MAX, 0, 0));
    assert_eq!(wrapping_inflate(&RectU64::of(0, 10, u64::MAX - 10, u64::MAX - 10)), RectU64::of(u64::MAX, 9, u64::MAX - 9, u64::MAX - 9));
    assert_eq!(wrapping_inflate(&RectU64::of(10, 0, u64::MAX - 10, u64::MAX - 10)), RectU64::of(9, u64::MAX, u64::MAX - 9, u64::MAX - 9));
    assert_eq!(wrapping_inflate(&RectU64::of(10, 10, u64::MAX, u64::MAX - 10)), RectU64::of(9, 9, 0, u64::MAX - 9));
    assert_eq!(wrapping_inflate(&RectU64::of(10, 10, u64::MAX - 10, u64::MAX)), RectU64::of(9, 9, u64::MAX - 9, 0));
}
