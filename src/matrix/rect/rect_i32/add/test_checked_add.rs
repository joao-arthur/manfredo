use super::checked_add;
use crate::matrix::rect::rect_i32::RectI32;

#[test]
fn test() {
    assert_eq!(checked_add(&RectI32::of(-7, 9, -12, 15), &RectI32::of(5, 4, 3, 2)), RectI32::of(-2, 13, -9, 17));
    assert_eq!(checked_add(&RectI32::of(-2, 13, -9, 17), &RectI32::of(9, -10, 11, -12)), RectI32::of(7, 3, 2, 5));
}
