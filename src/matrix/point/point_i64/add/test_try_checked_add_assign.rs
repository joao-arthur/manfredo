use super::try_checked_add_assign;
use crate::matrix::point::point_i64::PointI64;

const MIN: i64 = i64::MIN;
const MAX: i64 = i64::MAX;

#[test]
fn test() {
    let mut p = PointI64::of(0, 0);
    assert_eq!(try_checked_add_assign(&mut p, &PointI64::of(10, 13)), Some(()));
    assert_eq!(p, PointI64::of(10, 13));
    assert_eq!(try_checked_add_assign(&mut p, &PointI64::of(-25, -30)), Some(()));
    assert_eq!(p, PointI64::of(-15, -17));
}

#[test]
fn to_bounds() {
    let mut p_min = PointI64::of(MIN + 2, MIN + 5);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI64::of(-2, -5)), Some(()));
    assert_eq!(p_min, PointI64::min());

    let mut p_max = PointI64::of(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI64::of(2, 5)), Some(()));
    assert_eq!(p_max, PointI64::max());
}

#[test]
fn out_of_bounds() {
    let mut p_min = PointI64::of(MIN + 2, MIN + 5);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI64::of(-10, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI64::of(0, -10)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI64::of(-10, -10)), None);
    assert_eq!(p_min, PointI64::of(MIN + 2, MIN + 5));

    let mut p_max = PointI64::of(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI64::of(10, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI64::of(0, 10)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI64::of(10, 10)), None);
    assert_eq!(p_max, PointI64::of(MAX - 2, MAX - 5));
}

#[test]
fn limits_out_of_bounds() {
    let mut p_min = PointI64::of(MIN + 1, MIN + 1);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI64::of(MIN, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI64::of(0, MIN)), None);
    assert_eq!(try_checked_add_assign(&mut p_min, &PointI64::min()), None);
    assert_eq!(p_min, PointI64::of(MIN + 1, MIN + 1));

    let mut p_max = PointI64::of(MAX - 1, MAX - 1);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI64::of(MAX, 0)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI64::of(0, MAX)), None);
    assert_eq!(try_checked_add_assign(&mut p_max, &PointI64::max()), None);
    assert_eq!(p_max, PointI64::of(MAX - 1, MAX - 1));
}
