use super::try_checked_add;
use crate::cartesian::d1::point::{point_i64::Point as PointI, point_u64::Point};

const MAX: u64 = u64::MAX;

#[test]
fn test() {
    assert_eq!(try_checked_add(&Point::min(), &PointI::of(10)), Some(Point::of(10)));
    assert_eq!(try_checked_add(&Point::of(10), &PointI::of(-5)), Some(Point::of(5)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&Point::of(2), &PointI::of(-2)), Some(Point::min()));
    assert_eq!(try_checked_add(&Point::of(MAX - 2), &PointI::of(2)), Some(Point::max()));
}

#[test]
fn out_of_bounds() {
    let p_min = Point::of(2);
    assert_eq!(try_checked_add(&p_min, &PointI::of(-10)), None);

    let p_max = Point::of(MAX - 2);
    assert_eq!(try_checked_add(&p_max, &PointI::of(10)), None);
}

#[test]
fn limits_out_of_bounds() {
    let p_min = Point::of(1);
    assert_eq!(try_checked_add(&p_min, &PointI::min()), None);

    let p_max = Point::of(MAX - 1);
    assert_eq!(try_checked_add(&p_max, &PointI::max()), None);
}
