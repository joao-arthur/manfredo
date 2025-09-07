use super::{checked_add, checked_add_assign, try_checked_add, try_checked_add_assign};
use crate::cartesian::point::point_f64::{MAX, MIN, PointF64};

#[test]
fn test() {
    assert_eq!(checked_add(&PointF64::of(0.0, 0.0), &PointF64::of(10.0, 13.0)), PointF64::of(10.0, 13.0));
    assert_eq!(checked_add(&PointF64::of(10.0, 13.0), &PointF64::of(-5.0, -3.0)), PointF64::of(5.0, 10.0));
}
