use super::saturating_add_assign;
use crate::matrix::point::{point_i16::PointI16 as PointI, point_u16::Point};

const MAX: u16 = u16::MAX;

#[test]
fn test() {
    let mut p = Point::min();
    saturating_add_assign(&mut p, &PointI::of(10, 13));
    assert_eq!(p, Point::of(10, 13));
    saturating_add_assign(&mut p, &PointI::of(-5, -3));
    assert_eq!(p, Point::of(5, 10));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::of(2, 5);
    saturating_add_assign(&mut p_min, &PointI::of(-2, -5));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::of(MAX - 2, MAX - 5);
    saturating_add_assign(&mut p_max, &PointI::of(2, 5));
    assert_eq!(p_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::of(2, 5);
    saturating_add_assign(&mut p_min, &PointI::of(-10, -10));
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::of(MAX - 2, MAX - 5);
    saturating_add_assign(&mut p_max, &PointI::of(10, 10));
    assert_eq!(p_max, Point::max());
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::of(1, 1);
    saturating_add_assign(&mut p_min, &PointI::min());
    assert_eq!(p_min, Point::min());

    let mut p_max = Point::of(MAX - 1, MAX - 1);
    saturating_add_assign(&mut p_max, &PointI::max());
    assert_eq!(p_max, Point::max());
}
