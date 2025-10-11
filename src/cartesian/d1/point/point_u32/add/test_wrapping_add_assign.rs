use super::wrapping_add_assign;
use crate::cartesian::d1::point::{point_i32::Point as PointI, point_u32::Point};

const MAX: u32 = u32::MAX;

#[test]
fn test() {
    let mut p = Point::min();
    wrapping_add_assign(&mut p, &PointI::of(10));
    assert_eq!(p, Point::of(10));
    wrapping_add_assign(&mut p, &PointI::of(-5));
    assert_eq!(p, Point::of(5));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::of(2);
    wrapping_add_assign(&mut p_min, &PointI::of(-2));
    assert_eq!(p_min, Point::min());

    let mut m_max = Point::of(MAX - 2);
    wrapping_add_assign(&mut m_max, &PointI::of(2));
    assert_eq!(m_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::of(2);
    wrapping_add_assign(&mut p_min, &PointI::of(-10));
    assert_eq!(p_min, Point::of(MAX - 7));

    let mut m_max = Point::of(MAX - 2);
    wrapping_add_assign(&mut m_max, &PointI::of(10));
    assert_eq!(m_max, Point::of(7));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::of(1);
    wrapping_add_assign(&mut p_min, &PointI::min());
    assert_eq!(p_min, Point::of(2147483649));

    let mut m_max = Point::of(MAX - 1);
    wrapping_add_assign(&mut m_max, &PointI::max());
    assert_eq!(m_max, Point::of(2147483645));
}
