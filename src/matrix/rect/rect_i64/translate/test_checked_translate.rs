use super::checked_translate;
use crate::matrix::{point::point_i64::PointI64, rect::rect_i64::RectI64};

#[test]
fn test() {
    assert_eq!(checked_translate(&RectI64::of(5, 9, 13, 37), &PointI64::of(-10, -20)), RectI64::of(-5, -11, 3, 17));
    assert_eq!(checked_translate(&RectI64::of(-5, -11, 3, 17), &PointI64::of(6, -19)), RectI64::of(1, -30, 9, -2));
}
