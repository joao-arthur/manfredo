use super::try_checked_inflate;
use crate::cartesian::rect::rect_i64::RectI64;

#[test]
fn try_checked_inflate_min_bounds() {
    assert_eq!(
        try_checked_inflate(&RectI64::of(i64::MIN + 7, i64::MIN + 3, i64::MIN + 9, i64::MIN + 13)),
        Some(RectI64::of(i64::MIN + 6, i64::MIN + 2, i64::MIN + 10, i64::MIN + 14))
    );
    assert_eq!(
        try_checked_inflate(&RectI64::of(i64::MIN + 6, i64::MIN + 2, i64::MIN + 10, i64::MIN + 14)),
        Some(RectI64::of(i64::MIN + 5, i64::MIN + 1, i64::MIN + 11, i64::MIN + 15))
    );
    assert_eq!(
        try_checked_inflate(&RectI64::of(i64::MIN + 5, i64::MIN + 1, i64::MIN + 11, i64::MIN + 15)),
        Some(RectI64::of(i64::MIN + 4, i64::MIN, i64::MIN + 12, i64::MIN + 16))
    );
}

#[test]
fn try_checked_inflate_max_bounds() {
    assert_eq!(
        try_checked_inflate(&RectI64::of(i64::MAX - 33, i64::MAX - 17, i64::MAX - 5, i64::MAX - 3)),
        Some(RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2))
    );
    assert_eq!(
        try_checked_inflate(&RectI64::of(i64::MAX - 34, i64::MAX - 18, i64::MAX - 4, i64::MAX - 2)),
        Some(RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1))
    );
    assert_eq!(
        try_checked_inflate(&RectI64::of(i64::MAX - 35, i64::MAX - 19, i64::MAX - 3, i64::MAX - 1)),
        Some(RectI64::of(i64::MAX - 36, i64::MAX - 20, i64::MAX - 2, i64::MAX))
    );
}

#[test]
fn try_checked_inflate_to_bounds() {
    assert_eq!(try_checked_inflate(&RectI64::of(i64::MIN + 1, i64::MIN + 1, i64::MAX - 1, i64::MAX - 1)), Some(RectI64::largest()));
    assert_eq!(
        try_checked_inflate(&RectI64::of(i64::MIN + 1, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10)),
        Some(RectI64::of(i64::MIN, i64::MIN + 9, i64::MAX - 9, i64::MAX - 9))
    );
    assert_eq!(
        try_checked_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 1, i64::MAX - 10, i64::MAX - 10)),
        Some(RectI64::of(i64::MIN + 9, i64::MIN, i64::MAX - 9, i64::MAX - 9))
    );
    assert_eq!(
        try_checked_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 1, i64::MAX - 10)),
        Some(RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MAX, i64::MAX - 9))
    );
    assert_eq!(
        try_checked_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX - 1)),
        Some(RectI64::of(i64::MIN + 9, i64::MIN + 9, i64::MAX - 9, i64::MAX))
    );
}

#[test]
fn try_checked_inflate_out_of_bounds() {
    assert_eq!(try_checked_inflate(&RectI64::largest()), None);
    assert_eq!(try_checked_inflate(&RectI64::of(i64::MIN, i64::MIN + 10, i64::MAX - 10, i64::MAX - 10)), None);
    assert_eq!(try_checked_inflate(&RectI64::of(i64::MIN + 10, i64::MIN, i64::MAX - 10, i64::MAX - 10)), None);
    assert_eq!(try_checked_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX, i64::MAX - 10)), None);
    assert_eq!(try_checked_inflate(&RectI64::of(i64::MIN + 10, i64::MIN + 10, i64::MAX - 10, i64::MAX)), None);
}
