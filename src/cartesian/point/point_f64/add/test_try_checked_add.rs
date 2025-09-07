use super::try_checked_add;
use crate::cartesian::point::point_f64::{MAX, MIN, PointF64};

#[test]
fn test() {
    assert_eq!(try_checked_add(&PointF64::of(0.0, 0.0), &PointF64::of(10.0, 13.0)), Some(PointF64::of(10.0, 13.0)));
    assert_eq!(try_checked_add(&PointF64::of(10.0, 10.0), &PointF64::of(-5.0, -3.0)), Some(PointF64::of(5.0, 7.0)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&PointF64::of(MIN + 2.0, MIN + 5.0), &PointF64::of(-2.0, -5.0)), Some(PointF64::min()));
    assert_eq!(try_checked_add(&PointF64::of(MAX - 2.0, MAX - 5.0), &PointF64::of(2.0, 5.0)), Some(PointF64::max()));
}

#[test]
fn out_of_bounds() {
    let p_min = PointF64::of(MIN + 2.0, MIN + 5.0);
    assert_eq!(try_checked_add(&p_min, &PointF64::of(-10.0, 0.0)), None);
    assert_eq!(try_checked_add(&p_min, &PointF64::of(0.0, -10.0)), None);
    assert_eq!(try_checked_add(&p_min, &PointF64::of(-10.0, -10.0)), None);

    let m_max = PointF64::of(MAX - 2.0, MAX - 5.0);
    assert_eq!(try_checked_add(&m_max, &PointF64::of(10.0, 0.0)), None);
    assert_eq!(try_checked_add(&m_max, &PointF64::of(0.0, 10.0)), None);
    assert_eq!(try_checked_add(&m_max, &PointF64::of(10.0, 10.0)), None);
}

#[test]
fn limits_out_of_bounds() {
    let p_min = PointF64::of(MIN + 1.0, MIN + 1.0);
    assert_eq!(try_checked_add(&p_min, &PointF64::of(MIN, 0.0)), None);
    assert_eq!(try_checked_add(&p_min, &PointF64::of(0.0, MIN)), None);
    assert_eq!(try_checked_add(&p_min, &PointF64::min()), None);

    let p_max = PointF64::of(MAX - 1.0, MAX - 1.0);
    assert_eq!(try_checked_add(&p_max, &PointF64::of(MAX, 0.0)), None);
    assert_eq!(try_checked_add(&p_max, &PointF64::of(0.0, MAX)), None);
    assert_eq!(try_checked_add(&p_max, &PointF64::max()), None);
}
