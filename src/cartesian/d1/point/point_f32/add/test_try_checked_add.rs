use super::try_checked_add;
use crate::cartesian::d1::point::point_f32::{MAX, MIN, Point};

#[test]
fn test() {
    assert_eq!(try_checked_add(&Point::of(0.0), &Point::of(10.0)), Some(Point::of(10.0)));
    assert_eq!(try_checked_add(&Point::of(10.0), &Point::of(-5.0)), Some(Point::of(5.0)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&Point::of(MIN + 2.0), &Point::of(-2.0)), Some(Point::min()));
    assert_eq!(try_checked_add(&Point::of(MAX - 2.0), &Point::of(2.0)), Some(Point::max()));
}

#[test]
fn out_of_bounds() {
    let p_min = Point::of(MIN + 2.0);
    assert_eq!(try_checked_add(&p_min, &Point::of(-10.0)), None);

    let m_max = Point::of(MAX - 2.0);
    assert_eq!(try_checked_add(&m_max, &Point::of(10.0)), None);
}

#[test]
fn limits_out_of_bounds() {
    let p_min = Point::of(MIN + 1.0);
    assert_eq!(try_checked_add(&p_min, &Point::min()), None);

    let p_max = Point::of(MAX - 1.0);
    assert_eq!(try_checked_add(&p_max, &Point::max()), None);
}
