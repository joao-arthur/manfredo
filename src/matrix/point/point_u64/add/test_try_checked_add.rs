use super::try_checked_add;
use crate::matrix::point::{point_i64::PointI64, point_u64::PointU64};

const MAX: u64 = u64::MAX;

#[test]
fn test() {
    assert_eq!(try_checked_add(&PointU64::min(), &PointI64::of(10, 13)), Some(PointU64::of(10, 13)));
    assert_eq!(try_checked_add(&PointU64::of(10, 10), &PointI64::of(-5, -3)), Some(PointU64::of(5, 7)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&PointU64::of(2, 5), &PointI64::of(-2, -5)), Some(PointU64::min()));
    assert_eq!(try_checked_add(&PointU64::of(MAX - 2, MAX - 5), &PointI64::of(2, 5)), Some(PointU64::max()));
}

#[test]
fn out_of_bounds() {
    let p_min = PointU64::of(2, 5);
    assert_eq!(try_checked_add(&p_min, &PointI64::of(-10, 0)), None);
    assert_eq!(try_checked_add(&p_min, &PointI64::of(0, -10)), None);
    assert_eq!(try_checked_add(&p_min, &PointI64::of(-10, -10)), None);

    let m_max = PointU64::of(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add(&m_max, &PointI64::of(10, 0)), None);
    assert_eq!(try_checked_add(&m_max, &PointI64::of(0, 10)), None);
    assert_eq!(try_checked_add(&m_max, &PointI64::of(10, 10)), None);
}

#[test]
fn limits_out_of_bounds() {
    let p_min = PointU64::of(1, 1);
    assert_eq!(try_checked_add(&p_min, &PointI64::of(i64::MIN, 0)), None);
    assert_eq!(try_checked_add(&p_min, &PointI64::of(0, i64::MIN)), None);
    assert_eq!(try_checked_add(&p_min, &PointI64::min()), None);

    let p_max = PointU64::of(MAX - 1, MAX - 1);
    assert_eq!(try_checked_add(&p_max, &PointI64::of(i64::MAX, 0)), None);
    assert_eq!(try_checked_add(&p_max, &PointI64::of(0, i64::MAX)), None);
    assert_eq!(try_checked_add(&p_max, &PointI64::max()), None);
}
