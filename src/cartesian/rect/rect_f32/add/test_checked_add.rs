use super::checked_add;
use crate::cartesian::rect::rect_f32::RectF32;

#[test]
fn test() {
    assert_eq!(checked_add(&RectF32::of(-7.0, 9.0, -12.0, 15.0), &RectF32::of(5.0, 4.0, 3.0, 2.0)), RectF32::of(-2.0, 13.0, -9.0, 17.0));
    assert_eq!(checked_add(&RectF32::of(5.0, 4.0, 15.0, 20.0), &RectF32::of(-14.0, -13.0, -12.0, -11.0)), RectF32::of(-9.0, -9.0, 3.0, 9.0));
}
