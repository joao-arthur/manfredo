use super::try_checked_add;
use crate::cartesian::d1::point::point_i8::{MAX, MIN, Point};

#[test]
fn test() {
    assert_eq!(try_checked_add(&Point::of(0), &Point::of(10)), Some(Point::of(10)));
    assert_eq!(try_checked_add(&Point::of(10), &Point::of(-5)), Some(Point::of(5)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&Point::of(MIN + 2), &Point::of(-2)), Some(Point::min()));
    assert_eq!(try_checked_add(&Point::of(MAX - 2), &Point::of(2)), Some(Point::max()));
}

#[test]
fn out_of_bounds() {
    let p_min = Point::of(MIN + 2);
    assert_eq!(try_checked_add(&p_min, &Point::of(-10)), None);

    let p_max = Point::of(MAX - 2);
    assert_eq!(try_checked_add(&p_max, &Point::of(10)), None);
}

#[test]
fn limits_out_of_bounds() {
    let p_min = Point::of(MIN + 1);
    assert_eq!(try_checked_add(&p_min, &Point::min()), None);

    let p_max = Point::of(MAX - 1);
    assert_eq!(try_checked_add(&p_max, &Point::max()), None);
}
