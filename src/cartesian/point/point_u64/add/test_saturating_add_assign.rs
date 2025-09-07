use super::saturating_add_assign;
use crate::cartesian::point::{point_i64::PointI64, point_u64::PointU64};

const MAX: u64 = u64::MAX;

#[test]
fn test() {
    let mut p = PointU64::min();
    saturating_add_assign(&mut p, &PointI64::of(10, 13));
    assert_eq!(p, PointU64::of(10, 13));
    saturating_add_assign(&mut p, &PointI64::of(-5, -3));
    assert_eq!(p, PointU64::of(5, 10));
}

#[test]
fn to_bounds() {
    let mut p_min = PointU64::of(2, 5);
    saturating_add_assign(&mut p_min, &PointI64::of(-2, -5));
    assert_eq!(p_min, PointU64::min());

    let mut p_max = PointU64::of(MAX - 2, MAX - 5);
    saturating_add_assign(&mut p_max, &PointI64::of(2, 5));
    assert_eq!(p_max, PointU64::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = PointU64::of(2, 5);
    saturating_add_assign(&mut p_min, &PointI64::of(-10, -10));
    assert_eq!(p_min, PointU64::min());

    let mut p_max = PointU64::of(MAX - 2, MAX - 5);
    saturating_add_assign(&mut p_max, &PointI64::of(10, 10));
    assert_eq!(p_max, PointU64::max());
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = PointU64::of(1, 1);
    saturating_add_assign(&mut p_min, &PointI64::min());
    assert_eq!(p_min, PointU64::min());

    let mut p_max = PointU64::of(MAX - 1, MAX - 1);
    saturating_add_assign(&mut p_max, &PointI64::max());
    assert_eq!(p_max, PointU64::max());
}
