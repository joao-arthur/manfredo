use super::{checked_add, checked_add_assign, try_checked_add, try_checked_add_assign};
use crate::cartesian::point::{point_i16::PointI16, point_u16::PointU16};

const MAX: u16 = u16::MAX;

#[test]
fn test() {
    assert_eq!(try_checked_add(&PointU16::min(), &PointI16::of(10, 13)), Some(PointU16::of(10, 13)));
    assert_eq!(try_checked_add(&PointU16::of(10, 10), &PointI16::of(-5, -3)), Some(PointU16::of(5, 7)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&PointU16::of(2, 5), &PointI16::of(-2, -5)), Some(PointU16::min()));
    assert_eq!(try_checked_add(&PointU16::of(MAX - 2, MAX - 5), &PointI16::of(2, 5)), Some(PointU16::max()));
}

#[test]
fn out_of_bounds() {
    let p_min = PointU16::of(2, 5);
    assert_eq!(try_checked_add(&p_min, &PointI16::of(-10, 0)), None);
    assert_eq!(try_checked_add(&p_min, &PointI16::of(0, -10)), None);
    assert_eq!(try_checked_add(&p_min, &PointI16::of(-10, -10)), None);

    let m_max = PointU16::of(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add(&m_max, &PointI16::of(10, 0)), None);
    assert_eq!(try_checked_add(&m_max, &PointI16::of(0, 10)), None);
    assert_eq!(try_checked_add(&m_max, &PointI16::of(10, 10)), None);
}

#[test]
fn limits_out_of_bounds() {
    let p_min = PointU16::of(1, 1);
    assert_eq!(try_checked_add(&p_min, &PointI16::of(i16::MIN, 0)), None);
    assert_eq!(try_checked_add(&p_min, &PointI16::of(0, i16::MIN)), None);
    assert_eq!(try_checked_add(&p_min, &PointI16::min()), None);

    let p_max = PointU16::of(MAX - 1, MAX - 1);
    assert_eq!(try_checked_add(&p_max, &PointI16::of(i16::MAX, 0)), None);
    assert_eq!(try_checked_add(&p_max, &PointI16::of(0, i16::MAX)), None);
    assert_eq!(try_checked_add(&p_max, &PointI16::max()), None);
}
