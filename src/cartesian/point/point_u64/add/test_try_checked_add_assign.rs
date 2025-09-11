use super::try_checked_add_assign;
use crate::cartesian::point::{point_i64::PointI64 as PointI, point_u64::PointU64};

const MAX: u64 = u64::MAX;

#[test]
fn test() {
    let mut p = PointU64::min();
    assert_eq!(try_checked_add_assign(&mut p, &PointI::of(10, 13)), Some(()));
    assert_eq!(p, PointU64::of(10, 13));
    assert_eq!(try_checked_add_assign(&mut p, &PointI::of(-5, -3)), Some(()));
    assert_eq!(p, PointU64::of(5, 10));
}

#[test]
fn to_bounds() {
    let mut p_min = PointU64::of(2, 5);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::of(-2, -5)), Some(()));
    assert_eq!(p_min, PointU64::min());

    let mut p_max = PointU64::of(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::of(2, 5)), Some(()));
    assert_eq!(p_max, PointU64::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = PointU64::of(2, 5);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::of(-10, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::of(0, -10)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::of(-10, -10)), None);
    assert_eq!(p_min, PointU64::of(2, 5));

    let mut p_max = PointU64::of(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::of(10, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::of(0, 10)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::of(10, 10)), None);
    assert_eq!(p_max, PointU64::of(MAX - 2, MAX - 5));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = PointU64::of(1, 1);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::of(i64::MIN, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::of(0, i64::MIN)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI::min()), None);
    assert_eq!(p_min, PointU64::of(1, 1));

    let mut p_max = PointU64::of(MAX - 1, MAX - 1);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::of(i64::MAX, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::of(0, i64::MAX)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI::max()), None);
    assert_eq!(p_max, PointU64::of(MAX - 1, MAX - 1));
}
