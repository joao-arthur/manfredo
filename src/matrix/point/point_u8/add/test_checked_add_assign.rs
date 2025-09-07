use super::{checked_add, checked_add_assign, try_checked_add, try_checked_add_assign};
use crate::matrix::point::{point_i8::PointI8, point_u8::PointU8};

const MAX: u8 = u8::MAX;

#[test]
fn test() {
    let mut p = PointU8::min();
    checked_add_assign(&mut p, &PointI8::of(10, 13));
    assert_eq!(p, PointU8::of(10, 13));
    checked_add_assign(&mut p, &PointI8::of(-5, -3));
    assert_eq!(p, PointU8::of(5, 10));
}
