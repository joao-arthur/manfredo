use super::checked_add;
use crate::cartesian::rect::{rect_i64::RectI64, rect_u64::RectU64};

#[test]
fn test_checked_add() {
    assert_eq!(checked_add(&RectU64::of(0, 0, 12, 15), &RectI64::of(5, 4, 3, 2)), RectU64::of(5, 4, 15, 17));
    assert_eq!(checked_add(&RectU64::of(5, 4, 15, 20), &RectI64::of(-4, -3, -2, -1)), RectU64::of(1, 1, 13, 19));
}
