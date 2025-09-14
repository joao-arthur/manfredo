use super::wrapping_add_assign;
use crate::cartesian::d2::point::{point_i64::Point as PointI, point_u64::Point};

const MAX: u64 = u64::MAX;

#[test]
fn test() {
    let mut p = Point::min();
    wrapping_add_assign(&mut p, &PointI::of(10, 13));
    assert_eq!(p, Point::of(10, 13));
    wrapping_add_assign(&mut p, &PointI::of(-5, -3));
    assert_eq!(p, Point::of(5, 10));
}

#[test]
fn to_bounds() {
    let mut p_min = Point::of(2, 5);
    wrapping_add_assign(&mut p_min, &PointI::of(-2, -5));
    assert_eq!(p_min, Point::min());

    let mut m_max = Point::of(MAX - 2, MAX - 5);
    wrapping_add_assign(&mut m_max, &PointI::of(2, 5));
    assert_eq!(m_max, Point::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = Point::of(2, 5);
    wrapping_add_assign(&mut p_min, &PointI::of(-10, -10));
    assert_eq!(p_min, Point::of(MAX - 7, MAX - 4));

    let mut m_max = Point::of(MAX - 2, MAX - 5);
    wrapping_add_assign(&mut m_max, &PointI::of(10, 10));
    assert_eq!(m_max, Point::of(7, 4));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = Point::of(1, 1);
    wrapping_add_assign(&mut p_min, &PointI::min());
    assert_eq!(p_min, Point::of(9223372036854775809, 9223372036854775809));

    let mut m_max = Point::of(MAX - 1, MAX - 1);
    wrapping_add_assign(&mut m_max, &PointI::max());
    assert_eq!(m_max, Point::of(9223372036854775805, 9223372036854775805));
}
