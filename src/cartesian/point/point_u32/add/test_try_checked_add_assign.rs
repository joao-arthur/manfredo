use super::{checked_add, checked_add_assign, try_checked_add, try_checked_add_assign};
use crate::cartesian::point::{point_i32::PointI32, point_u32::PointU32};

const MAX: u32 = u32::MAX;

#[test]
fn test() {
    let mut p = PointU32::min();
    assert_eq!(try_checked_add_assign(&mut p, &PointI32::of(10, 13)), Some(()));
    assert_eq!(p, PointU32::of(10, 13));
    assert_eq!(try_checked_add_assign(&mut p, &PointI32::of(-5, -3)), Some(()));
    assert_eq!(p, PointU32::of(5, 10));
}

#[test]
fn to_bounds() {
    let mut p_min = PointU32::of(2, 5);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI32::of(-2, -5)), Some(()));
    assert_eq!(p_min, PointU32::min());

    let mut p_max = PointU32::of(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI32::of(2, 5)), Some(()));
    assert_eq!(p_max, PointU32::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = PointU32::of(2, 5);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI32::of(-10, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI32::of(0, -10)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI32::of(-10, -10)), None);
    assert_eq!(p_min, PointU32::of(2, 5));

    let mut p_max = PointU32::of(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI32::of(10, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI32::of(0, 10)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI32::of(10, 10)), None);
    assert_eq!(p_max, PointU32::of(MAX - 2, MAX - 5));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = PointU32::of(1, 1);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI32::of(i32::MIN, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI32::of(0, i32::MIN)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI32::min()), None);
    assert_eq!(p_min, PointU32::of(1, 1));

    let mut p_max = PointU32::of(MAX - 1, MAX - 1);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI32::of(i32::MAX, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI32::of(0, i32::MAX)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI32::max()), None);
    assert_eq!(p_max, PointU32::of(MAX - 1, MAX - 1));
}
