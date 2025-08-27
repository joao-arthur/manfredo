use super::try_saturating_inflate;
use crate::cartesian::rect::rect_u64::RectU64;

#[test]
fn try_saturating_inflate_min_bounds() {
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
fn try_saturating_inflate_max_bounds() {
    assert_eq!(
        try_saturating_inflate(&RectU64::of(u64::MAX - 33, u64::MAX - 17, u64::MAX - 5, u64::MAX - 3)),
        Some(RectU64::of(u64::MAX - 34, u64::MAX - 18, u64::MAX - 4, u64::MAX - 2))
    );
    assert_eq!(
        try_saturating_inflate(&RectU64::of(u64::MAX - 34, u64::MAX - 18, u64::MAX - 4, u64::MAX - 2)),
        Some(RectU64::of(u64::MAX - 35, u64::MAX - 19, u64::MAX - 3, u64::MAX - 1))
    );
    assert_eq!(
        try_saturating_inflate(&RectU64::of(u64::MAX - 35, u64::MAX - 19, u64::MAX - 3, u64::MAX - 1)),
        Some(RectU64::of(u64::MAX - 36, u64::MAX - 20, u64::MAX - 2, u64::MAX))
    );
    assert_eq!(
        try_saturating_inflate(&RectU64::of(u64::MAX - 36, u64::MAX - 20, u64::MAX - 2, u64::MAX)),
        Some(RectU64::of(u64::MAX - 37, u64::MAX - 22, u64::MAX - 1, u64::MAX))
    );
    assert_eq!(
        try_saturating_inflate(&RectU64::of(u64::MAX - 37, u64::MAX - 22, u64::MAX - 1, u64::MAX)),
        Some(RectU64::of(u64::MAX - 38, u64::MAX - 24, u64::MAX, u64::MAX))
    );
    assert_eq!(
        try_saturating_inflate(&RectU64::of(u64::MAX - 38, u64::MAX - 24, u64::MAX, u64::MAX)),
        Some(RectU64::of(u64::MAX - 40, u64::MAX - 26, u64::MAX, u64::MAX))
    );
    assert_eq!(
        try_saturating_inflate(&RectU64::of(u64::MAX - 40, u64::MAX - 26, u64::MAX, u64::MAX)),
        Some(RectU64::of(u64::MAX - 42, u64::MAX - 28, u64::MAX, u64::MAX))
    );
    assert_eq!(
        try_saturating_inflate(&RectU64::of(u64::MAX - 42, u64::MAX - 28, u64::MAX, u64::MAX)),
        Some(RectU64::of(u64::MAX - 44, u64::MAX - 30, u64::MAX, u64::MAX))
    );
}

#[test]
fn try_saturating_inflate_to_bounds() {
    assert_eq!(try_saturating_inflate(&RectU64::of(1, 1, u64::MAX - 1, u64::MAX - 1)), Some(RectU64::largest()));
    assert_eq!(try_saturating_inflate(&RectU64::of(0, 0, u64::MAX - 1, u64::MAX - 1)), Some(RectU64::largest()));
    assert_eq!(try_saturating_inflate(&RectU64::of(1, 1, u64::MAX, u64::MAX)), Some(RectU64::largest()));
    assert_eq!(try_saturating_inflate(&RectU64::of(1, 10, u64::MAX - 10, u64::MAX - 10)), Some(RectU64::of(0, 9, u64::MAX - 9, u64::MAX - 9)));
    assert_eq!(try_saturating_inflate(&RectU64::of(10, 1, u64::MAX - 10, u64::MAX - 10)), Some(RectU64::of(9, 0, u64::MAX - 9, u64::MAX - 9)));
    assert_eq!(try_saturating_inflate(&RectU64::of(10, 10, u64::MAX - 1, u64::MAX - 10)), Some(RectU64::of(9, 9, u64::MAX, u64::MAX - 9)));
    assert_eq!(try_saturating_inflate(&RectU64::of(10, 10, u64::MAX - 10, u64::MAX - 1)), Some(RectU64::of(9, 9, u64::MAX - 9, u64::MAX)));
}

#[test]
fn try_saturating_inflate_out_of_bounds() {
    assert_eq!(try_saturating_inflate(&RectU64::largest()), None);
    assert_eq!(try_saturating_inflate(&RectU64::of(0, 10, u64::MAX, u64::MAX - 10)), None);
    assert_eq!(try_saturating_inflate(&RectU64::of(10, 0, u64::MAX - 10, u64::MAX)), None);
}
