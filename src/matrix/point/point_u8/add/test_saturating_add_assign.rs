use super::saturating_add_assign;
use crate::matrix::point::{point_i8::PointI8 as PointI, point_u8::PointU8};

const MAX: u8 = u8::MAX;

#[test]
fn test() {
    let mut p = PointU8::min();
    saturating_add_assign(&mut p, &PointI::of(10, 13));
    assert_eq!(p, PointU8::of(10, 13));
    saturating_add_assign(&mut p, &PointI::of(-5, -3));
    assert_eq!(p, PointU8::of(5, 10));
}

#[test]
fn to_bounds() {
    let mut p_min = PointU8::of(2, 5);
    saturating_add_assign(&mut p_min, &PointI::of(-2, -5));
    assert_eq!(p_min, PointU8::min());

    let mut p_max = PointU8::of(MAX - 2, MAX - 5);
    saturating_add_assign(&mut p_max, &PointI::of(2, 5));
    assert_eq!(p_max, PointU8::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = PointU8::of(2, 5);
    saturating_add_assign(&mut p_min, &PointI::of(-10, -10));
    assert_eq!(p_min, PointU8::min());

    let mut p_max = PointU8::of(MAX - 2, MAX - 5);
    saturating_add_assign(&mut p_max, &PointI::of(10, 10));
    assert_eq!(p_max, PointU8::max());
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = PointU8::of(1, 1);
    saturating_add_assign(&mut p_min, &PointI::min());
    assert_eq!(p_min, PointU8::min());

    let mut p_max = PointU8::of(MAX - 1, MAX - 1);
    saturating_add_assign(&mut p_max, &PointI::max());
    assert_eq!(p_max, PointU8::max());
}
