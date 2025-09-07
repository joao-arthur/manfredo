use super::wrapping_add_assign;
use crate::matrix::point::{point_i16::PointI16, point_u16::PointU16};

const MAX: u16 = u16::MAX;

#[test]
fn test() {
    let mut p = PointU16::min();
    wrapping_add_assign(&mut p, &PointI16::of(10, 13));
    assert_eq!(p, PointU16::of(10, 13));
    wrapping_add_assign(&mut p, &PointI16::of(-5, -3));
    assert_eq!(p, PointU16::of(5, 10));
}

#[test]
fn to_bounds() {
    let mut p_min = PointU16::of(2, 5);
    wrapping_add_assign(&mut p_min, &PointI16::of(-2, -5));
    assert_eq!(p_min, PointU16::min());

    let mut m_max = PointU16::of(MAX - 2, MAX - 5);
    wrapping_add_assign(&mut m_max, &PointI16::of(2, 5));
    assert_eq!(m_max, PointU16::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = PointU16::of(2, 5);
    wrapping_add_assign(&mut p_min, &PointI16::of(-10, -10));
    assert_eq!(p_min, PointU16::of(MAX - 7, MAX - 4));

    let mut m_max = PointU16::of(MAX - 2, MAX - 5);
    wrapping_add_assign(&mut m_max, &PointI16::of(10, 10));
    assert_eq!(m_max, PointU16::of(7, 4));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = PointU16::of(1, 1);
    wrapping_add_assign(&mut p_min, &PointI16::min());
    assert_eq!(p_min, PointU16::of(32769, 32769));

    let mut m_max = PointU16::of(MAX - 1, MAX - 1);
    wrapping_add_assign(&mut m_max, &PointI16::max());
    assert_eq!(m_max, PointU16::of(32765, 32765));
}
