use super::wrapping_add_assign;
use crate::cartesian::point::{point_i64::PointI64 as PointI, point_u64::PointU64};

const MAX: u64 = u64::MAX;

#[test]
fn test() {
    let mut p = PointU64::min();
    wrapping_add_assign(&mut p, &PointI::of(10, 13));
    assert_eq!(p, PointU64::of(10, 13));
    wrapping_add_assign(&mut p, &PointI::of(-5, -3));
    assert_eq!(p, PointU64::of(5, 10));
}

#[test]
fn to_bounds() {
    let mut p_min = PointU64::of(2, 5);
    wrapping_add_assign(&mut p_min, &PointI::of(-2, -5));
    assert_eq!(p_min, PointU64::min());

    let mut m_max = PointU64::of(MAX - 2, MAX - 5);
    wrapping_add_assign(&mut m_max, &PointI::of(2, 5));
    assert_eq!(m_max, PointU64::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = PointU64::of(2, 5);
    wrapping_add_assign(&mut p_min, &PointI::of(-10, -10));
    assert_eq!(p_min, PointU64::of(MAX - 7, MAX - 4));

    let mut m_max = PointU64::of(MAX - 2, MAX - 5);
    wrapping_add_assign(&mut m_max, &PointI::of(10, 10));
    assert_eq!(m_max, PointU64::of(7, 4));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = PointU64::of(1, 1);
    wrapping_add_assign(&mut p_min, &PointI::min());
    assert_eq!(p_min, PointU64::of(9223372036854775809, 9223372036854775809));

    let mut m_max = PointU64::of(MAX - 1, MAX - 1);
    wrapping_add_assign(&mut m_max, &PointI::max());
    assert_eq!(m_max, PointU64::of(9223372036854775805, 9223372036854775805));
}
