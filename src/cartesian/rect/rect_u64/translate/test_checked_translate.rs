use super::checked_translate;
use crate::cartesian::{point::point_i64::PointI64, rect::rect_u64::RectU64};

#[test]
fn test() {
    assert_eq!(checked_translate(&RectU64::of(0, 0, 12, 15), &PointI64::of(5, 4)), RectU64::of(5, 4, 17, 19));
    assert_eq!(checked_translate(&RectU64::of(5, 4, 17, 19), &PointI64::of(-4, -2)), RectU64::of(1, 2, 13, 17));
}
