use super::checked_add;
use crate::matrix::rect::rect_i16::RectI16;

#[test]
fn test() {
    assert_eq!(checked_add(&RectI16::of(-7, 9, -12, 15), &RectI16::of(5, 4, 3, 2)), RectI16::of(-2, 13, -9, 17));
    assert_eq!(checked_add(&RectI16::of(-2, 13, -9, 17), &RectI16::of(9, -10, 11, -12)), RectI16::of(7, 3, 2, 5));
}
