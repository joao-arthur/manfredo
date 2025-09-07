use super::{checked_add, checked_add_assign, try_checked_add, try_checked_add_assign};
use crate::cartesian::point::point_f64::{MAX, MIN, PointF64};

#[test]
fn test() {
    let mut p = PointF64::of(0.0, 0.0);
    checked_add_assign(&mut p, &PointF64::of(10.0, 13.0));
    assert_eq!(p, PointF64::of(10.0, 13.0));
    checked_add_assign(&mut p, &PointF64::of(-25.0, -30.0));
    assert_eq!(p, PointF64::of(-15.0, -17.0));
}
