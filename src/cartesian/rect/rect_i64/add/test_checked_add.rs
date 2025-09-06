use super::checked_add;
use crate::cartesian::rect::rect_i64::RectI64;

#[test]
fn test() {
    assert_eq!(checked_add(&RectI64::of(-7, 9, -12, 15), &RectI64::of(5, 4, 3, 2)), RectI64::of(-2, 13, -9, 17));
    assert_eq!(checked_add(&RectI64::of(-2, 13, -9, 17), &RectI64::of(9, -10, 11, -12)), RectI64::of(7, 3, 2, 5));
}
