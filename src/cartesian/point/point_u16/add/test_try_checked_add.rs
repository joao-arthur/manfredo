use super::try_checked_add;
use crate::cartesian::point::{point_i16::PointI16 as PointI, point_u16::Point};

const MAX: u16 = u16::MAX;

#[test]
fn test() {
    assert_eq!(try_checked_add(&Point::min(), &PointI::of(10, 13)), Some(Point::of(10, 13)));
    assert_eq!(try_checked_add(&Point::of(10, 10), &PointI::of(-5, -3)), Some(Point::of(5, 7)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&Point::of(2, 5), &PointI::of(-2, -5)), Some(Point::min()));
    assert_eq!(try_checked_add(&Point::of(MAX - 2, MAX - 5), &PointI::of(2, 5)), Some(Point::max()));
}

#[test]
fn out_of_bounds() {
    let p_min = Point::of(2, 5);
    assert_eq!(try_checked_add(&p_min, &PointI::of(-10, 0)), None);
    assert_eq!(try_checked_add(&p_min, &PointI::of(0, -10)), None);
    assert_eq!(try_checked_add(&p_min, &PointI::of(-10, -10)), None);

    let m_max = Point::of(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add(&m_max, &PointI::of(10, 0)), None);
    assert_eq!(try_checked_add(&m_max, &PointI::of(0, 10)), None);
    assert_eq!(try_checked_add(&m_max, &PointI::of(10, 10)), None);
}

#[test]
fn limits_out_of_bounds() {
    let p_min = Point::of(1, 1);
    assert_eq!(try_checked_add(&p_min, &PointI::of(i16::MIN, 0)), None);
    assert_eq!(try_checked_add(&p_min, &PointI::of(0, i16::MIN)), None);
    assert_eq!(try_checked_add(&p_min, &PointI::min()), None);

    let p_max = Point::of(MAX - 1, MAX - 1);
    assert_eq!(try_checked_add(&p_max, &PointI::of(i16::MAX, 0)), None);
    assert_eq!(try_checked_add(&p_max, &PointI::of(0, i16::MAX)), None);
    assert_eq!(try_checked_add(&p_max, &PointI::max()), None);
}
