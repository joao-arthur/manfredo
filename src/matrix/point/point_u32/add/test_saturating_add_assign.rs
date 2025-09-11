use super::saturating_add_assign;
use crate::matrix::point::{point_i32::PointI32 as PointI, point_u32::PointU32};

const MAX: u32 = u32::MAX;

#[test]
fn test() {
    let mut p = PointU32::min();
    saturating_add_assign(&mut p, &PointI::of(10, 13));
    assert_eq!(p, PointU32::of(10, 13));
    saturating_add_assign(&mut p, &PointI::of(-5, -3));
    assert_eq!(p, PointU32::of(5, 10));
}

#[test]
fn to_bounds() {
    let mut p_min = PointU32::of(2, 5);
    saturating_add_assign(&mut p_min, &PointI::of(-2, -5));
    assert_eq!(p_min, PointU32::min());

    let mut p_max = PointU32::of(MAX - 2, MAX - 5);
    saturating_add_assign(&mut p_max, &PointI::of(2, 5));
    assert_eq!(p_max, PointU32::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = PointU32::of(2, 5);
    saturating_add_assign(&mut p_min, &PointI::of(-10, -10));
    assert_eq!(p_min, PointU32::min());

    let mut p_max = PointU32::of(MAX - 2, MAX - 5);
    saturating_add_assign(&mut p_max, &PointI::of(10, 10));
    assert_eq!(p_max, PointU32::max());
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = PointU32::of(1, 1);
    saturating_add_assign(&mut p_min, &PointI::min());
    assert_eq!(p_min, PointU32::min());

    let mut p_max = PointU32::of(MAX - 1, MAX - 1);
    saturating_add_assign(&mut p_max, &PointI::max());
    assert_eq!(p_max, PointU32::max());
}
