use super::checked_add;
use crate::cartesian::rect::rect_f64::RectF64;

#[test]
fn test() {
    assert_eq!(checked_add(&RectF64::of(-7.0, 9.0, -12.0, 15.0), &RectF64::of(5.0, 4.0, 3.0, 2.0)), RectF64::of(-2.0, 13.0, -9.0, 17.0));
    assert_eq!(checked_add(&RectF64::of(-2.0, 13.0, -9.0, 17.0), &RectF64::of(9.0, -10.0, 11.0, -12.0)), RectF64::of(7.0, 3.0, 2.0, 5.0));
}
