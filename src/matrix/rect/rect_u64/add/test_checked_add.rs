use super::checked_add;
use crate::matrix::rect::{rect_i64::Rect, rect_u64::RectU64};

#[test]
fn test() {
    assert_eq!(checked_add(&RectU64::of(0, 0, 12, 15), &Rect::of(5, 4, 3, 2)), RectU64::of(5, 4, 15, 17));
    assert_eq!(checked_add(&RectU64::of(5, 4, 15, 20), &Rect::of(-4, -3, -2, -1)), RectU64::of(1, 1, 13, 19));
}
