use super::checked_translate;
use crate::cartesian::{point::point_i64::PointI64, rect::rect_i64::RectI64};

#[test]
fn test_checked_translate() {
    assert_eq!(checked_translate(&RectI64::of(0, 0, 10, 10), &PointI64::of(10, 20)), RectI64::of(10, 20, 20, 30));
    assert_eq!(checked_translate(&RectI64::of(10, 20, 20, 30), &PointI64::of(-20, -15)), RectI64::of(-10, 5, 0, 15));
}
