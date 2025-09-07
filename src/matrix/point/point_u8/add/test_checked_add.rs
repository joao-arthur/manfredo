use super::{checked_add, checked_add_assign, try_checked_add, try_checked_add_assign};
use crate::matrix::point::{point_i8::PointI8, point_u8::PointU8};

const MAX: u8 = u8::MAX;

#[test]
fn test_try_checked_add_assign() {
    let mut p = PointU8::min();
    assert_eq!(try_checked_add_assign(&mut p, &PointI8::of(10, 13)), Some(()));
    assert_eq!(p, PointU8::of(10, 13));
    assert_eq!(try_checked_add_assign(&mut p, &PointI8::of(-5, -3)), Some(()));
    assert_eq!(p, PointU8::of(5, 10));
}

#[test]
fn try_checked_add_assign_to_bounds() {
    let mut p_min = PointU8::of(2, 5);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI8::of(-2, -5)), Some(()));
    assert_eq!(p_min, PointU8::min());

    let mut p_max = PointU8::of(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI8::of(2, 5)), Some(()));
    assert_eq!(p_max, PointU8::max());
}

#[test]
fn try_checked_add_assign_out_of_bounds() {
    let mut p_min = PointU8::of(2, 5);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI8::of(-10, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI8::of(0, -10)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI8::of(-10, -10)), None);
    assert_eq!(p_min, PointU8::of(2, 5));

    let mut p_max = PointU8::of(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI8::of(10, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI8::of(0, 10)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI8::of(10, 10)), None);
    assert_eq!(p_max, PointU8::of(MAX - 2, MAX - 5));
}

#[test]
fn try_checked_add_assign_limits_out_of_bounds() {
    let mut p_min = PointU8::of(1, 1);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI8::of(i8::MIN, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI8::of(0, i8::MIN)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI8::min()), None);
    assert_eq!(p_min, PointU8::of(1, 1));

    let mut p_max = PointU8::of(MAX - 1, MAX - 1);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI8::of(i8::MAX, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI8::of(0, i8::MAX)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI8::max()), None);
    assert_eq!(p_max, PointU8::of(MAX - 1, MAX - 1));
}

#[test]
fn test_try_checked_add() {
    assert_eq!(try_checked_add(&PointU8::min(), &PointI8::of(10, 13)), Some(PointU8::of(10, 13)));
    assert_eq!(try_checked_add(&PointU8::of(10, 10), &PointI8::of(-5, -3)), Some(PointU8::of(5, 7)));
}

#[test]
fn try_checked_add_to_bounds() {
    assert_eq!(try_checked_add(&PointU8::of(2, 5), &PointI8::of(-2, -5)), Some(PointU8::min()));
    assert_eq!(try_checked_add(&PointU8::of(MAX - 2, MAX - 5), &PointI8::of(2, 5)), Some(PointU8::max()));
}

#[test]
fn try_checked_add_out_of_bounds() {
    let p_min = PointU8::of(2, 5);
    assert_eq!(try_checked_add(&p_min, &PointI8::of(-10, 0)), None);
    assert_eq!(try_checked_add(&p_min, &PointI8::of(0, -10)), None);
    assert_eq!(try_checked_add(&p_min, &PointI8::of(-10, -10)), None);

    let m_max = PointU8::of(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add(&m_max, &PointI8::of(10, 0)), None);
    assert_eq!(try_checked_add(&m_max, &PointI8::of(0, 10)), None);
    assert_eq!(try_checked_add(&m_max, &PointI8::of(10, 10)), None);
}

#[test]
fn try_checked_add_limits_out_of_bounds() {
    let p_min = PointU8::of(1, 1);
    assert_eq!(try_checked_add(&p_min, &PointI8::of(i8::MIN, 0)), None);
    assert_eq!(try_checked_add(&p_min, &PointI8::of(0, i8::MIN)), None);
    assert_eq!(try_checked_add(&p_min, &PointI8::min()), None);

    let p_max = PointU8::of(MAX - 1, MAX - 1);
    assert_eq!(try_checked_add(&p_max, &PointI8::of(i8::MAX, 0)), None);
    assert_eq!(try_checked_add(&p_max, &PointI8::of(0, i8::MAX)), None);
    assert_eq!(try_checked_add(&p_max, &PointI8::max()), None);
}

#[test]
fn test_checked_add_assign() {
    let mut p = PointU8::min();
    checked_add_assign(&mut p, &PointI8::of(10, 13));
    assert_eq!(p, PointU8::of(10, 13));
    checked_add_assign(&mut p, &PointI8::of(-5, -3));
    assert_eq!(p, PointU8::of(5, 10));
}

#[test]
fn test_checked_add() {
    assert_eq!(checked_add(&PointU8::min(), &PointI8::of(10, 13)), PointU8::of(10, 13));
    assert_eq!(checked_add(&PointU8::of(10, 13), &PointI8::of(-5, -3)), PointU8::of(5, 10));
}
