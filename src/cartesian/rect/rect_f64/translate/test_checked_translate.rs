use super::checked_translate;
use crate::cartesian::{point::point_f64::PointF64, rect::rect_f64::RectF64};

#[test]
fn test() {
    assert_eq!(checked_translate(&RectF64::of(0.0, 0.0, 10.0, 10.0), &PointF64::of(10.0, 20.0)), RectF64::of(10.0, 20.0, 20.0, 30.0));
    assert_eq!(checked_translate(&RectF64::of(10.0, 20.0, 20.0, 30.0), &PointF64::of(-20.0, -15.0)), RectF64::of(-10.0, 5.0, 0.0, 15.0));
}
