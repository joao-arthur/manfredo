use super::checked_add;
use crate::cartesian::rect::{rect_i64::RectI64, rect_u64::RectU64};

#[test]
fn test() {
    assert_eq!(checked_add(&RectU64::of(0, 0, 12, 10), &RectI64::of(5, 4, 3, 2)), RectU64::of(5, 4, 15, 12));
    assert_eq!(checked_add(&RectU64::of(5, 4, 15, 12), &RectI64::of(-4, -3, -2, -1)), RectU64::of(1, 1, 13, 11));
}
