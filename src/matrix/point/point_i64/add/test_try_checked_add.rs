use super::try_checked_add;
use crate::matrix::point::point_i64::PointI64;

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

#[test]
fn test() {
    assert_eq!(try_checked_add(&PointI64::of(0, 0), &PointI64::of(10, 13)), Some(PointI64::of(10, 13)));
    assert_eq!(try_checked_add(&PointI64::of(10, 10), &PointI64::of(-5, -3)), Some(PointI64::of(5, 7)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&PointI64::of(MIN + 2, MIN + 5), &PointI64::of(-2, -5)), Some(PointI64::min()));
    assert_eq!(try_checked_add(&PointI64::of(MAX - 2, MAX - 5), &PointI64::of(2, 5)), Some(PointI64::max()));
}

#[test]
fn out_of_bounds() {
    let p_min = PointI64::of(MIN + 2, MIN + 5);
    assert_eq!(try_checked_add(&p_min, &PointI64::of(-10, 0)), None);
    assert_eq!(try_checked_add(&p_min, &PointI64::of(0, -10)), None);
    assert_eq!(try_checked_add(&p_min, &PointI64::of(-10, -10)), None);

    let m_max = PointI64::of(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add(&m_max, &PointI64::of(10, 0)), None);
    assert_eq!(try_checked_add(&m_max, &PointI64::of(0, 10)), None);
    assert_eq!(try_checked_add(&m_max, &PointI64::of(10, 10)), None);
}

#[test]
fn limits_out_of_bounds() {
    let p_min = PointI64::of(MIN + 1, MIN + 1);
    assert_eq!(try_checked_add(&p_min, &PointI64::of(MIN, 0)), None);
    assert_eq!(try_checked_add(&p_min, &PointI64::of(0, MIN)), None);
    assert_eq!(try_checked_add(&p_min, &PointI64::min()), None);

    let p_max = PointI64::of(MAX - 1, MAX - 1);
    assert_eq!(try_checked_add(&p_max, &PointI64::of(MAX, 0)), None);
    assert_eq!(try_checked_add(&p_max, &PointI64::of(0, MAX)), None);
    assert_eq!(try_checked_add(&p_max, &PointI64::max()), None);
}
