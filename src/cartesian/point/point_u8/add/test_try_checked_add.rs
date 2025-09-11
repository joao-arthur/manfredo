use super::try_checked_add;
use crate::cartesian::point::{point_i8::PointI8 as PointI, point_u8::PointU8};

const MAX: u8 = u8::MAX;

#[test]
fn test() {
    assert_eq!(try_checked_add(&PointU8::min(), &PointI::of(10, 13)), Some(PointU8::of(10, 13)));
    assert_eq!(try_checked_add(&PointU8::of(10, 10), &PointI::of(-5, -3)), Some(PointU8::of(5, 7)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&PointU8::of(2, 5), &PointI::of(-2, -5)), Some(PointU8::min()));
    assert_eq!(try_checked_add(&PointU8::of(MAX - 2, MAX - 5), &PointI::of(2, 5)), Some(PointU8::max()));
}

#[test]
fn out_of_bounds() {
    let p_min = PointU8::of(2, 5);
    assert_eq!(try_checked_add(&p_min, &PointI::of(-10, 0)), None);
    assert_eq!(try_checked_add(&p_min, &PointI::of(0, -10)), None);
    assert_eq!(try_checked_add(&p_min, &PointI::of(-10, -10)), None);

    let m_max = PointU8::of(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add(&m_max, &PointI::of(10, 0)), None);
    assert_eq!(try_checked_add(&m_max, &PointI::of(0, 10)), None);
    assert_eq!(try_checked_add(&m_max, &PointI::of(10, 10)), None);
}

#[test]
fn limits_out_of_bounds() {
    let p_min = PointU8::of(1, 1);
    assert_eq!(try_checked_add(&p_min, &PointI::of(i8::MIN, 0)), None);
    assert_eq!(try_checked_add(&p_min, &PointI::of(0, i8::MIN)), None);
    assert_eq!(try_checked_add(&p_min, &PointI::min()), None);

    let p_max = PointU8::of(MAX - 1, MAX - 1);
    assert_eq!(try_checked_add(&p_max, &PointI::of(i8::MAX, 0)), None);
    assert_eq!(try_checked_add(&p_max, &PointI::of(0, i8::MAX)), None);
    assert_eq!(try_checked_add(&p_max, &PointI::max()), None);
}
