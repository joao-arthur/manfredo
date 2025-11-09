use super::try_checked_add;
use crate::matrix::d1::point::point_i16::{MAX, MIN, Point};

#[test]
fn test() {
    assert_eq!(try_checked_add(&Point::zero(), &Point::new(10)), Some(Point::new(10)));
    assert_eq!(try_checked_add(&Point::new(10), &Point::new(-5)), Some(Point::new(5)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&Point::new(MIN + 2), &Point::new(-2)), Some(Point::min()));
    assert_eq!(try_checked_add(&Point::new(MAX - 2), &Point::new(2)), Some(Point::max()));
}

#[test]
fn out_of_bounds() {
    let p_min = Point::new(MIN + 2);
    assert_eq!(try_checked_add(&p_min, &Point::new(-10)), None);

    let p_max = Point::new(MAX - 2);
    assert_eq!(try_checked_add(&p_max, &Point::new(10)), None);
}

#[test]
fn limits_out_of_bounds() {
    let p_min = Point::new(MIN + 1);
    assert_eq!(try_checked_add(&p_min, &Point::min()), None);

    let p_max = Point::new(MAX - 1);
    assert_eq!(try_checked_add(&p_max, &Point::max()), None);
}
