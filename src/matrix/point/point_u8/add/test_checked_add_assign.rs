use super::checked_add_assign;
use crate::matrix::point::{point_i8::PointI8 as PointI, point_u8::PointU8};

#[test]
fn test() {
    let mut p = PointU8::min();
    checked_add_assign(&mut p, &PointI::of(10, 13));
    assert_eq!(p, PointU8::of(10, 13));
    checked_add_assign(&mut p, &PointI::of(-5, -3));
    assert_eq!(p, PointU8::of(5, 10));
}
