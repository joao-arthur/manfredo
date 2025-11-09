use super::try_checked_add;
use crate::cartesian::{
    d1::point::point_u64::MAX,
    d2::point::{point_i64::Point as PointI, point_u64::Point},
};

#[test]
fn test() {
    assert_eq!(try_checked_add(&Point::min(), &PointI::new(10, 13)), Some(Point::new(10, 13)));
    assert_eq!(try_checked_add(&Point::new(10, 10), &PointI::new(-5, -3)), Some(Point::new(5, 7)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&Point::new(2, 5), &PointI::new(-2, -5)), Some(Point::min()));
    assert_eq!(try_checked_add(&Point::new(MAX - 2, MAX - 5), &PointI::new(2, 5)), Some(Point::max()));
}

#[test]
fn out_of_bounds() {
    let p_min = Point::new(2, 5);
    assert_eq!(try_checked_add(&p_min, &PointI::new(-10, 0)), None);
    assert_eq!(try_checked_add(&p_min, &PointI::new(0, -10)), None);
    assert_eq!(try_checked_add(&p_min, &PointI::new(-10, -10)), None);

    let p_max = Point::new(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add(&p_max, &PointI::new(10, 0)), None);
    assert_eq!(try_checked_add(&p_max, &PointI::new(0, 10)), None);
    assert_eq!(try_checked_add(&p_max, &PointI::new(10, 10)), None);
}

#[test]
fn limits_out_of_bounds() {
    let p_min = Point::new(1, 1);
    assert_eq!(try_checked_add(&p_min, &PointI::new(i64::MIN, 0)), None);
    assert_eq!(try_checked_add(&p_min, &PointI::new(0, i64::MIN)), None);
    assert_eq!(try_checked_add(&p_min, &PointI::min()), None);

    let p_max = Point::new(MAX - 1, MAX - 1);
    assert_eq!(try_checked_add(&p_max, &PointI::new(i64::MAX, 0)), None);
    assert_eq!(try_checked_add(&p_max, &PointI::new(0, i64::MAX)), None);
    assert_eq!(try_checked_add(&p_max, &PointI::max()), None);
}
