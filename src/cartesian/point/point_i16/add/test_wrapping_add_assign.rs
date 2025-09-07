use super::wrapping_add_assign;
use crate::cartesian::point::point_i16::PointI16;

const MIN: i16 = i16::MIN;
const MAX: i16 = i16::MAX;

#[test]
fn test() {
    let mut p = PointI16::of(0, 0);
    wrapping_add_assign(&mut p, &PointI16::of(10, 13));
    assert_eq!(p, PointI16::of(10, 13));
    wrapping_add_assign(&mut p, &PointI16::of(-5, -3));
    assert_eq!(p, PointI16::of(5, 10));
}

#[test]
fn to_bounds() {
    let mut p_min = PointI16::of(MIN + 2, MIN + 5);
    wrapping_add_assign(&mut p_min, &PointI16::of(-2, -5));
    assert_eq!(p_min, PointI16::min());

    let mut m_max = PointI16::of(MAX - 2, MAX - 5);
    wrapping_add_assign(&mut m_max, &PointI16::of(2, 5));
    assert_eq!(m_max, PointI16::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = PointI16::of(MIN + 2, MIN + 5);
    wrapping_add_assign(&mut p_min, &PointI16::of(-10, -10));
    assert_eq!(p_min, PointI16::of(MAX - 7, MAX - 4));

    let mut m_max = PointI16::of(MAX - 2, MAX - 5);
    wrapping_add_assign(&mut m_max, &PointI16::of(10, 10));
    assert_eq!(m_max, PointI16::of(MIN + 7, MIN + 4));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = PointI16::of(MIN + 1, MIN + 1);
    wrapping_add_assign(&mut p_min, &PointI16::min());
    assert_eq!(p_min, PointI16::of(1, 1));

    let mut m_max = PointI16::of(MAX - 1, MAX - 1);
    wrapping_add_assign(&mut m_max, &PointI16::max());
    assert_eq!(m_max, PointI16::of(-3, -3));
}
