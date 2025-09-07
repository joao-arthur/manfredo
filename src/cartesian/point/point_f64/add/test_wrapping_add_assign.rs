use super::{wrapping_add, wrapping_add_assign};
use crate::cartesian::point::point_f64::{MAX, MIN, PointF64};

#[test]
fn test_wrapping_add_assign() {
    let mut p = PointF64::of(0.0, 0.0);
    wrapping_add_assign(&mut p, &PointF64::of(10.0, 13.0));
    assert_eq!(p, PointF64::of(10.0, 13.0));
    wrapping_add_assign(&mut p, &PointF64::of(-5.0, -3.0));
    assert_eq!(p, PointF64::of(5.0, 10.0));
}

#[test]
fn wrapping_add_assign_to_bounds() {
    let mut p_min = PointF64::of(MIN + 2.0, MIN + 5.0);
    wrapping_add_assign(&mut p_min, &PointF64::of(-2.0, -5.0));
    assert_eq!(p_min, PointF64::min());

    let mut m_max = PointF64::of(MAX - 2.0, MAX - 5.0);
    wrapping_add_assign(&mut m_max, &PointF64::of(2.0, 5.0));
    assert_eq!(m_max, PointF64::max());
}

#[test]
fn wrapping_add_assign_out_of_bounds() {
    let mut p_min = PointF64::of(MIN + 2.0, MIN + 5.0);
    wrapping_add_assign(&mut p_min, &PointF64::of(-10.0, -10.0));
    assert_eq!(p_min, PointF64::of(MAX - 7.0, MAX - 4.0));

    let mut m_max = PointF64::of(MAX - 2.0, MAX - 5.0);
    wrapping_add_assign(&mut m_max, &PointF64::of(10.0, 10.0));
    assert_eq!(m_max, PointF64::of(MIN + 7.0, MIN + 4.0));
}

#[test]
fn wrapping_add_assign_limits_out_of_bounds() {
    let mut p_min = PointF64::of(MIN + 1.0, MIN + 1.0);
    wrapping_add_assign(&mut p_min, &PointF64::min());
    assert_eq!(p_min, PointF64::of(1.0, 1.0));

    let mut m_max = PointF64::of(MAX - 1.0, MAX - 1.0);
    wrapping_add_assign(&mut m_max, &PointF64::max());
    assert_eq!(m_max, PointF64::of(-3.0, -3.0));
}

#[test]
fn test_wrapping_add() {
    assert_eq!(wrapping_add(&PointF64::of(0.0, 0.0), &PointF64::of(10.0, 13.0)), PointF64::of(10.0, 13.0));
    assert_eq!(wrapping_add(&PointF64::of(10.0, 10.0), &PointF64::of(-5.0, -3.0)), PointF64::of(5.0, 7.0));
}

#[test]
fn wrapping_add_to_bounds() {
    assert_eq!(wrapping_add(&PointF64::of(MIN + 2.0, MIN + 5.0), &PointF64::of(-2.0, -5.0)), PointF64::min());
    assert_eq!(wrapping_add(&PointF64::of(MAX - 2.0, MAX - 5.0), &PointF64::of(2.0, 5.0)), PointF64::max());
}

#[test]
fn wrapping_add_out_of_bounds() {
    assert_eq!(wrapping_add(&PointF64::of(MIN + 2.0, MIN + 5.0), &PointF64::of(-10.0, -10.0)), PointF64::of(MAX - 7.0, MAX - 4.0));
    assert_eq!(wrapping_add(&PointF64::of(MAX - 2.0, MAX - 5.0), &PointF64::of(10.0, 10.0)), PointF64::of(MIN + 7.0, MIN + 4.0));
}

#[test]
fn wrapping_add_limits_out_of_bounds() {
    assert_eq!(wrapping_add(&PointF64::of(MIN + 1.0, MIN + 1.0), &PointF64::min()), PointF64::of(1.0, 1.0));
    assert_eq!(wrapping_add(&PointF64::of(MAX - 1.0, MAX - 1.0), &PointF64::max()), PointF64::of(-3.0, -3.0));
}
