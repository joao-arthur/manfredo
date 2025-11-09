use super::try_checked_add;
use crate::cartesian::{
    d1::point::point_i16::{MAX, MIN},
    d2::point::point_i16::Point,
};

#[test]
fn test() {
    assert_eq!(try_checked_add(&Point::zero(), &Point::new(10, 13)), Some(Point::new(10, 13)));
    assert_eq!(try_checked_add(&Point::new(10, 10), &Point::new(-5, -3)), Some(Point::new(5, 7)));
}

#[test]
fn to_bounds() {
    assert_eq!(try_checked_add(&Point::new(MIN + 2, MIN + 5), &Point::new(-2, -5)), Some(Point::min()));
    assert_eq!(try_checked_add(&Point::new(MAX - 2, MAX - 5), &Point::new(2, 5)), Some(Point::max()));
}

#[test]
fn out_of_bounds() {
    let p_min = Point::new(MIN + 2, MIN + 5);
    assert_eq!(try_checked_add(&p_min, &Point::new(-10, 0)), None);
    assert_eq!(try_checked_add(&p_min, &Point::new(0, -10)), None);
    assert_eq!(try_checked_add(&p_min, &Point::new(-10, -10)), None);

    let p_max = Point::new(MAX - 2, MAX - 5);
    assert_eq!(try_checked_add(&p_max, &Point::new(10, 0)), None);
    assert_eq!(try_checked_add(&p_max, &Point::new(0, 10)), None);
    assert_eq!(try_checked_add(&p_max, &Point::new(10, 10)), None);
}

#[test]
fn limits_out_of_bounds() {
    let p_min = Point::new(MIN + 1, MIN + 1);
    assert_eq!(try_checked_add(&p_min, &Point::new(MIN, 0)), None);
    assert_eq!(try_checked_add(&p_min, &Point::new(0, MIN)), None);
    assert_eq!(try_checked_add(&p_min, &Point::min()), None);

    let p_max = Point::new(MAX - 1, MAX - 1);
    assert_eq!(try_checked_add(&p_max, &Point::new(MAX, 0)), None);
    assert_eq!(try_checked_add(&p_max, &Point::new(0, MAX)), None);
    assert_eq!(try_checked_add(&p_max, &Point::max()), None);
}
