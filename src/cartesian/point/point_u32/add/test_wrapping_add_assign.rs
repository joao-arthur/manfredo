use super::{wrapping_add, wrapping_add_assign};
use crate::cartesian::point::{point_i32::PointI32, point_u32::PointU32};

const MAX: u32 = u32::MAX;

#[test]
fn test() {
    let mut p = PointU32::min();
    wrapping_add_assign(&mut p, &PointI32::of(10, 13));
    assert_eq!(p, PointU32::of(10, 13));
    wrapping_add_assign(&mut p, &PointI32::of(-5, -3));
    assert_eq!(p, PointU32::of(5, 10));
}

#[test]
fn to_bounds() {
    let mut p_min = PointU32::of(2, 5);
    wrapping_add_assign(&mut p_min, &PointI32::of(-2, -5));
    assert_eq!(p_min, PointU32::min());

    let mut m_max = PointU32::of(MAX - 2, MAX - 5);
    wrapping_add_assign(&mut m_max, &PointI32::of(2, 5));
    assert_eq!(m_max, PointU32::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = PointU32::of(2, 5);
    wrapping_add_assign(&mut p_min, &PointI32::of(-10, -10));
    assert_eq!(p_min, PointU32::of(MAX - 7, MAX - 4));

    let mut m_max = PointU32::of(MAX - 2, MAX - 5);
    wrapping_add_assign(&mut m_max, &PointI32::of(10, 10));
    assert_eq!(m_max, PointU32::of(7, 4));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = PointU32::of(1, 1);
    wrapping_add_assign(&mut p_min, &PointI32::min());
    assert_eq!(p_min, PointU32::of(2147483649, 2147483649));

    let mut m_max = PointU32::of(MAX - 1, MAX - 1);
    wrapping_add_assign(&mut m_max, &PointI32::max());
    assert_eq!(m_max, PointU32::of(2147483645, 2147483645));
}
