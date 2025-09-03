use super::try_checked_inflate;
use crate::matrix::rect::rect_u64::RectU64;

#[test]
fn try_checked_inflate_min_bounds() {
    assert_eq!(try_checked_inflate(&RectU64::of(7, 3, 9, 13)), Some(RectU64::of(6, 2, 10, 14)));
    assert_eq!(try_checked_inflate(&RectU64::of(6, 2, 10, 14)), Some(RectU64::of(5, 1, 11, 15)));
    assert_eq!(try_checked_inflate(&RectU64::of(5, 1, 11, 15)), Some(RectU64::of(4, 0, 12, 16)));
}

#[test]
fn try_checked_inflate_max_bounds() {
    assert_eq!(
        try_checked_inflate(&RectU64::of(u64::MAX - 33, u64::MAX - 17, u64::MAX - 5, u64::MAX - 3)),
        Some(RectU64::of(u64::MAX - 34, u64::MAX - 18, u64::MAX - 4, u64::MAX - 2))
    );
    assert_eq!(
        try_checked_inflate(&RectU64::of(u64::MAX - 34, u64::MAX - 18, u64::MAX - 4, u64::MAX - 2)),
        Some(RectU64::of(u64::MAX - 35, u64::MAX - 19, u64::MAX - 3, u64::MAX - 1))
    );
    assert_eq!(
        try_checked_inflate(&RectU64::of(u64::MAX - 35, u64::MAX - 19, u64::MAX - 3, u64::MAX - 1)),
        Some(RectU64::of(u64::MAX - 36, u64::MAX - 20, u64::MAX - 2, u64::MAX))
    );
}

#[test]
fn try_checked_inflate_to_bounds() {
    assert_eq!(try_checked_inflate(&RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 1)), Some(RectU64::largest()));
    assert_eq!(try_checked_inflate(&RectU64::of(1, 10, u64::MAX - 10, u64::MAX - 10)), Some(RectU64::of(0, 9, u64::MAX - 9, u64::MAX - 9)));
    assert_eq!(try_checked_inflate(&RectU64::of(10, 1, u64::MAX - 10, u64::MAX - 10)), Some(RectU64::of(9, 0, u64::MAX - 9, u64::MAX - 9)));
    assert_eq!(try_checked_inflate(&RectU64::of(10, 10, u64::MAX - 1, u64::MAX - 10)), Some(RectU64::of(9, 9, u64::MAX, u64::MAX - 9)));
    assert_eq!(try_checked_inflate(&RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 1)), Some(RectU64::of(9, 9, u64::MAX - 9, u64::MAX)));
}

#[test]
fn try_checked_inflate_out_of_bounds() {
    assert_eq!(try_checked_inflate(&RectU64::largest()), None);
    assert_eq!(try_checked_inflate(&RectU64::of(0, 10, u64::MAX - 10, u64::MAX - 10)), None);
    assert_eq!(try_checked_inflate(&RectU64::of(10, 0, u64::MAX - 10, u64::MAX - 10)), None);
    assert_eq!(try_checked_inflate(&RectU64::of(10, 10, u64::MAX, u64::MAX - 10)), None);
    assert_eq!(try_checked_inflate(&RectU64::of(10, 10, u64::MAX - 10, u64::MAX)), None);
}
