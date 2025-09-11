use super::checked_translate;
use crate::matrix::{point::point_i64::PointI64, rect::rect_u64::Rect};

#[test]
fn test() {
    assert_eq!(checked_translate(&Rect::of(0, 0, 12, 15), &PointI64::of(5, 4)), Rect::of(5, 4, 17, 19));
    assert_eq!(checked_translate(&Rect::of(5, 4, 17, 19), &PointI64::of(-4, -2)), Rect::of(1, 2, 13, 17));
}
