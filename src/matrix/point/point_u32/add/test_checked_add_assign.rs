use super::checked_add_assign;
use crate::matrix::point::{point_i32::PointI32, point_u32::PointU32};

#[test]
fn test() {
    let mut p = PointU32::min();
    checked_add_assign(&mut p, &PointI32::of(10, 13));
    assert_eq!(p, PointU32::of(10, 13));
    checked_add_assign(&mut p, &PointI32::of(-5, -3));
    assert_eq!(p, PointU32::of(5, 10));
}
