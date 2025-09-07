use super::{checked_add, checked_add_assign, try_checked_add, try_checked_add_assign};
use crate::matrix::point::{point_i16::PointI16, point_u16::PointU16};

const MAX: u16 = u16::MAX;

#[test]
fn test() {
    let mut p = PointU16::min();
    checked_add_assign(&mut p, &PointI16::of(10, 13));
    assert_eq!(p, PointU16::of(10, 13));
    checked_add_assign(&mut p, &PointI16::of(-5, -3));
    assert_eq!(p, PointU16::of(5, 10));
}
