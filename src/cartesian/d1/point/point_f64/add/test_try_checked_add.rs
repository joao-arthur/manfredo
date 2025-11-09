use super::try_checked_add;
use crate::cartesian::d1::point::point_f64::{MAX, MIN, Point};

#[test]
fn test() {
    assert_eq!(try_checked_add(&Point::zero(), &Point::new(10.0)), Some(Point::new(10.0)));
    assert_eq!(try_checked_add(&Point::new(10.0), &Point::new(-5.0)), Some(Point::new(5.0)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&Point::new(MIN + 2.0), &Point::new(-2.0)), Some(Point::min()));
    assert_eq!(try_checked_add(&Point::new(MAX - 2.0), &Point::new(2.0)), Some(Point::max()));
}

#[test]
fn out_of_bounds() {
    let p_min = Point::new(MIN + 2.0);
    assert_eq!(try_checked_add(&p_min, &Point::new(-10.0)), None);

    let p_max = Point::new(MAX - 2.0);
    assert_eq!(try_checked_add(&p_max, &Point::new(10.0)), None);
}

#[test]
fn limits_out_of_bounds() {
    let p_min = Point::new(MIN + 1.0);
    assert_eq!(try_checked_add(&p_min, &Point::min()), None);

    let p_max = Point::new(MAX - 1.0);
    assert_eq!(try_checked_add(&p_max, &Point::max()), None);
}
